/// Left-pad a hex string with a leading zero if it has an odd number of characters.
/// `hex::decode` requires even-length input (each byte = 2 hex chars).
pub fn pad_hex(hex: &str) -> String {
    if hex.len().is_multiple_of(2) {
        hex.to_string()
    } else {
        format!("0{hex}")
    }
}

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
    let mut result = String::with_capacity(str.len());
    let mut capitalize_next = true;

    for ch in str.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }

    result
}
