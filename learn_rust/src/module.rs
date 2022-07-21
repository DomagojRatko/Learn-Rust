/**
 * In this module we will go through few examples and explain modules in rust lang.
 * Modules explained. (line 11)
 * Defining module example. (line 35)
 * 'use' keyword syntax. (line 38)
 */
#[path = "example/movie.rs"] mod movie;
pub mod module {
    use crate::module::movie;
    pub fn modules() {
        // A logical group of code is called a Module.
        // Multiple modules are compiled into a unit called crate.
        // Rust programs may contain a binary crate or a library crate.
        // A binary crate is an executable project that has a main() method.
        // A library crate is a group of components that can be reused in other projects.
        // Unlike a binary crate, a library crate does not have an entry point (main() method).
        // The Cargo tool is used to manage crates in Rust.
        // For example, the network module contains networking related functions and the graphics module contains drawing-related functions.
        // Modules are similar to namespaces in other programming languages.
        // Third-party crates can be downloaded using cargo from --> crates.io

        // Term & Description
        // | crate |
        // Is a compilation unit in Rust; Crate is compiled to binary or library.

        // | cargo |
        // The official Rust package management tool for crates.

        // | module |
        // Logically groups code within a crate.

        // | crates.io |
        // The official Rust package registry.

        // Defining module example. Check src\example\movie.rs
        movie::movies::play("Rust the hero!");

        // Keyword use syntax.
        // use public_module_name::function_name;
    }
}