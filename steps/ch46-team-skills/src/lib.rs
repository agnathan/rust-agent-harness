//! Book chapter 46. Workers propose. The librarian is the only writer.
//! Readers hold an `Arc` of the index and do not wait on the scan.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub const PROPOSAL_CAP: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Bundled,
    Team,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trust {
    Builtin,
    Community,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub body: String,
    pub layer: Layer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    pub skills: Vec<Skill>,
    pub version: u64,
    pub team_md: String,
}

impl Index {
    pub fn bundled(skills: Vec<Skill>) -> Self {
        let team_md = render_team(&skills);
        Self {
            skills,
            version: 1,
            team_md,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    pub name: String,
    pub body: String,
    pub force: bool,
    pub trust: Trust,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Propose {
    Queued,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reject {
    Guard,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notice {
    pub kind: &'static str,
    pub version: u64,
}

pub struct Library {
    index: Mutex<Arc<Index>>,
    notices: Mutex<Vec<Notice>>,
}

impl Library {
    pub fn new(index: Index) -> Self {
        Self {
            index: Mutex::new(Arc::new(index)),
            notices: Mutex::new(Vec::new()),
        }
    }

    pub fn load(&self) -> Arc<Index> {
        Arc::clone(&self.index.lock().expect("index"))
    }

    pub fn notices(&self) -> Vec<Notice> {
        self.notices.lock().expect("notices").clone()
    }
}

pub fn render_team(skills: &[Skill]) -> String {
    let mut text = String::from("# TEAM\n");
    for skill in skills {
        let layer = match skill.layer {
            Layer::Bundled => "bundled",
            Layer::Team => "team",
        };
        text.push_str(&format!("- {} ({layer})\n", skill.name));
    }
    text
}

pub fn skill_propose(queue: &mut VecDeque<Proposal>, proposal: Proposal) -> Propose {
    if queue.len() >= PROPOSAL_CAP {
        return Propose::Full;
    }
    queue.push_back(proposal);
    Propose::Queued
}

/// Dangerous community text is refused, including when `force` is set.
pub fn allow_install(proposal: &Proposal) -> bool {
    let dangerous = is_dangerous(&proposal.body);
    if dangerous && proposal.trust == Trust::Community {
        return false;
    }
    !proposal.body.trim().is_empty()
}

pub fn is_dangerous(body: &str) -> bool {
    let lower = body.to_ascii_lowercase();
    lower.contains("rm -rf /") || lower.contains("ignore previous instructions")
}

/// Swap the index pointer. The caller builds `next` before taking the lock.
pub fn swap_index(library: &Library, next: Arc<Index>) -> Arc<Index> {
    let mut slot = library.index.lock().expect("index");
    std::mem::replace(&mut *slot, next)
}

pub struct FakeModel {
    pub reply: &'static str,
}

impl FakeModel {
    pub fn complete(&self, prompt: &str) -> String {
        let _ = prompt;
        self.reply.to_string()
    }
}

/// The only writer. The model runs before the pointer swap.
pub fn librarian_accept(
    library: &Library,
    proposal: &Proposal,
    model: &FakeModel,
) -> Result<u64, Reject> {
    if proposal.body.trim().is_empty() {
        return Err(Reject::Empty);
    }
    if !allow_install(proposal) {
        return Err(Reject::Guard);
    }
    let _note = model.complete(&proposal.name);
    let current = library.load();
    let mut skills = current.skills.clone();
    skills.push(Skill {
        name: proposal.name.clone(),
        body: proposal.body.clone(),
        layer: Layer::Team,
    });
    let version = current.version + 1;
    let next = Arc::new(Index {
        team_md: render_team(&skills),
        skills,
        version,
    });
    swap_index(library, next);
    library.notices.lock().expect("notices").push(Notice {
        kind: "skills.updated",
        version,
    });
    Ok(version)
}

pub fn demo() -> String {
    let library = Library::new(Index::bundled(vec![Skill {
        name: "echo".into(),
        body: "Say the line back.".into(),
        layer: Layer::Bundled,
    }]));
    let mut queue = VecDeque::new();
    skill_propose(
        &mut queue,
        Proposal {
            name: "cite".into(),
            body: "Name the file you read.".into(),
            force: false,
            trust: Trust::Community,
        },
    );
    let proposal = queue.pop_front().expect("proposal");
    let version =
        librarian_accept(&library, &proposal, &FakeModel { reply: "ok" }).expect("accept");
    let index = library.load();
    format!("skills: v{version} {}", index.team_md.lines().count())
}
