mod data_types;
mod variables;
mod constant;

// main function to run program
// use on run --release to make program run faster
fn main() {
    // Data types explained. Read model data_types.rs
    data_types::module::data_types();

    // Variables explained. Read module variables.rs
    variables::module::variables();

    // Constant and shadowing explained. Read module constant.rs
    constant::module::constant();

}
