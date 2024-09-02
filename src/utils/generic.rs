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

pub fn default_struct(pkgname: &str, fname: &str) -> String {
    if fname == "mod" || fname == "entity" {
        to_title(pkgname)
    }
    else {
        to_title(fname)
    }
}

pub fn either(pkgname: &str, fname: &str, num: u16) -> String {
    match pkgname {
        "usecase" => {
            match num {
                0 => fname.to_string(),
                _ => pkgname.to_string()
            }
        }
        _ => {
            match num {
                0 => pkgname.to_string(),
                _ => fname.to_string()
            }
        }
    }
}