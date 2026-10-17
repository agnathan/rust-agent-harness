//! Book chapter 22. Skills are folders of SKILL.md. The index is rendered
//! once into an Arc. A later add becomes a note. It does not rewrite the Arc.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub const MAX_NAME_LENGTH: usize = 64;
pub const MAX_DESCRIPTION_LENGTH: usize = 1024;
pub const MAX_BODY_CHARS: usize = 100_000;

const SUPPORT: &[&str] = &["references", "templates", "scripts", "assets"];

const NOTE_PREFIX: &str =
    "[System: The skills available in this conversation changed after its system prompt was built.";
const ADDED_HEAD: &str = "Now available (load with skill_view(name) when relevant, like the skills listed in the system prompt):";
const REMOVED_HEAD: &str = "No longer available:";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    pub category: String,
    pub requires_mcp: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    Name(String),
    Description(String),
    Body(usize),
    Missing(String),
    Path(String),
    Io(String),
}

#[derive(Debug)]
pub struct ScriptedModel {
    reply: String,
    calls: u32,
}

impl ScriptedModel {
    pub fn new(reply: impl Into<String>) -> Self {
        Self {
            reply: reply.into(),
            calls: 0,
        }
    }

    pub fn calls(&self) -> u32 {
        self.calls
    }

    pub fn complete(&mut self, _prompt: &str) -> String {
        self.calls += 1;
        self.reply.clone()
    }
}

pub struct SkillSession {
    index: Arc<String>,
}

impl SkillSession {
    pub fn open(root: &Path, connected: &[String]) -> Result<Self, SkillError> {
        let skills = load_tree(root)?;
        Ok(Self {
            index: Arc::new(render_index(&skills, connected)),
        })
    }

    pub fn index(&self) -> Arc<String> {
        Arc::clone(&self.index)
    }
}

pub fn load_tree(root: &Path) -> Result<Vec<SkillMeta>, SkillError> {
    let mut found = Vec::new();
    let categories = list_dirs(root)?;
    for category in categories {
        let cat_name = file_name(&category)?;
        for skill_dir in list_dirs(&category)? {
            let path = skill_dir.join("SKILL.md");
            if !path.is_file() {
                continue;
            }
            let text = fs::read_to_string(&path).map_err(|err| SkillError::Io(err.to_string()))?;
            if text.chars().count() > MAX_BODY_CHARS {
                return Err(SkillError::Body(text.chars().count()));
            }
            let meta = parse_skill(&text, &cat_name, &file_name(&skill_dir)?)?;
            if found.iter().any(|got: &SkillMeta| got.name == meta.name) {
                continue;
            }
            found.push(meta);
        }
    }
    found.sort_by(|a, b| (&a.category, &a.name).cmp(&(&b.category, &b.name)));
    Ok(found)
}

pub fn skills_list(skills: &[SkillMeta], category: Option<&str>) -> Vec<SkillMeta> {
    skills
        .iter()
        .filter(|skill| category.is_none_or(|want| skill.category == want))
        .cloned()
        .collect()
}

pub fn skill_view(root: &Path, name: &str, file: Option<&str>) -> Result<String, SkillError> {
    let dir = find_dir(root, name)?;
    let path = match file {
        None => dir.join("SKILL.md"),
        Some(rel) => support_path(&dir, rel)?,
    };
    fs::read_to_string(&path).map_err(|err| SkillError::Io(err.to_string()))
}

pub fn render_index(skills: &[SkillMeta], connected: &[String]) -> String {
    let mut lines = vec![
        "## Skills".to_string(),
        "Before replying, scan the skills below. If a skill matches or is even partially relevant to your task, you MUST load it with skill_view(name) and follow its instructions. Err on the side of loading — it is always better to have context you don't need than to miss critical steps, pitfalls, or established workflows.".to_string(),
        "<available_skills>".to_string(),
    ];
    let mut category = String::new();
    for skill in skills {
        if skill.category != category {
            category = skill.category.clone();
            lines.push(format!("  {category}:"));
        }
        let mut row = if skill.description.is_empty() {
            format!("    - {}", skill.name)
        } else {
            format!("    - {}: {}", skill.name, skill.description)
        };
        if let Some(server) = &skill.requires_mcp {
            if !connected.iter().any(|got| got == server) {
                row.push_str(&format!(" [needs: {server}]"));
            }
        }
        lines.push(row);
    }
    lines.push("</available_skills>".to_string());
    lines.push(
        "Only proceed without loading a skill if genuinely none are relevant to the task."
            .to_string(),
    );
    lines.join("\n")
}

