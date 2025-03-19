pub fn contains_traits_str(path: &str) -> bool {
    path.contains("/traits/")
}

pub fn contains_act_str(path: &str) -> bool {
    path.contains("/act/")
}

pub fn is_ability_file(fname: &str) -> bool {
    fname == "ability"
}
