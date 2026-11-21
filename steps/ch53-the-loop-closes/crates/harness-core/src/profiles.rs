//! Book chapter 38. Several personas share one process. Each profile has its
//! own secret map. The prompt is the persona. A multiplexed read with no
//! scope fails closed.

use std::collections::HashMap;
use std::sync::Arc;

/// Hermes `UnscopedSecretError`: multiplex is on and no profile scope is installed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnscopedSecretError {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    Unscoped(UnscopedSecretError),
    UnknownProfile,
}

pub struct Profile {
    name: String,
    persona: Arc<str>,
    secrets: HashMap<String, String>,
}

impl Profile {
    pub fn new(
        name: impl Into<String>,
        persona: impl Into<String>,
        secrets: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        Self {
            name: name.into(),
            persona: Arc::<str>::from(persona.into()),
            secrets: secrets
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn persona(&self) -> &str {
        &self.persona
    }
}

pub struct Resolved {
    pub prompt: Arc<str>,
    pub has_secret: bool,
}

pub struct Profiles {
    profiles: HashMap<String, Profile>,
    process_env: HashMap<String, String>,
    multiplex: bool,
    scope: Option<String>,
}

impl Profiles {
    pub fn new(multiplex: bool) -> Self {
        Self {
            profiles: HashMap::new(),
            process_env: HashMap::new(),
            multiplex,
            scope: None,
        }
    }

    pub fn add(&mut self, profile: Profile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn set_process(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.process_env.insert(name.into(), value.into());
    }

    pub fn set_scope(&mut self, name: Option<&str>) {
        self.scope = name.map(str::to_string);
    }

    /// Resolve one credential. Multiplex with no scope raises. A scope never
    /// reads another profile's map, and under multiplex a miss does not fall
    /// through to the process map.
    pub fn get_secret(&self, name: &str) -> Result<Option<String>, UnscopedSecretError> {
        if let Some(scope) = &self.scope {
            let value = self
                .profiles
                .get(scope)
                .and_then(|profile| profile.secrets.get(name))
                .cloned();
            if value.is_some() || self.multiplex {
                return Ok(value);
            }
            return Ok(self.process_env.get(name).cloned());
        }
        if self.multiplex {
            return Err(UnscopedSecretError {
                name: name.to_string(),
            });
        }
        Ok(self.process_env.get(name).cloned())
    }

    pub fn prompt(&self, profile: &str) -> Option<Arc<str>> {
        self.profiles
            .get(profile)
            .map(|profile| Arc::clone(&profile.persona))
    }

    /// Persona plus whether the named secret is present. The secret bytes are
    /// not written into `prompt`.
    pub fn resolve(&self, profile: &str, secret_name: &str) -> Result<Resolved, ResolveError> {
        let Some(found) = self.profiles.get(profile) else {
            return Err(ResolveError::UnknownProfile);
        };
        let secret = self
            .get_secret(secret_name)
            .map_err(ResolveError::Unscoped)?;
        Ok(Resolved {
            prompt: Arc::clone(&found.persona),
            has_secret: secret.is_some(),
        })
    }

    pub fn secret_leaked_into_prompt(&self, profile: &str) -> bool {
        let Some(found) = self.profiles.get(profile) else {
            return false;
        };
        found
            .secrets
            .values()
            .any(|secret| !secret.is_empty() && found.persona.contains(secret.as_str()))
    }
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

pub fn demo() -> String {
    let mut store = Profiles::new(true);
    store.add(Profile::new(
        "writer",
        "you write",
        [("API_KEY", "sk-writer")],
    ));
    store.set_scope(Some("writer"));
    let resolved = store
        .resolve("writer", "API_KEY")
        .expect("writer is in scope");
    let _ = FakeModel { reply: "ok" }.complete(&resolved.prompt);
    "profile: writer".to_string()
}
