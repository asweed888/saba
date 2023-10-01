mod domain {
    pub mod model {
        pub mod manifest;
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
}
mod presentation {
    pub mod command {
        pub mod new;
        pub mod up;
    }
}
mod di {
    pub mod container;
} // Automatically exported by saba.


fn main() {
    println!("Hello, world!");
}
