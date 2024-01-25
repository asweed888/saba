pub mod generate {
    pub mod codefile {
        pub mod rust {
            pub mod r#mod;
            pub mod template;
            pub mod utils;
        }
        pub mod golang {
            pub mod r#mod;
            pub mod template;
        }
        pub mod python {
            pub mod r#mod;
            pub mod template;
        }
        pub mod typescript {
            pub mod r#mod;
            pub mod template;
        }
        pub mod lua {
            pub mod r#mod;
        }
        pub mod bash {
            pub mod r#mod;
        }
    }
}
pub mod manifest {
    pub mod rust {
        pub mod basic;
        pub mod template;
        pub mod utils;
    }
    pub mod golang {
        pub mod basic;
        pub mod template;
    }
    pub mod python {
        pub mod basic;
        pub mod template;
    }
    pub mod typescript {
        pub mod basic;
        pub mod template;
    }
    pub mod lua {
        pub mod basic;
    }
    pub mod bash {
        pub mod basic;
    }
    pub mod basic;
    pub mod interface;
    pub mod utils;
} // Automatically exported by saba.
