/**
 * In this module we will go through few examples and explain string types in rust lang.
 * String Literal. (line 24)
 * String Object. (line 31)
 * What is String::new(); (line 40)
 * What is String::from(); (line 45)
 * What are common methods used. (line 49)
 * Assigning value to String::new(); (line 84)
 * Using replace(); (line 89)
 * Using to_string(); (line 95)
 * Using as_str(); (line 102)
 * Using push(); (line 114)
 * Using push_str(); (line 120)
 * Using len(); (line 126)
 * Using trim(); (line 131)
 * Using split_whitespace(); (line 138)
 * Using split(); (line 148)
 * Using chars(); (line 162)
 * How to concatenation of strings. (line 169)
 * How to use Format macro. (line 178)
 */
pub mod module {
    pub fn string() {
        // String Literal(&str)
        let name:&str = "Rust tutorials";
        let message:&'static str = "some random text";
        println!("name is : {} message :{}",name,message);
        // NOTE: String literals are static by default.
        // This means that string literals are guaranteed to be valid for the duration of the entire program.

        // String Object(String)
        /* The String object type is provided in Standard Library.
        Unlike string literal, the string object type is not a part of the core language.
        It is defined as public structure in standard library pub struct String.
        String is a growable collection. It is mutable and UTF-8 encoded type.
        The String object type can be used to represent string values that are provided at runtime.
        String object is allocated in the heap.
         */

        // To create a String object.
        // String::new(); must be empty.
        let empty_string = String::new();
        println!("length is {}",empty_string.len());

        // String::from("value"); must have some value.
        let content_string = String::from("value");
        println!("length is {}",content_string.len());

        /* Common Methods - String Object
            1	new()	pub const fn new() → String
            Creates a new empty String.

            2	to_string()	fn to_string(&self) → String
            Converts the given value to a String.

            3	replace()	pub fn replace<'a, P>(&'a self, from: P, to: &str) → String
            Replaces all matches of a pattern with another string.

            4	as_str()	pub fn as_str(&self) → &str
            Extracts a string slice containing the entire string.

            5	push()	pub fn push(&mut self, ch: char)
            Appends the given char to the end of this String.

            6	push_str()	pub fn push_str(&mut self, string: &str)
            Appends a given string slice onto the end of this String.

            7	len()	pub fn len(&self) → usize
            Returns the length of this String, in bytes.

            8	trim()	pub fn trim(&self) → &str
            Returns a string slice with leading and trailing whitespace removed.

            9	split_whitespace()	pub fn split_whitespace(&self) → SplitWhitespace
            Splits a string slice by whitespace and returns an iterator.

            10	split()	pub fn split<'a, P>(&'a self, pat: P) → Split<'a, P> , where P is pattern can be &str, char, or a closure that determines the split.
            Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.

            11	chars()	pub fn chars(&self) → Chars
            Returns an iterator over the chars of a string slice.
         */

        // Using new(); and setting mut to assign value.
        let mut word = String::new();
        word.push_str("hello!");
        println!("{}", word);

        // Using to_string();
        // To access all methods of String object, convert a string literal to object type using the to_string() function.
        let number = 1234;
        let string = number.to_string();
        println!("{}",string);

        // Using replace();
        // The replace() function takes two parameters.
        // the first parameter is a string pattern to search for and the second parameter is the new value to be replaced.
        let word1 = "Hello friend!";
        let word2 = word1.replace("Hello","Howdy");
        println!("{}",word2);

        // Using as_str();
        // The as_str() function extracts a string slice containing the entire string.
        /*
        fn main() {
            let example_string = String::from("example_string");
            print_literal(example_string.as_str());
        }
        fn print_literal(data:&str ){
            println!("displaying string literal {}",data);
        }
         */

        // Using push();
        // The push() function appends the given char to the end of this String.
        let mut text = "Word".to_string();
        text.push('s');
        println!("{}",text);

        // Using push_str();
        // The push_str() function appends a given string slice onto the end of a String.
        let mut text1 = "words".to_string();
        text1.push_str("Big ");
        println!("{}",text1);

        // Using len();
        // The len() function returns the total number of characters in a string (including spaces).
        let count = "How many characters do we have here?";
        println!("length is {}",count.len());

        // Using trim();
        // The trim() function removes leading and trailing spaces in a string.
        // NOTE that this function will not remove the inline spaces.
        let some_text = " How many leading and trailing spaces do we have here? \r\n";
        println!("Before trim length is {}",some_text.len());
        println!("After trim length is {}",some_text.trim().len());

        // Using split_whitespace();
        // The split_whitespace() splits the input string into different strings.
        // It returns an iterator so we are iterating through the tokens
        let msg = "Some random text.".to_string();
        let mut i = 1;
        for words in msg.split_whitespace(){
            println!("word {} {}",i,words);
            i+=1;
        }

        // Using split();
        // The split() string method returns an iterator over substrings of a string slice, separated by characters matched by a pattern.
        // The limitation of the split() method is that the result cannot be stored for later use.
        // The collect method can be used to store the result returned by split() as a vector.
        let some_names = "Joe,Candice,Lola";
        for name in some_names.split(","){
            println!("name is {}",name);
        }
        // Store in a Vector example.
        let words:Vec<&str>= some_names.split(",").collect();
        println!("first person is {}",words[0]);
        println!("second person is {}",words[1]);
        println!("third person is {}",words[2]);

        // Using chars();
        // Individual characters in a string can be accessed using the chars method.
        let some_string = "RustTutorials".to_string();
        for c in some_string.chars(){
            print!("{} ",c);
        }

        // Concatenation of Strings with + operator or add function.
        // add(self,&str)->String {
        // returns a String object
        // }
        let w1 = "Rust".to_string();
        let w2 = "Tutorials".to_string();
        let w3 = w1 + &w2; // w2 reference is passed
        println!("\n {}",w3);

        // Format macro.
        // Another way to add to String objects together is using a macro function called format.
        let s1 = "Rust".to_string();
        let s2 = "tutorials".to_string();
        let s3 = format!("{} {}",s1,s2);
        println!("{}",s3);
    }
}