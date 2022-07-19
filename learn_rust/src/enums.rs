/**
 * In this module we will go through few examples and explain enum in rust lang.
 * Enum example. (line 13)
 * Struct and enum example. (line 19)
 * Option enum explained. (line 31)
 * Match statement and enum. (line 43)
 * Match with option. (line 49)
 * Match & enum with data type. (line 61)
 */
// NOTE: some variables will be named like this _name with _ on beginning. That's only to disable warning for unused variables.
pub mod module {
    pub fn enums() {
        // Enum example.
        let male = GenderCategory::Male; // (line 78)
        let female = GenderCategory::Female;
        println!("{:?}",male);
        println!("{:?}",female);

        // Struct and enum.
        let p1 = Person { // (line 85)
            _name:String::from("Joe"),
            _gender:GenderCategory::Male
        };
        let p2 = Person {
            _name:String::from("Ana"),
            _gender:GenderCategory::Female
        };
        println!("{:?}",p1);
        println!("{:?}",p2);

        // Option enum.
        // Here, the type T represents value of any type.
        // Rust does not support the null keyword.
        // The value None, in the enumOption, can be used by a function to return a null value.
        // If there is data to return, the function can return Some(data).
        // The program defines a function is_even(), with a return type Option.
        // The function verifies if the value passed is an even number.
        // If the input is even, then a value true is returned, else the function returns None.
        let result = is_even(3); // (line 91)
        println!("{:?}",result);
        println!("{:?}",is_even(30));

        // Match statement and enum.
        // The match statement can be used to compare values stored in an enum.
        print_size(CarType::SUV); // enum = (line 100) and function = (line 107)
        print_size(CarType::Hatch);
        print_size(CarType::Sedan);

        // Match with option.
        match is_even(5) { // (line 91)
            Some(data) => {
                if data==true {
                    println!("Even no");
                }
            },
            None => {
                println!("not even");
            }
        }

        // Match & enum with data type.
        // It is possible to add data type to each variant of an enum.
        let p1 = GenderCategoryNew::Name(String::from("Jack")); // (line 121)
        let p2 = GenderCategoryNew::UsrId(100);
        println!("{:?}",p1);
        println!("{:?}",p2);
        match p1 {
            GenderCategoryNew::Name(val)=> {
                println!("{}",val);
            }
            GenderCategoryNew::UsrId(val)=> {
                println!("{}",val);
            }
        }
    }

    #[derive(Debug)] // The attribute #[derive(Debug)] is used to suppress error the trait std::fmt::Debug is not implemented for GenderCategory.
    enum GenderCategory {
        Male,Female
    }

    // The `derive` attribute automatically creates the implementation.
    // Required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct Person {
        _name:String,
        _gender:GenderCategory
    }

    // Option enum example function.
    fn is_even(no:i32) -> Option<bool> {
        if no % 2 == 0 {
            Some(true)
        } else {
            None
        }
    }

    // Enum example.
    enum CarType {
        Hatch,
        Sedan,
        SUV
    }

    // The function takes a CarType enum as an argument and prints the size of the car based on the
    fn print_size(car:CarType) {
        match car {
            CarType::Hatch => {
                println!("Small sized car");
            },
            CarType::Sedan => {
                println!("medium sized car");
            },
            CarType::SUV =>{
                println!("Large sized Sports Utility car");
            }
        }
    }

    // Match & enum with data type example.
    #[derive(Debug)]
    enum GenderCategoryNew {
        Name(String),
        UsrId(i32)
    }
}