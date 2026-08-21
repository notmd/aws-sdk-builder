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
    match name.as_str() {
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern" | "false"
        | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move"
        | "mut" | "pub" | "ref" | "return" | "self" | "Self" | "static" | "struct" | "super"
        | "trait" | "true" | "type" | "unsafe" | "use" | "where" | "while" | "async" | "await"
        | "dyn" => format!("r#{name}"),
        _ => name,
    }
}
