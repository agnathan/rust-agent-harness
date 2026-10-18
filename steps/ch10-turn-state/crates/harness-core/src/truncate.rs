//! Head-and-tail truncation for one tool result. The kept slice is
//! `max_chars` long. The notice sits on top of that, which is how Hermes
//! `truncate_head_tail` spends the budget.

pub const TOOL_OUTPUT_MAX_CHARS: usize = 1024;
pub const HEAD_RATIO: f64 = 0.4;

pub fn truncate_head_tail(text: &str, max_chars: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if max_chars == 0 || chars.len() <= max_chars {
        return text.to_string();
    }
    let head_len = ((max_chars as f64) * HEAD_RATIO) as usize;
    let tail_len = max_chars - head_len;
    let omitted = chars.len() - head_len - tail_len;
    let head: String = chars[..head_len].iter().collect();
    let tail: String = chars[chars.len() - tail_len..].iter().collect();
    format!(
        "{head}\n\n... [OUTPUT TRUNCATED - {} chars omitted out of {} total] ...\n\n{tail}",
        with_commas(omitted),
        with_commas(chars.len())
    )
}

fn with_commas(value: usize) -> String {
    let digits = value.to_string();
    let mut out = String::new();
    for (index, ch) in digits.chars().rev().enumerate() {
        if index > 0 && index % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_short_result_is_unchanged_and_a_long_one_keeps_both_ends() {
        assert_eq!(truncate_head_tail("hello", 1024), "hello");
        let text = "abcdefghij".repeat(1_000);
        let truncated = truncate_head_tail(&text, 100);
        assert!(truncated.contains("OUTPUT TRUNCATED"));
        assert!(truncated.contains("9,900 chars omitted"));
        assert!(truncated.contains("out of 10,000 total"));
        assert!(truncated.starts_with(&text[..40]));
        assert!(truncated.ends_with(&text[text.len() - 60..]));
    }
}
