mod domain {
    pub mod model {
        pub mod manifest;
        pub mod rust;
        pub mod go;
    }
    pub mod repository {
        pub mod manifest;
    }
}
mod infrastructure {
    pub mod repository {
        pub mod manifest;
    }
}
mod usecase {
    pub mod manifest;
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
