mod domain {
    pub mod model {
        pub mod manifest;
        pub mod file_profile;
    }
    pub mod repository {
        pub mod manifest;
        pub mod file_profile;
    }
}
mod infrastructure {
    pub mod repository {
        pub mod manifest;
        pub mod file_profile;
    }
} // Automatically exported by saba.


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
