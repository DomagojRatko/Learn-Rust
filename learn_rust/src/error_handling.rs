/**
 * In this module we will go through few examples and explain error handling in rust lang.
 * Errors handling explained. (line 16)
 * Example of panic! macro. (line 31)
 * Example of enum and recoverable errors. (line 39)
 * Example of recoverable errors. (line 41)
 * Example of return error. (line 53)
 * unwrap() and expect() explained. (line 65)
 * Example unwrap(). (line 76)
 * Example expect(). (line 83)
 */
// NOTE: some variables will be named like this _name with _ on beginning. That's only to disable warning for unused variables.
pub mod module {
    use std::fs::File;
    pub fn error_handling() {
        // Errors can be classified into two major categories.
        // Recoverable and unRecoverable.
        // A recoverable error is an error that can be corrected.
        // A program can retry the failed operation or specify an alternate course of action when it encounters a recoverable error.
        // Recoverable errors do not cause a program to fail abruptly. An example of a recoverable error is File Not Found error.
        //
        // Unrecoverable errors cause a program to fail abruptly.
        // A program cannot revert to its normal state if an unrecoverable error occurs.
        // It cannot retry the failed operation or undo the error.
        // An example of an unrecoverable error is trying to access a location beyond the end of an array.
        //
        // Unlike other programming languages, Rust does not have exceptions.
        // It returns an enum Result<T, E> for recoverable errors, while it calls the panic macro if the program encounters an unrecoverable error.
        // The panic macro causes the program to exit abruptly.

        // Panic macro and unrecoverable errors
        // panic!("Hello");
        // println!("End of main"); //unreachable statement

        // panic! macro
        let _a = [10,20,30];
        // a[10]; //invokes a panic since index 10 cannot be reached

        // Result enum and recoverable errors. (line 89)

        // Example of recoverable errors.
        let f = File::open("main.jpg");   // main.jpg doesn't exist
        match f {
            Ok(f)=> {
                println!("file found {:?}",f);
            },
            Err(e)=> {
                println!("file not found \n{:?}",e);   //handled error
            }
        }
        println!("end of main");

        // Example of return error.
        let result = is_even(13); // (line 95)
        match result {
            Ok(d)=>{
                println!("no is even {}",d);
            },
            Err(msg)=>{
                println!("Error msg is {}",msg);
            }
        }
        println!("end of main");

        // unwrap() and expect() explained.
        // The standard library contains a couple of helper methods that both enums − Result<T,E> and Option<T> implement.
        // You can use them to simplify error cases where you really do not expect things to fail.
        // In case of success from a method, the "unwrap" function is used to extract the actual result.
        //
        // Unwrap - Expects self to be Ok/Some and returns the value contained within.
        // If it is Err or None instead, it raises a panic with the contents of the error displayed.
        //
        // Expect - Behaves like unwrap,
        // except that it outputs a custom message before panicking in addition to the contents of the error.

        // Example unwrap()
        let result = is_even(10).unwrap(); // (line 95)
        println!("result is {}",result);
        println!("end of main");
        // Modify the above code to pass an odd number to the is_even() function.
        // The unwrap() function will panic and return a default error message.

        // Example expect()
        // let f = File::open("pqr.txt").expect("File not able to open");
        //file does not exist
        println!("end of main");
        // The function expect() is similar to unwrap(). The only difference is that a custom error message can be displayed using expect.
    }

    // Example of enum with recoverable errors.
    // enum Result<T,E> {
    //     OK(T),
    //     Err(E)
    // }

    // Example of function with return recoverable errors.
    fn is_even(no:i32)->Result<bool,String> {
        if no % 2 == 0 {
            return Ok(true);
        } else {
            return Err("NOT_AN_EVEN".to_string());
        }
    }
}