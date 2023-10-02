mod domain {
    pub mod model {
        pub mod lang {
            pub mod rust;
            pub mod go;
        }
        pub mod code_file_recipe;
        pub mod manifest;
    }
    pub mod repository {
        pub mod code_file_recipe;
        pub mod manifest;
    }
}
mod infrastructure {
    pub mod repository {
        pub mod code_file_recipe;
        pub mod manifest;
    }
} // Automatically exported by saba.


fn main() {
    println!("Hello, world!");
}
