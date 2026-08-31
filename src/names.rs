//! Rust identifier spelling shared by model discovery and source mapping.

/// Smithy-RS's historical snake-case spelling for shape/module names.
pub fn snake_case(value: &str) -> String {
    const COMPLETE_WORDS: &[&str] = &["ipv4", "ipv6", "sigv4", "mib", "gib", "kib", "ttl"];
    let chars = value.chars().collect::<Vec<_>>();
    let mut result = String::new();
    let mut complete_word_in_progress = true;
    for (index, character) in chars.iter().enumerate() {
        if character.is_ascii_alphanumeric() {
            let current_word = result.rsplit('_').next().unwrap_or_default();
            let remaining = chars[index..].iter().collect::<String>();
            let word_in_progress = complete_word_in_progress
                && COMPLETE_WORDS.iter().any(|word| {
                    word.starts_with(&current_word.to_ascii_lowercase())
                        && format!("{current_word}{remaining}")
                            .to_ascii_lowercase()
                            .starts_with(word)
                        && !word.eq_ignore_ascii_case(current_word)
                });
            complete_word_in_progress = word_in_progress;
            let previous = chars.get(index.wrapping_sub(1));
            let next = chars.get(index + 1);
            let boundary = character.is_ascii_uppercase()
                && !result.is_empty()
                && !result.ends_with('_')
                && !word_in_progress
                && (previous.is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                    || (previous.is_some_and(|c| c.is_ascii_uppercase())
                        && next.is_some_and(|c| c.is_ascii_lowercase())));
            if boundary {
                complete_word_in_progress = true;
                result.push('_');
            }
            result.push(character.to_ascii_lowercase());
        } else {
            complete_word_in_progress = true;
            if !result.is_empty() && !result.ends_with('_') {
                result.push('_');
            }
        }
    }
    result.trim_matches('_').to_owned()
}

fn smithy_snake_case(value: &str) -> String {
    const COMPLETE_WORDS: &[&str] = &["ipv4", "ipv6", "sigv4", "mib", "gib", "kib", "ttl"];
    let chars = value.chars().collect::<Vec<_>>();
    let all_lowercase = value.to_lowercase() == value;
    let mut result = Vec::new();
    let mut current = String::new();
    let mut complete_word_in_progress = true;
    for (index, &next) in chars.iter().enumerate() {
        if !next.is_alphanumeric() {
            emit_word(&mut result, &mut current, next);
            complete_word_in_progress = true;
            continue;
        }
        if current.is_empty() {
            current.push(next);
            continue;
        }
        let remaining = chars[index..].iter().collect::<String>();
        let word_in_progress = complete_word_in_progress
            && COMPLETE_WORDS.iter().any(|word| {
                word.starts_with(&current.to_lowercase())
                    && format!("{current}{remaining}")
                        .to_lowercase()
                        .starts_with(word)
                    && !word.eq_ignore_ascii_case(&current)
            });
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

pub fn rust_module_name(value: &str) -> String {
    let name = smithy_snake_case(value);
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
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
            | "union"
    )
}

#[cfg(test)]
mod tests {
    use super::{rust_module_name, snake_case};

    #[test]
    fn follows_smithy_boundaries() {
        assert_eq!(rust_module_name("HeadBucket"), "head_bucket");
        assert_eq!(rust_module_name("ListObjectsV2"), "list_objects_v2");
        assert_eq!(
            snake_case("CustomKeyStoreHasCMKsException"),
            "custom_key_store_has_cm_ks_exception"
        );
        assert_eq!(rust_module_name("NotificationARNs"), "notification_arns");
        assert_eq!(rust_module_name("Type"), "type_");
    }
}
