//! Which server `ask` talks to, and which key it sends.
//!
//! OpenRouter is the default, because one OpenAI-shaped endpoint can reach
//! many model ids. Any other base URL is still an OpenAI-compatible server
//! (OpenAI itself, Ollama, a local fake) and uses that server's key.

use crate::provider::ProviderError;

pub const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";
pub const DEFAULT_MODEL: &str = "openai/gpt-4o-mini";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Endpoint {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
    pub http_referer: Option<String>,
    pub title: Option<String>,
}

/// Values already read from flags and the environment. The library does not
/// touch process environment variables, so tests can pass them in.
#[derive(Debug, Clone, Copy)]
pub struct EndpointInput<'a> {
    pub base_url: &'a str,
    pub api_key_flag: Option<&'a str>,
    pub openrouter_api_key: Option<&'a str>,
    pub openai_api_key: Option<&'a str>,
    pub model: &'a str,
    pub http_referer: Option<&'a str>,
    pub title: Option<&'a str>,
}

pub fn is_openrouter_host(base_url: &str) -> bool {
    reqwest::Url::parse(base_url)
        .ok()
        .and_then(|url| {
            url.host_str()
                .map(|host| host.eq_ignore_ascii_case("openrouter.ai"))
        })
        .unwrap_or(false)
}

pub fn resolve_endpoint(input: EndpointInput<'_>) -> Result<Endpoint, ProviderError> {
    if input.model.trim().is_empty() {
        return Err(ProviderError::EmptyModel);
    }
    let base_url = {
        let trimmed = input.base_url.trim().trim_end_matches('/');
        if trimmed.is_empty() {
            OPENROUTER_BASE_URL.to_string()
        } else {
            trimmed.to_string()
        }
    };
    let api_key = match nonempty(input.api_key_flag) {
        Some(key) => key.to_string(),
        None if is_openrouter_host(&base_url) => match nonempty(input.openrouter_api_key) {
            Some(key) => key.to_string(),
            None => {
                return Err(ProviderError::MissingKey {
                    name: "OPENROUTER_API_KEY",
                })
            }
        },
        None => nonempty(input.openai_api_key).unwrap_or("").to_string(),
    };
    Ok(Endpoint {
        base_url,
        api_key,
        model: input.model.trim().to_string(),
        http_referer: nonempty(input.http_referer).map(str::to_string),
        title: nonempty(input.title).map(str::to_string),
    })
}

fn nonempty(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|text| !text.is_empty())
}
