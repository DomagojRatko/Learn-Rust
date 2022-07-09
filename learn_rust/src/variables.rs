/**
    * In this module we will go through few examples and explain variables in rust lang.
    * Rust naming conventions. (line 8)
    * Syntax and type specification. (line 28)
    * Immutable variables. (line 35)
    * Mutable variables. (line 44)
 */
// NOTE: some variables will be named like this _name with _ on beginning. That's only to disable warning for unused variables.
pub mod module {
    pub fn variables() {
        /*  * Item      Convention
            * Crates	unclear
            * Modules	snake_case
            * Types	UpperCamelCase
            * Traits	UpperCamelCase
            * Enum variants	UpperCamelCase
            * Functions	snake_case
            * Methods	snake_case
            * General constructors	new or with_more_details
            * Conversion constructors	from_some_other_type
            * Macros	snake_case!
            * Local variables	snake_case
            * Statics	SCREAMING_SNAKE_CASE
            * Constants	SCREAMING_SNAKE_CASE
            * Type parameters	concise UpperCamelCase, usually single uppercase letter: T
            * Lifetimes	short lowercase, usually a single letter: 'a, 'de, 'src
            * Features	unclear but see C-FEATURE
         */

        // Syntax
        // The data type is optional while declaring a variable in Rust.
        let _variable_name = 20; // no type specified
        let _variable_name: &str = "value"; //type specified

        // Immutable
        /* By default, variables are immutable − read only in Rust.
        In other words, the variable's value cannot be changed once a value is bound to a variable name.
         */
        let fees = 25_000;
        println!("fees is {} ",fees);
        // fees = 35_000; Note: This will not work!
        println!("fees changed is {}",fees);

        // Mutable
        /* Variables are immutable by default.
        Prefix the variable name with mut keyword to make it mutable.
        The value of a mutable variable can be changed.
         */
        let mut _some_fees:i32 = 25_000;
        println!("fees is {} ",fees);
        _some_fees = 35_000; // NOTE: Adding keyword mut to variable we can change its value.
        println!("fees changed is {}",fees);
    }
}