/**
 * In this module we will go through few examples and explain Input and output in rust lang.
 * Read example. (line 11)
 * Write example. (line 18)
 * CommandLine arguments example. (line 23)
 * Example sum of values passed as commandline arguments. (line 31)
 */
pub mod module {
    use std::io::Write;
    pub fn input_output() {
        // Read example.
        let mut line = String::new();
        println!("Enter your name :");
        let b1 = std::io::stdin().read_line(&mut line).unwrap();
        println!("Hello , {}", line);
        println!("no of bytes read , {}", b1);

        // Write example. use (line )
        let b2 = std::io::stdout().write("Rust ".as_bytes()).unwrap();
        let b3 = std::io::stdout().write(String::from("Help").as_bytes()).unwrap();
        std::io::stdout().write(format!("\nbytes written {}",(b2+b3)).as_bytes()).unwrap();

        // CommandLine arguments
        let cmd_line = std::env::args();
        println!("No of elements in arguments is :{}",cmd_line.len());
        //print total number of values passed
        for arg in cmd_line {
            println!("[{}]",arg); //print all values passed as commandline arguments
        }

        // Example sum of values passed as commandline arguments.
        let cmd_line1 = std::env::args();
        println!("No of elements in arguments is :{}",cmd_line1.len());
        // total number of elements passed
        let mut sum = 0;
        let mut has_read_first_arg = false;
        //iterate through all the arguments and calculate their sum
        for arg in cmd_line1 {
            if has_read_first_arg { //skip the first argument since it is the exe file name
                sum += arg.parse::<i32>().unwrap();
            }
            has_read_first_arg = true;
            // set the flag to true to calculate sum for the subsequent arguments.
        }
        println!("sum is {}",sum);
    }
}