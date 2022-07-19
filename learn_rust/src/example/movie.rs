// pub mod - creates public module
pub mod movies {
    // pub fn - creates public function
    pub fn play(name: &str) {
        println!("Playing movie {}",name);
    }
}

// So lets now explain how can we access this movie.rs
// This is example structure.
// src
// ├── main.rs
// └── movie.rs

// main.rs
// mod movie;
// fn main() {
//     movie::movies::play("Rust the hero!");
// }

// movie.rs
// pub mod movies {
//     pub fn play(name: &str) {
//         println!("Playing movie {}",name);
//     }
// }


// So in case we would want to set movie.rs in different package and call it.
// This is example structure.
// src
// ├── main.rs
// ├── module.rs
// └── example
//     └── movie.rs

// main.rs
// mod module;
// #[path = "example/movie.rs"] mod movie;
// fn main() {
//     module::module::modules();
// }

// module.rs
// pub mod module {
//     use crate::movie;
//     pub fn modules() {
//         movie::movies::play("Rust the hero!");
//     }
// }

// movie.rs
// pub mod movies {
//     pub fn play(name: &str) {
//         println!("Playing movie {}",name);
//     }
// }
