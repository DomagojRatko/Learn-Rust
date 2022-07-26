/**
 * In this module we will go through few examples and explain generics in rust lang.
 * Generics explained. (line 12)
 * Generic structure. (line 18)
 * Traits explained. (line 26)
 * Examples trait. (line 30)
 * Generic functions example. (line 38)
 */
pub mod module {
    use std::fmt::Display;
    pub fn generics() {
        // Generics are a facility to write code for multiple contexts with different types.
        // In Rust, generics refer to the parameterization of data types and traits.
        // Generics allows to write more concise and clean code by reducing code duplication and providing type-safety.
        // The concept of Generics can be applied to methods, functions, structures, enumerations, collections and traits.
        // The <T> syntax known as the type parameter, is used to declare a generic construct. T represents any data-type.

        // Generic structure.
        //generic type of i32
        let t:Data<i32> = Data{value:350}; // (line 45)
        println!("value is :{} ",t.value);
        //generic type of String
        let t2:Data<String> = Data{value:"Tom".to_string()};
        println!("value is :{} ",t2.value);

        // Traits. (line 50)
        // Traits can be used to implement a standard set of behaviors (methods) across multiple structures.
        // Traits are like interfaces in Object-oriented Programming.

        // Examples defines a trait Printable with a method print(), which is implemented by the structure book.
        // Create an instance of the structure.
        let b1 = Book { // (line 60)
            id:1001,
            name:"Rust in Action"
        };
        b1.print();

        // Generic functions
        print_pro(10 as u8); // (line 76)
        print_pro(20 as u16);
        print_pro("Hello TutorialsPoint");
    }

    // Generic structure example.
    struct Data<T> {
        value:T,
    }

    // trait example.
    trait SomeTrait {
        //abstract or method which is empty
        fn method1(&self);
        // this is already implemented , this is free
        fn method2(&self){
            //some contents of method2
        }
    }

    // Declare a structure.
    struct Book {
        name:&'static str,
        id:u32
    }
    // Declare a trait.
    trait Printable {
        fn print(&self);
    }
    // Implement the trait.
    impl Printable for Book {
        fn print(&self){
            println!("Printing book with id:{} and name {}",self.id,self.name)
        }
    }

    // Generic functions example.
    fn print_pro<T:Display>(t:T){
        println!("Inside print_pro generic function:");
        println!("{}",t);
    }
}