pub fn change_note(previous: &str, skills: &[SkillMeta], connected: &[String]) -> Option<String> {
    let old = index_names(previous);
    let rendered = render_index(skills, connected);
    let new = index_names(&rendered);
    let added: Vec<&str> = new
        .iter()
        .filter(|name| !old.iter().any(|got| got == *name))
        .map(String::as_str)
        .collect();
    let removed: Vec<&str> = old
        .iter()
        .filter(|name| !new.iter().any(|got| got == *name))
        .map(String::as_str)
        .collect();
    if added.is_empty() && removed.is_empty() {
        return None;
    }
    let mut parts = vec![NOTE_PREFIX.to_string()];
    if !added.is_empty() {
        parts.push(ADDED_HEAD.to_string());
        for name in added {
            parts.push(format!("    - {name}"));
        }
    }
    if !removed.is_empty() {
        parts.push(format!("{REMOVED_HEAD} {}", removed.join(", ")));
    }
    Some(parts.join("\n") + "]")
}

pub fn demo(root: &Path) -> Result<(String, String), SkillError> {
    fs::create_dir_all(root.join("files/rename")).map_err(|err| SkillError::Io(err.to_string()))?;
    let raw = "---\nname: rename\ndescription: Rename a file\n---\nRename a file by its path.\n";
    fs::write(root.join("files/rename/SKILL.md"), raw)
        .map_err(|err| SkillError::Io(err.to_string()))?;
    let session = SkillSession::open(root, &[])?;
    let mut model = ScriptedModel::new("skill_view rename");
    let call = model.complete(session.index().as_str());
    let name = call.trim_start_matches("skill_view ").trim();
    let body = skill_view(root, name, None)?;
    let line = body.lines().last().unwrap_or("").to_string();
    Ok((name.to_string(), line))
}

fn parse_skill(text: &str, category: &str, dir_name: &str) -> Result<SkillMeta, SkillError> {
    let (front, _body) = split_frontmatter(text);
    let mut name = dir_name.to_string();
    let mut description = String::new();
    let mut requires_mcp = None;
    for line in front.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let value = value.trim().trim_matches('"').to_string();
        match key.trim() {
            "name" if !value.is_empty() => name = value,
            "description" => description = value,
            "requires_mcp" if !value.is_empty() => requires_mcp = Some(value),
            _ => {}
        }
    }
    if name.chars().count() > MAX_NAME_LENGTH {
        return Err(SkillError::Name(name));
    }
    if description.chars().count() > MAX_DESCRIPTION_LENGTH {
        return Err(SkillError::Description(description));
    }
    Ok(SkillMeta {
        name,
        description,
        category: category.to_string(),
        requires_mcp,
    })
}

fn split_frontmatter(text: &str) -> (&str, &str) {
    let rest = text.strip_prefix("---\n").unwrap_or(text);
    if let Some((front, body)) = rest.split_once("\n---\n") {
        if text.starts_with("---\n") {
            return (front, body);
        }
    }
    ("", text)
}

fn index_names(index: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in index.lines() {
        let Some(rest) = line.strip_prefix("    - ") else {
            continue;
        };
        let name = rest.split(':').next().unwrap_or(rest).trim();
        let name = name.split(" [needs:").next().unwrap_or(name).trim();
        if !name.is_empty() {
            names.push(name.to_string());
        }
    }
    names
}

fn find_dir(root: &Path, name: &str) -> Result<PathBuf, SkillError> {
    for category in list_dirs(root)? {
        for skill_dir in list_dirs(&category)? {
            let path = skill_dir.join("SKILL.md");
            if !path.is_file() {
                continue;
            }
            let text = fs::read_to_string(&path).map_err(|err| SkillError::Io(err.to_string()))?;
            let meta = parse_skill(&text, &file_name(&category)?, &file_name(&skill_dir)?)?;
            if meta.name == name {
                return Ok(skill_dir);
            }
        }
    }
    Err(SkillError::Missing(name.to_string()))
}

fn support_path(dir: &Path, rel: &str) -> Result<PathBuf, SkillError> {
    if rel.contains("..") || Path::new(rel).is_absolute() {
        return Err(SkillError::Path(rel.to_string()));
    }
    let path = Path::new(rel);
    let first = path
        .components()
        .next()
        .and_then(|part| part.as_os_str().to_str())
        .unwrap_or("");
    if !SUPPORT.contains(&first) {
        return Err(SkillError::Path(rel.to_string()));
    }
    Ok(dir.join(path))
}

fn list_dirs(root: &Path) -> Result<Vec<PathBuf>, SkillError> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut dirs = Vec::new();
    let entries = fs::read_dir(root).map_err(|err| SkillError::Io(err.to_string()))?;
    for entry in entries {
        let entry = entry.map_err(|err| SkillError::Io(err.to_string()))?;
        if entry.path().is_dir() {
            dirs.push(entry.path());
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn file_name(path: &Path) -> Result<String, SkillError> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .ok_or_else(|| SkillError::Path(path.display().to_string()))
}
