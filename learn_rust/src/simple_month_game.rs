// Notice add this dependencies to your Cargo.toml
// rand = "0.8.5"
// ansi_term = "0.12"

extern crate ansi_term;

use ansi_term::Color;
use rand::Rng;
use std::io;
use std::time::Instant;

const NUMBER_OF_ROUNDS: i32 = 10;

struct GameStats {
    start_time: Instant,
    end_time: Instant,
    score: i32,
}

fn main() {
    println!("{}", Color::Yellow.paint("GUESS THE MONTH GAME!"));
    println!("{}", Color::RGB(220, 220, 220)
        .paint("Enter a number from 1 to 12 to guess."));

    let game_stats = game_loop();

    print_stats(&game_stats);
}

fn game_loop() -> GameStats {
    let mut rng = rand::thread_rng();
    let mut rounds = NUMBER_OF_ROUNDS;
    let mut score = 0;

    let months = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];

    let start_time = Instant::now();

    while rounds > 0 {
        let random_number = rng.gen_range(1..=12);
        let month = months[random_number as usize - 1];

        println!(
            "{}{}",
            Color::Yellow.paint("What month is "),
            Color::Green.paint(month)
        );

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_guess = user_input.trim();
        if let Ok(guess) = user_guess.parse::<i32>() {
            if (1..=12).contains(&guess) {
                if guess == random_number {
                    println!("{}", Color::Green.paint("Correct!"));
                    score += 1;
                    rounds -= 1;
                } else {
                    println!("{}", Color::Red.paint("Incorrect!"));
                    rounds -= 1;
                }
            } else {
                println!("{}", Color::Red.paint("Please use a number from 1 to 12!"));
            }
        } else {
            println!("{}", Color::Red.paint("Invalid input. Please enter a number."));
        }
    }

    let end_time = Instant::now();

    GameStats {
        start_time,
        end_time,
        score,
    }
}

fn print_stats(stats: &GameStats) {
    println!();
    println!("{}", Color::Yellow.paint("STATS:"));

    println!("{}{}", Color::Yellow.paint("Rounds: "),
             Color::Green.paint(NUMBER_OF_ROUNDS.to_string()));

    println!("{}{}", Color::Yellow.paint("Score: "),
             Color::Green.paint((stats.score * 10).to_string()));

    println!("{}{}", Color::Yellow.paint("Correct Answers: "),
             Color::Green.paint(stats.score.to_string()));

    println!("{}{}", Color::Yellow.paint("Wrong Answers: "),
             Color::Red.paint((NUMBER_OF_ROUNDS - stats.score).to_string()));

    let correct_percent = (stats.score * 100) / NUMBER_OF_ROUNDS;
    println!(
        "{}{}%",
        Color::Yellow.paint("Correct percentage: "),
        Color::Green.paint(correct_percent.to_string())
    );

    let elapsed_time = stats.end_time.duration_since(stats.start_time);
    let elapsed_secs = elapsed_time.as_secs();
    println!(
        "{}{}sec",
        Color::Yellow.paint("Time: "),
        Color::Green.paint(elapsed_secs.to_string())
    );
}
