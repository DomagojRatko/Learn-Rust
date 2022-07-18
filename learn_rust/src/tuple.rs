/**
 * In this module we will go through few examples and explain tuple in rust lang.
 * Tuple explained. (line 11)
 * Tuple example. (line 18)
 * Tuple select individual element. (line 23)
 * Tuple pass as parameter. (line 29)
 * Destructing tuple example. (line 33)
 */
pub mod module {
    pub fn tuple() {
        // Tuple is a compound data type. A scalar type can store only one type of data.
        // For example, an i32 variable can store only a single integer value.
        // In compound types, we can store more than one value at a time and it can be of different types.
        //
        // Tuples have a fixed length - once declared they cannot grow or shrink in size.
        // The tuple index starts from 0.

        // Tuple example.
        let tuple:(i32,f64,u8) = (-325,4.9,22);
        println!("{:?}",tuple);
        // NOTE: Use the println!("{:?}", tuple_name) syntax to print values in a tuple.

        // Tuple individual values display.
        let new_tuple:(i32, f64, u8) = (-325, 4.9, 22);
        println!("integer is :{:?}", new_tuple.0);
        println!("float is :{:?}", new_tuple.1);
        println!("unsigned integer is :{:?}", new_tuple.2);

        // Pass tuple as parameter.
        let b:(i32,bool,f64) = (110,true,10.9);
        print_fn(b); // (line 40)

        // Destructing tuple.
        // Destructing assignment is a feature of rust wherein we unpack the values of a tuple.
        // This is achieved by assigning a tuple to distinct variables.
        let b:(i32,bool,f64) = (30,true,7.9);
        print(b); // (line 46)
    }

    // Passing tuple as parameter function.
    fn print_fn(x:(i32,bool,f64)){
        println!("Inside print method");
        println!("{:?}",x);
    }

    // Tuple destructing function.
    fn print(x:(i32,bool,f64)) {
        println!("Inside print method");
        let (age,is_male,cgpa) = x; //assigns a tuple to distinct variables
        println!("Age is {} , isMale? {},cgpa is {}",age,is_male,cgpa);
    }
}