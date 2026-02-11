pub fn to_snake_case(str: &str) -> String {
    let mut out = String::with_capacity(str.len());

    for (i, char) in str.chars().enumerate() {
        if char.is_uppercase() && i != 0 {
            out.push('_');
        }

        out.push(char.to_ascii_lowercase());
    }

    out
}

pub fn to_pascal_case(str: &str) -> String {
    if let Some((first, rest)) = str.chars().next().map(|first| (first, &str[1..])) {
        let mut result = String::with_capacity(str.len());

        result.push(first.to_ascii_uppercase());
        result.push_str(rest);

        result
    } else {
        String::new()
    }
}
