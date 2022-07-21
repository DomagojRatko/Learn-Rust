/**
 * In this module we will go through few examples and explain structure in rust lang.
 * Structure explained. (line 13)
 * Usage of structure example. (line 17)
 * Modifying a struct instance. (line 25)
 * Passing a struct as parameter. (line 34)
 * Returning structure from a function. (line 37)
 * Method in structure. (line 40)
 * Static method in structure. (line 55)
 */
pub mod module {
    pub fn structure() {
        // Arrays are used to represent a homogeneous collection of values.
        // Similarly, a structure is another user defined data type available in Rust that allows us to combine data items of different types,
        // including another structure. A structure defines data as a key-value pair.

        // Usage of structure.
        let emp1 = Employee { // (line 64)
            company:String::from("RustLearn"),
            name:String::from("Joe"),
            age:50
        };
        println!("Name is :{} company is {} age is {}",emp1.name,emp1.company,emp1.age);

        // Modifying a struct instance.
        let mut emp2 = Employee {
            company:String::from("RustLearn"),
            name:String::from("Joe"),
            age:50
        };
        emp2.age = 40; // (Needs to be mutable)
        println!("Name is :{} company is {} age is {}",emp2.name,emp2.company,emp2.age);

        // Passing a struct as parameter.
        display(emp2); // (line 71)

        // Returning structure from a function.
        who_is_elder(emp1); // (line 76)

        // Method in structure.
        // Methods are like functions. They are a logical group of programming instructions.
        // Methods are declared with the fn keyword. The scope of a method is within the structure block.
        // Methods are declared outside the structure block.
        // The impl keyword is used to define a method within the context of a structure.
        // The first parameter of a method will be always self, which represents the calling instance of the structure.
        // Methods operate on the data members of a structure.
        // To invoke a method, we need to first instantiate the structure.
        // The method can be called using the structure's instance.
        let small = Rectangle { // (line 86) and imp for calculating area of rectangle (line 91)
            width:10,
            height:20
        };
        println!("width is {} height is {} area of Rectangle is {}",small.width,small.height,small.area());

        // Static method in structure.
        // Static methods can be used as utility methods.
        // These methods exist even before the structure is instantiated.
        // Static methods are invoked using the structure's name and can be accessed without an instance.
        // Unlike normal methods, a static method will not take the &self parameter.
        let p1 = Point::get_instance(10,20); // (line 104)
        p1.display(); // (line 111)
    }

    // Structure example.
    struct Employee {
        company:String,
        name:String,
        age:u32
    }

    // Passing structure function.
    fn display( emp:Employee) {
        println!("Name is :{} company is {} age is {}",emp.name,emp.company,emp.age);
    }

    // Return structure function.
    fn who_is_elder(emp:Employee) -> Employee {
        if emp.age > 40 {
            println!("Employee {} is older than 40. His age is {}", emp.name, emp.age)
        } else {
            println!("Employee {} is younger than 40. His age is {}", emp.name, emp.age)
        }
        return emp;
    }

    // Structure example.
    struct Rectangle {
        width:u32, height:u32
    }

    // Logic to calculate area of a rectangle.
    impl Rectangle {
        fn area(&self)->u32 {
            // Use the . operator to fetch the value of a field via the self keyword.
            self.width * self.height
        }
    }

    // Structure example.
    struct Point {
        x: i32, y: i32,
    }

    // `impl` is used to define methods for a structure.
    impl Point {
        // Static method that creates objects of the Point structure.
        fn get_instance(x: i32, y: i32) -> Point {
            Point { x, y }
        }
        // Display values of the structure's field.
        fn display(&self){
            println!("x={} y={}",self.x,self.y );
        }
    }
}