#![allow(unused_doc_comments)]
/**
 * In this module we will go through few examples and explain borrowing in rust lang.
 * What is problem? (line 10)
 * What is borrowing. (line 29)
 * Mutable references example. (line 40)
 * Mutating a string reference example. (line 49)
 */
pub mod module {
    pub fn borrowing() {
        // It is very inconvenient to pass the ownership of a variable to another function and then return the ownership.
        // Rust supports a concept borrowing,
        // where the ownership of a value is transferred temporarily to an entity and then returned to the original owner entity.
        let v = vec![10,20,30];
        print_vector(v); // (line 58)
        // println!("{}",v[0]); // this line gives error
        // The main function invokes a function print_vector().
        // A vector is passed as parameter to this function.
        // The ownership of the vector is also passed to the print_vector() function from the main().
        // The above code will result in an error as shown below when the main() function tries to access the vector v.
        /**
        | print_vector(v);
        | - value moved here
        | println!("{}",v[0]);
        | ^ value used here after move
        */
        // This is because a variable or value can no longer be used
        // by the function that originally owned it once the ownership is transferred to another function.

        // What is borrowing.
        // When a function transfers its control over a variable/value to another function temporarily,
        // for a while, it is called borrowing.
        // This is achieved by passing a reference to the variable (& var_name) rather than passing the variable/value itself to the function.
        // The ownership of the variable/ value is transferred to the original owner of the variable after
        // the function to which the control was passed completes execution.
        // a list of nos
        let v = vec![10,20,30];
        print_vector_pass(&v); // passing reference (line 63)
        println!("Printing the value from main() v[0]={}",v[0]);

        // Mutable references.
        // A function can modify a borrowed resource by using a mutable reference to such resource.
        // A mutable reference is prefixed with &mut. Mutable references can operate only on mutable variables.
        let mut i = 3;
        add_one(&mut i); // (line 68)
        println!("{}", i);
        // The main() function declares a mutable integer variable i and passes a mutable reference of i to the add_one().
        // The add_one() increments the value of the variable i by one.

        // Mutating a string reference
        let mut name:String = String::from("TutorialsPoint");
        display(&mut name); // (line 73)
        //pass a mutable reference of name
        println!("The value of name after modification is:{}",name);
        // The main() function passes a mutable reference of the variable name to the display() function.
        // The display function appends an additional string to the original name variable.
    }

    // Example function to demonstrate error.
    fn print_vector(x:Vec<i32>){
        println!("Inside print_vector function {:?}",x);
    }

    // Example of borrowing function.
    fn print_vector_pass(x:&Vec<i32>){
        println!("Inside print_vector function {:?}",x);
    }

    // Example of mutable reference function.
    fn add_one(e: &mut i32) {
        *e+= 1;
    }

    // Example of mutating a string reference function.
    fn display(param_name:&mut String){
        println!("param_name value is :{}",param_name);
        param_name.push_str(" Rocks");
        //Modify the actual string,name
    }
}