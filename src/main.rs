extern crate passwords;
extern crate question;

use std::io;
use passwords::PasswordGenerator;
use question::{Answer, Question};

fn main() {
    loop {
        println!("Please input your desired password length: ");
        let mut password_length = String::new();
        io::stdin().read_line(&mut password_length)
            .expect("Failed to read line");
        let password_length: u8 = match password_length.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let use_lowercase: bool;
        let answer = Question::new("Do you want to include lower case characters?").default(Answer::YES).show_defaults().confirm();
        if answer == Answer::YES {
            use_lowercase = true;
        }
        else {
            use_lowercase = false;
        }

        let use_uppercase: bool;
        let answer = Question::new("Do you want to include upper case characters?").default(Answer::YES).show_defaults().confirm();
        if answer == Answer::YES {
            use_uppercase = true;
        }
        else {
            use_uppercase = false;
        }

        let use_numbers: bool;
        let answer = Question::new("Do you want to include numbers?").default(Answer::YES).show_defaults().confirm();
        if answer == Answer::YES {
            use_numbers = true;
        }
        else {
            use_numbers = false;
        }

        let use_symbols: bool;
        let answer = Question::new("Do you want to add special characters?").default(Answer::YES).show_defaults().confirm();
        if answer == Answer::YES {
            use_symbols = true;
        }
        else {
            use_symbols = false;
        }

        if use_lowercase == false && use_uppercase == false && use_numbers == false && use_symbols == false {
            println!("Here is your password:");
            println!("You don't generate anything huh...");
            break;
        }

        let p = PasswordGenerator::new().length(password_length.into()).numbers(use_numbers).lowercase_letters(use_lowercase).uppercase_letters(use_uppercase).symbols(use_symbols).spaces(false).exclude_similar_characters(false).strict(true);
        println!("Here is your password: {}", p.generate_one().unwrap());
        break;
    }
}
