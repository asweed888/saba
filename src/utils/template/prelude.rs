pub fn to_title(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_uppercase().next().unwrap());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        } else if c == '_' {
            capitalize_next = true;
        }
    }

    result
}