//! Book chapter 25. A curator pass marks skills stale, then archives them.
//! The request path only enqueues that pass. A write uses the chapter 23
//! gate. An install uses the chapter 23 scan.

use std::collections::VecDeque;
use std::fs::{self, File};
use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

use flate2::write::GzEncoder;
use flate2::Compression;
use skill_write::{scan_text, should_allow_install, Decision, Trust};

pub const STALE_AFTER_DAYS: i64 = 14;
pub const ARCHIVE_AFTER_DAYS: i64 = 30;
pub const INTERVAL_HOURS: i64 = 24 * 7;
pub const MIN_IDLE_HOURS: i64 = 2;
pub const CONSOLIDATE: bool = false;
pub const PRUNE_BUILTINS: bool = false;
pub const KEEP_SNAPSHOTS: usize = 2;
const QUEUE_CAP: usize = 64;

const EXCLUDE: &[&str] = &[
    ".curator_backups",
    ".hub",
    ".locks",
    ".git",
    ".archive",
    ".curator_ledger.jsonl",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    Agent,
    Bundled,
    Hub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Active,
    Stale,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub age_days: i64,
    pub pinned: bool,
    pub cron: bool,
    pub origin: Origin,
    pub state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Counts {
    pub checked: usize,
    pub marked_stale: usize,
    pub archived: usize,
    pub skipped: usize,
    pub consolidated: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CuratorError {
    Scan(String),
    Staged(String),
    Path(String),
    Io(String),
}

pub struct Curator {
    root: PathBuf,
    approval: Mutex<bool>,
    staged: Mutex<Vec<String>>,
    queue: Mutex<VecDeque<String>>,
    consolidate: bool,
    next_snapshot: Mutex<u32>,
}

impl Curator {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, CuratorError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(io)?;
        Ok(Self {
            root,
            approval: Mutex::new(false),
            staged: Mutex::new(Vec::new()),
            queue: Mutex::new(VecDeque::new()),
            consolidate: CONSOLIDATE,
            next_snapshot: Mutex::new(0),
        })
    }

    pub fn set_write_approval(&self, on: bool) {
        *self.approval.lock().expect("approval") = on;
    }

    pub fn set_consolidate(&mut self, on: bool) {
        self.consolidate = on;
    }

    pub fn consolidate(&self) -> bool {
        self.consolidate
    }

    pub fn staged(&self) -> Vec<String> {
        self.staged.lock().expect("staged").clone()
    }

    pub fn queue_len(&self) -> usize {
        self.queue.lock().expect("queue").len()
    }

    /// The request path. Records that a pass should run. Does not scan,
    /// snapshot, or move a directory.
    pub fn enqueue(&self) {
        let mut queue = self.queue.lock().expect("queue");
        if queue.len() == QUEUE_CAP {
            queue.pop_front();
        }
        queue.push_back("curate".to_string());
    }

    pub fn due(idle_hours: i64, since_last_hours: i64) -> bool {
        idle_hours >= MIN_IDLE_HOURS && since_last_hours >= INTERVAL_HOURS
    }

    pub fn pass(&self, skills: &mut [Skill]) -> Result<Counts, CuratorError> {
        self.snapshot()?;
        let mut counts = Counts::default();
        for skill in skills.iter_mut() {
            counts.checked += 1;
            if skill.pinned || skill.cron || !self.candidate(skill) {
                counts.skipped += 1;
                continue;
            }
            if skill.age_days >= ARCHIVE_AFTER_DAYS {
                let label = format!("archive {}", skill.name);
                if self.hold(&label) {
                    continue;
                }
                self.archive_dir(&skill.name)?;
                skill.state = State::Archived;
                counts.archived += 1;
            } else if skill.age_days >= STALE_AFTER_DAYS {
                skill.state = State::Stale;
                counts.marked_stale += 1;
            }
        }
        if self.consolidate {
            counts.consolidated = 0;
        }
        Ok(counts)
    }

    pub fn install(&self, raw: &str, trust: Trust, force: bool) -> Result<(), CuratorError> {
        let result = scan_text(raw);
        match should_allow_install(&result, trust, force) {
            Decision::Allow => {}
            Decision::Ask | Decision::Block => {
                return Err(CuratorError::Scan(format!("{:?}", result.verdict)));
            }
        }
        let name = front_name(raw).unwrap_or_else(|| "skill".into());
        let dir = self.root.join(&name);
        if !within_root(&self.root, &dir) {
            return Err(CuratorError::Path(name));
        }
        let label = format!("install {name}");
        if self.hold(&label) {
            return Err(CuratorError::Staged(label));
        }
        fs::create_dir_all(&dir).map_err(io)?;
        fs::write(dir.join("SKILL.md"), raw).map_err(io)?;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<PathBuf, CuratorError> {
        let backups = self.root.join(".curator_backups");
        fs::create_dir_all(&backups).map_err(io)?;
        let id = {
            let mut next = self.next_snapshot.lock().expect("snapshot");
            let id = *next;
            *next += 1;
            id
        };
        let path = backups.join(format!("{id:04}-skills.tar.gz"));
        let file = File::create(&path).map_err(io)?;
        let encoder = GzEncoder::new(file, Compression::fast());
        let mut archive = tar::Builder::new(encoder);
        add_tree(&self.root, &self.root, &mut archive)?;
        let encoder = archive.into_inner().map_err(io)?;
        encoder.finish().map_err(io)?;
        prune_snapshots(&backups)?;
        Ok(path)
    }

    fn candidate(&self, skill: &Skill) -> bool {
        match skill.origin {
            Origin::Hub => false,
            Origin::Bundled => PRUNE_BUILTINS,
            Origin::Agent => true,
        }
    }

    fn hold(&self, label: &str) -> bool {
        if !*self.approval.lock().expect("approval") {
            return false;
        }
        self.staged.lock().expect("staged").push(label.to_string());
        true
    }

    fn archive_dir(&self, name: &str) -> Result<(), CuratorError> {
        let from = self.root.join(name);
        let dest_dir = self.root.join(".archive");
        let dest = dest_dir.join(name);
        if !within_root(&self.root, &from) || !within_root(&dest_dir, &dest) {
            return Err(CuratorError::Path(name.to_string()));
        }
        if !from.exists() {
            return Ok(());
        }
        fs::create_dir_all(&dest_dir).map_err(io)?;
        fs::rename(&from, dest).map_err(io)?;
        Ok(())
    }
}

pub fn demo(root: &Path) -> Result<String, CuratorError> {
    let curator = Curator::open(root)?;
    let dir = root.join("notes");
    fs::create_dir_all(&dir).map_err(io)?;
    fs::write(dir.join("SKILL.md"), "---\nname: notes\n---\nKeep.\n").map_err(io)?;
    let mut skills = vec![Skill {
        name: "notes".into(),
        age_days: STALE_AFTER_DAYS,
        pinned: false,
        cron: false,
        origin: Origin::Agent,
        state: State::Active,
    }];
    let counts = curator.pass(&mut skills)?;
    Ok(format!("stale: {}", counts.marked_stale))
}

fn add_tree(
    root: &Path,
    dir: &Path,
    archive: &mut tar::Builder<GzEncoder<File>>,
) -> Result<(), CuratorError> {
    for entry in fs::read_dir(dir).map_err(io)? {
        let entry = entry.map_err(io)?;
        let path = entry.path();
        if excluded(&path) {
            continue;
        }
        let name = path.strip_prefix(root).unwrap_or(&path);
        if path.is_dir() {
            add_tree(root, &path, archive)?;
        } else {
            archive.append_path_with_name(&path, name).map_err(io)?;
        }
    }
    Ok(())
}

fn excluded(path: &Path) -> bool {
    path.components().any(|part| {
        part.as_os_str()
            .to_str()
            .is_some_and(|name| EXCLUDE.contains(&name))
    })
}

fn prune_snapshots(dir: &Path) -> Result<(), CuratorError> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).map_err(io)? {
        let entry = entry.map_err(io)?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("gz") {
            files.push(path);
        }
    }
    files.sort();
    let extra = files.len().saturating_sub(KEEP_SNAPSHOTS);
    for path in files.into_iter().take(extra) {
        fs::remove_file(path).map_err(io)?;
    }
    Ok(())
}

fn within_root(root: &Path, path: &Path) -> bool {
    let Ok(rel) = path.strip_prefix(root) else {
        return false;
    };
    let mut depth = 0i32;
    for part in rel.components() {
        match part {
            Component::Normal(_) => depth += 1,
            Component::CurDir => {}
            Component::ParentDir => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            Component::RootDir | Component::Prefix(_) => return false,
        }
    }
    true
}

fn front_name(raw: &str) -> Option<String> {
    for line in raw.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if key.trim() == "name" {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn io(err: impl ToString) -> CuratorError {
    CuratorError::Io(err.to_string())
}
