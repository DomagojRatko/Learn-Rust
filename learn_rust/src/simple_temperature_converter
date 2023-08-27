// Easy example for begginers.

use std::io;
use crate::Scales::{Celsius, Fahrenheit};

enum Scales {
    Fahrenheit,
    Celsius,
}

impl ToString for Scales {
    fn to_string(&self) -> String {
        match self {
            Fahrenheit => String::from("Fahrenheit"),
            Celsius => String::from("Celsius")
        }
    }
}

fn main() {

    let scale: Scales;

    // User input select Fahrenheit or Celsius
    println!("Please choose. Press (f) Fahrenheit or (c) Celsius to convert.");
    loop {
        let mut temp_scale = String::new();
        io::stdin()
            .read_line(&mut temp_scale)
            .expect("Failed to read line");

        if temp_scale.trim() == "f" {
            scale = Fahrenheit;
            break;
        } else if temp_scale.trim() == "c" {
            scale = Celsius;
            break;
        } else {
            println!("Wrong input. Please select (f) Fahrenheit or (c) Celsius");
            continue;
        }
    }

    // User input number to convert to selected Fahrenheit or Celsius
    println!("Input number to convert {} :", scale.to_string());
    loop {
        let mut temp_degrees = String::new();
        io::stdin()
            .read_line(&mut temp_degrees)
            .expect("Failed to read line");

        match temp_degrees.trim().parse() {
            Ok(number) => {
                // Print result.
                print_result(scale, number);
                break;
            },
            Err(_) => {
                println!("Wrong input. Please input number!");
                continue;
            },
        };
    }

}

fn print_result(scale: Scales, number: f32) {
    if scale.to_string() == "Fahrenheit" {
        println!("Result: {}°C", convert_to_celsius(number));
    } else if scale.to_string() == "Celsius" {
        println!("Result: {}°F", convert_to_fahrenheit(number));
    }
}

fn convert_to_celsius(number:f32) -> f32 {
    (number - 32.0) / 1.8
}

fn convert_to_fahrenheit(number:f32) -> f32 {
    (number * 1.8) + 32.0
}
