/**
 * In this module we will go through few examples and explain iterator and closure in rust lang.
 * Iterator example. (line 12)
 * Example iter(); (line 23)
 * Example into_iter(); (line 33)
 * Example iter_mut(); (line 42)
 * Closure explained. (line 52)
 * Closure example. (line 60)
 */
pub mod module {
    pub fn iterator_closure() {
        // Iterator example.
        let a = [10,20,30];
        let mut iter = a.iter();
        // Fetch an iterator object for the array.
        println!("{:?}",iter);
        // Fetch individual values from the iterator object.
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());
        println!("{:?}",iter.next());

        // example iter();
        let names = vec!["Joe", "Jack", "Karen"];
        for name in names.iter() {
            match name {
                &"Jack" => println!("There is a Jack among us!"),
                _ => println!("Hello {}", name),
            }
        }
        println!("{:?}",names);

        // Example into_iter();
        let names1 = vec!["Joe", "Jack", "Karen"];
        for name in names1.into_iter() {
            match name {
                "Jack" => println!("There is a Jack among us!"),
                _ => println!("Hello {}", name),
            }
        }

        // Example iter_mut();
        let mut names2 = vec!["Joe", "Jack", "Karen"];
        for name in names2.iter_mut() {
            match name {
                &mut "Jack" => println!("There is a Jack among us!"),
                _ => println!("Hello {}", name),
            }
        }
        println!("{:?}",names2);

        // Closure explained.
        // Closure refers to a function within another function.
        // These are anonymous functions – functions without a name.
        // Closure can be used to assign a function to a variable.
        // This allows a program to pass a function as a parameter to other functions.
        // Closure is also known as an inline function.
        // Variables in the outer function can be accessed by inline functions.

        // Closure example.
        let is_even = |x| {
            x%2==0
        };
        let no = 13;
        println!("{} is even ? {}",no,is_even(no));
    }
}