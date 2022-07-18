/**
 * In this module we will go through few examples and explain array in rust lang.
 * Example of array syntax. (line 15)
 * Array without data type. (line 20)
 * Array with default values. (line 25)
 * Array with for loop. (line 30)
 * Array with iter() function. (line 38)
 * Mutable array. (line 46)
 * Passing array as parameters. (line 51)
 * Passing array as reference. (line 56)
 * Array declaration and constants. (line 61)
 */
pub mod module {
    pub fn arrays() {
        // Example of array.
        let arr1:[i32;4] = [10,20,30,40];
        println!("array is {:?}",arr1);
        println!("array size is :{}",arr1.len());

        // Array without data type.
        let arr2 = [10,20,30,40];
        println!("array is {:?}",arr2);
        println!("array size is :{}",arr2.len());

        // Array with default values.
        let arr3:[i32;4] = [-1;4];
        println!("array is {:?}",arr3);
        println!("array size is :{}",arr3.len());

        // Array with for loop.
        let arr4:[i32;4] = [10,20,30,40];
        println!("array is {:?}",arr4);
        println!("array size is :{}",arr4.len());
        for index in 0..4 {
            println!("index is: {} & value is : {}",index,arr4[index]);
        }

        // Array with iter() function.
        let arr5:[i32;4] = [10,20,30,40];
        println!("array is {:?}",arr5);
        println!("array size is :{}",arr5.len());
        for val in arr5.iter(){
            println!("value is :{}",val);
        }

        // Mutable array.
        let mut arr6:[i32;4] = [10,20,30,40];
        arr6[1] = 0;
        println!("{:?}",arr6);

        // Passing array as parameters.
        let arr7 = [10,20,30];
        update_par(arr7); // (line 77)
        print!("Inside main {:?}",arr7);

        // Passing array as reference.
        let mut arr8 = [10,20,30];
        update_ref(&mut arr8); // (line 85)
        print!("Inside main {:?}",arr8);

        // Array declaration and constants.
        // let N: usize = 20;
        // let arr9 = [0; N]; //Error: non-constant used with constant
        // print!("{}",arr9[10])
        // The compiler will result in an exception.
        // This is because an array's length must be known at compile time.
        // Here, the value of the variable "N" will be determined at runtime.
        // In other words, variables cannot be used to define the size of an array.
        const N: usize = 20;
        // pointer sized
        let arr9 = [0; N];
        print!("{}",arr9[10])
        // The value of an identifier prefixed with the const keyword is defined at compile time and cannot be changed at runtime.
        // usize is pointer-sized, thus its actual size depends on the architecture you are compiling your program for.
    }

    // Passing array as parameter.
    fn update_par(mut arr:[i32;3]){
        for i in 0..3 {
            arr[i] = 0;
        }
        println!("Inside update {:?}",arr);
    }

    // Passing array as reference.
    fn update_ref(arr:&mut [i32;3]){
        for i in 0..3 {
            arr[i] = 0;
        }
        println!("Inside update {:?}",arr);
    }
}