/// Convert a name to PascalCase for Rust type names
pub fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' || c == '.' || c == ' ' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

/// Convert a name to snake_case for Rust field names
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_was_upper = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_was_upper {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_was_upper = true;
        } else {
            result.push(c);
            prev_was_upper = false;
        }
    }

    result
}

/// Sanitize field names to be valid Rust identifiers
pub fn sanitize_field_name(s: &str) -> String {
    // Replace invalid characters
    let s = s.replace('-', "_").replace('.', "_").replace(' ', "_");

    // If it starts with a number or is a Rust keyword, prefix with underscore
    if s.chars().next().map(|c| c.is_numeric()).unwrap_or(false) || is_rust_keyword(&s) {
        format!("_{}", s)
    } else {
        s
    }
}

/// Check if a string is a Rust keyword
fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
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
            | "Self"
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
    )
}

/// Escape strings for Rust doc comments
pub fn escape_doc_comment(s: &str) -> String {
    s.replace("*/", "* /")
}

/// Generate doc comment from description
pub fn generate_doc_comment(description: &Option<String>) -> String {
    if let Some(desc) = description {
        let escaped = escape_doc_comment(desc);
        format!("/// {}\n", escaped.replace('\n', "\n/// "))
    } else {
        String::new()
    }
}
