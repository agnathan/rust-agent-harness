//! E1 and E2. `agent/redact.py` on Hermes `main`: `redact_sensitive_text` and
//! `RedactingFormatter`. `redact_for_egress` is that scrub plus the Bearer residue
//! sweep. Patterns below are the prefix families read from `_PREFIX_PATTERNS`
//! (provider keys, GitHub, Slack, Google, AWS, Stripe, Hugging Face, npm, GitLab)
//! plus env assignments, `Authorization` headers, and a PEM block. Plugin-registered
//! patterns and the vault are not copied.

use std::sync::OnceLock;

use regex::Regex;

const MASK: &str = "***";

fn patterns() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(concat!(
            r"sk-[A-Za-z0-9_-](?:\.?[A-Za-z0-9_-]){9,}",
            r"|ghp_[A-Za-z0-9]{10,}",
            r"|github_pat_[A-Za-z0-9_]{10,}",
            r"|gho_[A-Za-z0-9]{10,}",
            r"|ghu_[A-Za-z0-9]{10,}",
            r"|ghs_[A-Za-z0-9]{10,}",
            r"|ghr_[A-Za-z0-9]{10,}",
            r"|xox[baprs]-[A-Za-z0-9-]{10,}",
            r"|AIza[A-Za-z0-9_-]{30,}",
            r"|AKIA[A-Z0-9]{16}",
            r"|sk_live_[A-Za-z0-9]{10,}",
            r"|sk_test_[A-Za-z0-9]{10,}",
            r"|hf_[A-Za-z0-9]{10,}",
            r"|npm_[A-Za-z0-9]{10,}",
            r"|glpat-[A-Za-z0-9_\-]{10,}",
            r"|-----BEGIN [A-Z ]*PRIVATE KEY-----[\s\S]*?-----END [A-Z ]*PRIVATE KEY-----",
        ))
        .expect("redaction pattern")
    })
}

fn env_assign() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(
            r"(?i)\b([A-Z0-9_]*(?:API_?KEY|TOKEN|SECRET|PASSWORD|PASSWD|CREDENTIAL)[A-Z0-9_]*)\s*=\s*(\S+)",
        )
        .expect("env pattern")
    })
}

fn auth_header() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r#"(?i)((?:Proxy-)?Authorization:\s*)([A-Za-z][\w.+-]*\s+)?([^\s"']+)"#)
            .expect("auth header")
    })
}

fn bearer_residue() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)\bBearer\s+(?:\[[^\]]+\]|[A-Za-z0-9._~+/-]{20,}=*)").expect("bearer")
    })
}

/// `redact_sensitive_text`. Short matches are fully masked. The prefix stays so a
/// second pass does not see a new key.
pub fn redact_sensitive_text(text: &str) -> String {
    let prefixed = patterns().replace_all(text, |caps: &regex::Captures| {
        let raw = caps.get(0).map(|m| m.as_str()).unwrap_or("");
        let prefix_len = raw.find(['-', '_']).map(|i| i + 1).unwrap_or(0);
        format!("{}{MASK}", &raw[..prefix_len])
    });
    let assigned = env_assign().replace_all(&prefixed, |caps: &regex::Captures| {
        format!("{}={MASK}", caps.get(1).map(|m| m.as_str()).unwrap_or(""))
    });
    auth_header()
        .replace_all(&assigned, |caps: &regex::Captures| {
            format!("{}{MASK}", caps.get(1).map(|m| m.as_str()).unwrap_or(""))
        })
        .into_owned()
}

/// `redact_for_egress`. The sensitive-text scrub, then the Bearer residue sweep.
pub fn redact_for_egress(text: &str) -> String {
    let scrubbed = redact_sensitive_text(text);
    bearer_residue()
        .replace_all(&scrubbed, "Bearer [redacted]")
        .into_owned()
}

/// `RedactingFormatter`: format, then scrub. This is a function so the harness
/// does not take a logging facade. Call it on the line you are about to log.
pub fn redacting_formatter(line: &str) -> String {
    redact_sensitive_text(line)
}
