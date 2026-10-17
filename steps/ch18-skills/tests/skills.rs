use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use skills::{
    change_note, demo, load_tree, render_index, skill_view, skills_list, SkillSession,
    MAX_NAME_LENGTH,
};

struct Temp {
    path: PathBuf,
}

impl Temp {
    fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let path = std::env::temp_dir().join(format!("ch18-{}-{nanos}-{n}", std::process::id()));
        fs::create_dir_all(&path).expect("dir");
        Self { path }
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_skill(root: &std::path::Path, category: &str, name: &str, description: &str, extra: &str) {
    let dir = root.join(category).join(name);
    fs::create_dir_all(&dir).expect("skill dir");
    let text = format!("---\nname: {name}\ndescription: {description}\n{extra}---\nDo {name}.\n");
    fs::write(dir.join("SKILL.md"), text).expect("write");
}

#[test]
fn the_index_is_sorted_and_byte_stable() {
    let temp = Temp::new();
    write_skill(&temp.path, "files", "zeta", "Last", "");
    write_skill(
        &temp.path,
        "files",
        "alpha",
        "First",
        "requires_mcp: github\n",
    );
    write_skill(&temp.path, "mail", "send", "Send mail", "");
    let skills = load_tree(&temp.path).expect("load");
    let once = render_index(&skills, &[]);
    let twice = render_index(&skills, &[]);
    assert_eq!(once, twice);
    assert!(once.find("alpha").unwrap() < once.find("zeta").unwrap());
    assert!(once.contains("[needs: github]"));
    let connected = render_index(&skills, &["github".to_string()]);
    assert!(!connected.contains("[needs:"));
    let listed = skills_list(&skills, Some("files"));
    assert_eq!(listed.len(), 2);
    assert!(skills_list(&skills, Some("mail")).len() == 1);
}

#[test]
fn skill_view_returns_the_whole_file_and_refuses_a_path_escape() {
    let temp = Temp::new();
    write_skill(&temp.path, "files", "rename", "Rename a file", "");
    fs::create_dir_all(temp.path.join("files/rename/references")).expect("ref");
    fs::write(temp.path.join("files/rename/references/notes.md"), "a note").expect("notes");
    let whole = skill_view(&temp.path, "rename", None).expect("view");
    assert!(whole.contains("name: rename"));
    assert!(whole.contains("Do rename."));
    let note = skill_view(&temp.path, "rename", Some("references/notes.md")).expect("file");
    assert_eq!(note, "a note");
    assert!(skill_view(&temp.path, "rename", Some("../SKILL.md")).is_err());
    assert!(skill_view(&temp.path, "rename", Some("secret.txt")).is_err());
}

#[test]
fn a_new_skill_is_a_note_and_the_arc_stays() {
    let temp = Temp::new();
    write_skill(&temp.path, "files", "rename", "Rename a file", "");
    let session = SkillSession::open(&temp.path, &[]).expect("open");
    let kept: Arc<String> = session.index();
    write_skill(&temp.path, "files", "sync", "Sync a repo", "");
    let skills = load_tree(&temp.path).expect("reload");
    let note = change_note(kept.as_str(), &skills, &[]).expect("note");
    assert!(note.contains("sync"));
    assert!(note.starts_with("[System: The skills available"));
    assert_eq!(session.index().as_str(), kept.as_str());
    assert!(change_note(kept.as_str(), &load_tree(&temp.path).unwrap(), &[]).is_some());
}

#[test]
fn a_long_name_is_refused_and_the_demo_is_keyless() {
    let temp = Temp::new();
    let long = "n".repeat(MAX_NAME_LENGTH + 1);
    write_skill(&temp.path, "files", "bad", "ok", "");
    let dir = temp.path.join("files/bad");
    fs::write(
        dir.join("SKILL.md"),
        format!("---\nname: {long}\ndescription: ok\n---\nbody\n"),
    )
    .expect("overwrite");
    assert!(load_tree(&temp.path).is_err());

    let demo_root = temp.path.join("demo");
    let (name, line) = demo(&demo_root).expect("demo");
    assert_eq!(name, "rename");
    assert_eq!(line, "Rename a file by its path.");
}
