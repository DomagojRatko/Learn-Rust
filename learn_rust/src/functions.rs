/**
 * In this module we will go through few examples and explain functions in rust lang.
 * How to call functions. (line 11)
 * Creating simple function. (line 26)
 * Creating function with parameters. (line 31)
 * Creating function with return value. (line 36)
 * Creating function with pass by reference.(line 41)
 */
pub mod module {
    pub fn function() {
        // Calling simple function.
        example();

        // Calling function with parameters.
        para_example(10);

        // Calling function with return value.
        println!("{}",return_example(2));

        // Calling pass by reference example.
        let mut no:i32 = 5;
        mutate_no_to_zero(&mut no);
        println!("The value of no is:{}",no)
    }

    // Example of simple function.
    fn example() {
        println!("Simple function");
    }

    // Example of simple function with parameters.
    fn para_example(n:u8) {
        println!("{}",n);
    }

    // Example of function with return value.
    fn return_example(n:u8) -> u8 {
        return n + n;
    }

    // Pass by reference example.
    // When you pass parameters by reference, unlike value parameters,
    // a new storage location is not created for these parameters.
    // The reference parameters represent the same memory location as the actual parameters that are supplied to the method.
    // Parameter values can be passed by reference by prefixing the variable name with an & .
    //
    // In the example given below, we have a variable no, which is initially 5.
    // A reference to the variable no is passed to the mutate_no_to_zero() function.
    // The function operates on the original variable.
    // After the function call, when control returns back to main method,
    // the value of the original variable will be the zero.
    fn mutate_no_to_zero(param_no:&mut i32){
        *param_no = 0; //de reference
    }
    // The * operator is used to access value stored in the memory location that the variable param_no points to.
    // This is also known as dereferencing.
}