pub fn rust_crate_name(package_name: &str) -> String {
    package_name.replace('-', "_")
}

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

pub fn rust_identifier(value: &str) -> String {
    let name = snake_case(value);
    if is_rust_keyword(&name) {
        format!("r#{name}")
    } else {
        name
    }
}

pub fn rust_module_name(value: &str) -> String {
    let name = snake_case(value);
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
    use super::{rust_identifier, rust_module_name};

    #[test]
    fn rust_keywords_use_context_appropriate_names() {
        assert_eq!(rust_identifier("Type"), "r#type");
        assert_eq!(rust_module_name("Type"), "type_");
    }
}
