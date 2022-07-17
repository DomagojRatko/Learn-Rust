/**
    * In this module we will go through few examples and explain data types in rust lang.
    * How to print line. (line 14)
    * How to print line with some value. (line 17)
    * How comments work. (line 20)
    * How declaring variables works. (line 25)
    * How integers work and how are they declared. (line 32)
    * How integer overflow works. (line 52)
    * How float works. (line 58)
 */
// NOTE: some variables will be named like this _name with _ on beginning. That's only to disable warning for unused variables.
pub mod module {
    pub fn data_types() {
        println!(); // prints just a newline
        println!("Hello, world!"); // prints simple text

        let arg_string = "some"; //variable string that we can print
        println!("format {} arguments", arg_string); //prints format some arguments

        // This is single line comment
        /* This is a
        Multi-line comment
        */

        // declaring variable
        // note compiler can automatically infer data type of the variable based on the value assigned to it.
        let _one = "tutorial";
        let _two = 4.5;
        let _three = true;
        let _four = '♥';

        // Integer
        /*
        Integers can be further classified as Signed and Unsigned.
        Signed integers can store both negative and positive values.
        Unsigned integers can only store positive values.
        A detailed description if integer types is given below.
         */
        let _a:u8 = 255; // i8 = 127;
        let _b:u16 = 65535; // i16 = 32,768;
        let _c:u32 = 4294967295; // i32 = 2147483647;
        let _d:u64 = 18446744073709551615; // i64 = 9,223,372,036,854,775,807;
        let _e:u128 = 340282366920938463463374607431768211455; // i128 = 170141183460469231731687303715884105728;
        /*
        The size of an integer can be arch.
        This means the size of the data type will be derived from the architecture of the machine.
        An integer the size of which is arch will be 32 bits on an x86 machine and 64 bits on an x64 machine.
        An arch integer is primarily used when indexing some sort of collection.
         */
        let _arch:usize = 1; // isize = 1;

        // Integer Overflow
        let price:u8 = 255;
        // let new_price:u8 = 260; // to big number for u8 so overflow will happen.
        println!("age is {} ",price); // output will be 255
        // println!("weight is {}",new_price); // output will be 4;

        // Float
        /* Float data type in Rust can be classified as f32 and f64.
        The f32 type is a single-precision float, and f64 has double precision.
        The default type is f64.
        */
        let _result = 10.00;        //f64 by default
        let _interest:f32 = 8.35;
        let _cost:f64 = 15000.600;  //double precision
    }
}
