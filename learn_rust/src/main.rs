mod data_types;
mod variables;
mod constant;
mod string;
mod decision_making;
mod loops;

// main function to run program
// use on run --release to make program run faster
fn main() {
    // Data types explained. Read model data_types.rs
    data_types::module::data_types();

    // Variables explained. Read module variables.rs
    variables::module::variables();

    // Constant and shadowing explained. Read module constant.rs
    constant::module::constant();

    // String data types explained. Read module string.rs
    string::module::string();

    // Decision making or if statements explained. Read module decision_making.rs
    decision_making::module::decision_making();

    // Loops explained. Read module loops
    loops::module::loops();

}
