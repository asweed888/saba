mod domain {
    pub mod model {
        pub mod manifest;
        pub mod rust_file;
        pub mod go_file;
    }
    pub mod repository {
        pub mod manifest;
        pub mod rust_file;
        pub mod go_file;
    }
}
mod infrastructure {
    pub mod repository {
        pub mod manifest;
        pub mod rust_file;
        pub mod go_file;
    }
}
mod usecase {
    pub mod manifest;
    pub mod rust_file;
    pub mod go_file;
}
mod presentation {
    pub mod command {
        pub mod new;
        pub mod up;
    }
} // Automatically exported by saba.



fn main() {
    println!("Hello, world!");
}
