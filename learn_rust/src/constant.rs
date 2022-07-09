/**
 * In this module we will go through few examples and explain constant in rust lang.
 * How do declare constant. (line 8)
 * How shadow variable. (line 20)
 * Constants can't be shadowed.(line 31)
 */
// NOTE: some variables will be named like this _name with _ on beginning. That's only to disable warning for unused variables.
pub mod module {
    pub fn constant() {
        /* Constants represent values that cannot be changed.
        If you declare a constant then there is no way its value changes.
        The keyword for using constants is const. Constants must be explicitly typed.
        Constants can be declared in any scope, including the global scope,
        which makes them useful for values that many parts of the code need to know about.
        Following is the syntax to declare a constant.
         */
        const _VARIABLE_NAME:&str = "value";
        // NOTE: constant declaration must specify the data type and constants are immutable.

        // Shadowing of variables and constants.
        // The new variable overrides the previous variable.
        let price = 100.00;
        let price = 50.50 ;
        println!("The value of salary is :{}",price);
        // NOTE: Output will be "The value of salary is :1.50".
        // NOTE: Rust supports variables with different data types while shadowing.
        let uname = "Alex";
        let uname = uname.len();
        println!("name changed to integer : {}",uname);

        // Constants cannot be shadowed
        // const USER:&str = "Joe";
        // const USER:usize = NAME.len();
        // Error : `NAME` already defined
    }
}