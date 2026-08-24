/// Converts a modeled identifier to the spelling used for Rust fields and
/// symbols. Smithy keeps this legacy spelling for symbols such as `CMKs`.
pub fn snake_case(value: &str) -> String {
    let mut result = String::new();
    let chars = value.chars().collect::<Vec<_>>();
    for (index, character) in chars.iter().enumerate() {
        if character.is_ascii_alphanumeric() {
            let previous = chars.get(index.wrapping_sub(1));
            let next = chars.get(index + 1);
            let boundary = character.is_ascii_uppercase()
                && !result.is_empty()
                && !result.ends_with('_')
                && (previous.is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                    || previous.is_some_and(|c| c.is_ascii_uppercase())
                        && next.is_some_and(|c| c.is_ascii_lowercase()));
            if boundary {
                result.push('_');
            }
            result.push(character.to_ascii_lowercase());
        } else if !result.is_empty() && !result.ends_with('_') {
            result.push('_');
        }
    }
    result.trim_matches('_').to_owned()
}

/// Converts an identifier using Smithy-RS's acronym-aware module spelling.
/// This is intentionally separate from [`snake_case`]: module paths and Rust
/// symbols have different historical casing contracts in smithy-rs.
fn smithy_module_case(value: &str) -> String {
    const COMPLETE_WORDS: &[&str] = &["ipv4", "ipv6", "sigv4", "mib", "gib", "kib", "ttl"];

    let chars = value.chars().collect::<Vec<_>>();
    let all_lowercase = value.to_lowercase() == value;
    let mut result = Vec::new();
    let mut current = String::new();
    let mut complete_word_in_progress = true;

    for (index, &next) in chars.iter().enumerate() {
        let compute_word_in_progress = || {
            complete_word_in_progress
                && !current.is_empty()
                && COMPLETE_WORDS.iter().any(|word| {
                    let remaining = chars[index..].iter().collect::<String>();
                    let candidate = format!("{current}{remaining}");
                    word.starts_with(&current.to_lowercase())
                        && candidate.to_lowercase().starts_with(word)
                        && !word.eq_ignore_ascii_case(&current)
                })
        };

        if !next.is_alphanumeric() {
            emit_word(&mut result, &mut current, next);
            complete_word_in_progress = true;
            continue;
        }
        if current.is_empty() {
            current.push(next);
            continue;
        }

        let word_in_progress = compute_word_in_progress();
        let boundary = (!word_in_progress && lowered_followed_by_upper(&current, next))
            || (!word_in_progress && all_lowercase && digit_followed_by_lower(&current, next))
            || end_of_acronym(
                &current,
                next,
                chars.get(index + 1).copied(),
                chars.get(index + 2).copied(),
            );
        if boundary {
            emit_word(&mut result, &mut current, next);
            complete_word_in_progress = true;
        } else {
            current.push(next);
            complete_word_in_progress = word_in_progress;
        }
    }
    if !current.is_empty() {
        result.push(current.to_lowercase());
    }
    result.join("_")
}

fn emit_word(words: &mut Vec<String>, current: &mut String, next: char) {
    if !current.is_empty() {
        words.push(current.to_lowercase());
    }
    current.clear();
    if next.is_alphanumeric() {
        current.push(next);
    }
}

fn lowered_followed_by_upper(current: &str, next: char) -> bool {
    next.is_ascii_uppercase()
        && current
            .chars()
            .last()
            .is_some_and(|character| character.is_lowercase() || character.is_ascii_digit())
}

fn digit_followed_by_lower(current: &str, next: char) -> bool {
    current
        .chars()
        .last()
        .is_some_and(|character| character.is_ascii_digit())
        && next.is_lowercase()
}

fn end_of_acronym(
    current: &str,
    next: char,
    peek: Option<char>,
    double_peek: Option<char>,
) -> bool {
    if !current
        .chars()
        .last()
        .is_some_and(|character| character.is_uppercase())
        || !next.is_uppercase()
        || !peek.is_some_and(|character| character.is_lowercase())
    {
        return false;
    }
    if peek == Some('s') && !double_peek.is_some_and(|character| character.is_lowercase()) {
        return false;
    }
    if peek == Some('v') && double_peek.is_some_and(|character| character.is_ascii_digit()) {
        return false;
    }
    true
}

pub fn rust_identifier(value: &str) -> String {
    let name = snake_case(value);
    if is_rust_keyword(&name) {
        format!("r#{name}")
    } else {
        name
    }
}

/// Returns the spelling that Rustdoc accepts in an intra-doc link path.
pub fn rustdoc_identifier(value: &str) -> &str {
    value.strip_prefix("r#").unwrap_or(value)
}

pub fn rust_module_name(value: &str) -> String {
    let name = smithy_module_case(value);
    if is_rust_keyword(&name) {
        format!("{name}_")
    } else {
        name
    }
}

fn is_rust_keyword(value: &str) -> bool {
    matches!(
        value,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
    )
}

#[cfg(test)]
mod tests {
    use super::{rust_identifier, rust_module_name, rustdoc_identifier, snake_case};

    #[test]
    fn rust_keywords_use_context_appropriate_names() {
        assert_eq!(rust_identifier("Type"), "r#type");
        assert_eq!(rust_module_name("Type"), "type_");
        assert_eq!(rustdoc_identifier("r#type"), "type");
        assert_eq!(rustdoc_identifier("value"), "value");
    }

    #[test]
    fn follows_smithy_word_boundary_rules() {
        assert_eq!(
            snake_case("CustomKeyStoreHasCMKsException"),
            "custom_key_store_has_cm_ks_exception"
        );
        assert_eq!(
            rust_module_name("CustomKeyStoreHasCMKsException"),
            "custom_key_store_has_cmks_exception"
        );
        assert_eq!(rust_module_name("NotificationARNs"), "notification_arns");
        assert_eq!(rust_module_name("IAMUser"), "iam_user");
        assert_eq!(rust_module_name("DynamoDBv2Action"), "dynamo_dbv2_action");
        assert_eq!(rust_module_name("IpV6Address"), "ipv6_address");
    }
}
