mod data_types;
mod variables;
mod constant;
mod string;
mod decision_making;
mod loops;
mod functions;
mod tuple;
mod arrays;
mod ownership;
mod borrowing;
mod slices;
mod structure;
mod enums;
mod module;
mod vectors;
mod hashmap;
mod hashset;
mod error_handling;

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

    // Loops explained. Read module loops.rs
    loops::module::loops();

    // Functions explained. Read module functions.rs
    functions::module::function();

    // Tuple explained. Read module tuple.rs
    tuple::module::tuple();

    // Array explained. Read module arrays.rs
    arrays::module::arrays();

    // Ownership explained. Read module ownership.rs
    ownership::module::ownership();

    // Borrowing explained. Read module borrowing.rs
    borrowing::module::borrowing();

    // Slices explained. Read module slices.rs
    slices::module::slices();

    // Structure explained. Read module structure.rs
    structure::module::structure();

    // Enum explained. Read module enums.rs
    enums::module::enums();

    // Modules explained. Read module module.rs
    module::module::modules();

    // Vectors explained. Read module vectors.rs
    vectors::module::vectors();

    // HashMap explained. Read module hashmap.rs
    hashmap::module::hashmap();

    // HashSet explained. Read module hashset.rs
    hashset::module::hashset();

    // Error handling explained. Read module error_handling.rs
    error_handling::module::error_handling();

}