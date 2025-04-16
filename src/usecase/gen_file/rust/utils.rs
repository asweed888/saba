pub fn contains_traits_str(path: &str) -> bool {
    path.contains("/abils/")
}

pub fn contains_act_str(path: &str) -> bool {
    path.contains("/act/")
}

pub fn is_traits_file(fname: &str) -> bool {
    fname == "abils"
}
