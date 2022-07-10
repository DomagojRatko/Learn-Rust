/**
 * In this module we will go through few examples and explain decision making in rust lang.
 * How to use decision making. (line 7)
 * How to use match statement. (line 18)
 */
pub mod module {
    pub fn decision_making() {
        // Simple if, else if and else statement example.
        let num = 2 ;
        if num > 0 {
            println!("{} is positive",num);
        } else if num < 0 {
            println!("{} is negative",num);
        } else {
            println!("{} is neither positive nor negative",num) ;
        }

        // Match statement example.
        // The match statement checks if a current value is matching from a list of values,
        // this is very much similar to the switch statement in C language.
        let state_code = "NY";
        let state = match state_code {
            "NY" => {println!("Found match for NY"); "New York"},
            "LA" => "Los Angeles",
            "HK" => "Hong Kong",
            _ => "Unknown"
        };
        println!("State name is {}",state);
    }
}