/**
 * In this module we will go through few examples and explain loops in rust lang.
 * For Loop. (line 12)
 * While Loop. (line 22)
 * Loop keyword use example. (line 30)
 * Continue statement. (line 40)
 */
pub mod module {
    pub fn loops() {
        // Definite Loop

        // Using For Loop example.
        for x in 1..11 { // 11 is not inclusive
            if x == 5 {
                continue;
            }
            println!("x is {}",x);
        }

        // Indefinite Loops.

        // While loop example.
        let mut y = 0;
        while y < 10 {
            y += 1;
            println!("inside loop x value is {}",y);
        }
        println!("outside loop x value is {}",y);

        // Loop example.
        let mut z = 0;
        loop {
            z += 1;
            println!("z={}",z);
            if z == 15 {
                break;
            }
        }

        // Using continue statement.
        let mut count = 0;
        for num in 0..21 {
            if num % 2==0 {
                continue;
            }
            count+=1;
        }
        println! (" The count of odd values between 0 and 20 is: {} ",count);
    }
}