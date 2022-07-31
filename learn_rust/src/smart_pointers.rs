/**
 * In this module we will go through few examples and explain smart pointers in rust lang.
 * Smart pointers explained. (line 12)
 * Box explained and example. (line 17)
 * Dereferencing example. (line 28)
 * Deref trait explained and example. (line 37)
 * Drop trait explained and example. (line 51)
 */
pub mod module {
    use std::ops::Deref;
    pub fn smart_pointers() {
        // Smart pointers explained.
        // Rust allocates everything on the stack by default.
        // You can store things on the heap by wrapping them in smart pointers like Box.
        // Types like Vec and String implicitly help heap allocation.

        // Box example.
        // The Box smart pointer also called a box allows you to store data on the heap rather than the stack.
        // The stack contains the pointer to the heap data.
        // A Box does not have performance overhead, other than storing their data on the heap.
        // Let us see how to use a box to store an i32 value on the heap.
        let var_i32 = 5;
        //stack
        let b = Box::new(var_i32);
        //heap
        println!("b = {}", b);

        // Dereferencing example.
        let x = 5;
        // Value type variable.
        let y = Box::new(x);
        // y points to a new value 5 in the heap.
        println!("{}",5==x);
        println!("{}",5==*y);
        // Dereferencing y.

        // Deref trait example. (line 61)
        // The Deref trait, provided by the standard library, requires us to implement one method named deref,
        // that borrows self and returns a reference to the inner data.
        // The following example creates a structure MyBox, which is a generic type.
        // It implements the trait Deref. This trait helps us access heap values wrapped by j using *j.
        let h = 5;
        let j = MyBox::new(h);
        // Calling static method.
        println!("5==h is {}",5==h);
        println!("5==*j is {}",5==*j);
        // Dereferencing j.
        println!("h==*j is {}",h==*j);
        // Dereferencing j.

        // Drop trait example. (line 76)
        // The Drop trait contains the drop() method.
        // This method is called when a structure that implemented this trait goes out of scope.
        // In some languages, the programmer must call code to free memory or resources every time they finish using an instance of a smart pointer.
        // In Rust, you can achieve automatic memory deallocation using Drop trait.
        let s = 50;
        MyNewBox::new(s);
        MyNewBox::new("Hello");
    }

    // Deref trait example.
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        // Generic structure with static method new
        fn new(h:T)-> MyBox<T> {
            MyBox(h)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0 //returns data
        }
    }

    // Drop trait example.
    struct MyNewBox<T>(T);
    impl<T> MyNewBox<T> {
        fn new(s:T)->MyNewBox<T>{
            MyNewBox(s)
        }
    }
    impl<T> Deref for MyNewBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
        &self.0
        }
    }
    impl<T> Drop for MyNewBox<T>{
        fn drop(&mut self){
            println!("dropping MyBox object from memory ");
        }
    }
}