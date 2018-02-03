#![feature(vec_remove_item)]

extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate rand;
extern crate rayon;

mod roll;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use structopt::StructOpt;
use roll::*;

fn get_word_list() -> Result<Vec<String>, std::io::Error> {
    let mut contents = String::new();

    let mut path = std::env::current_exe().unwrap().parent().unwrap().to_owned();
    path.push("words.txt");

    // Open and read the "words.txt" file
    BufReader::new(File::open(path)?).read_to_string(&mut contents)?;

    Ok(contents.split("\n").map(|s| s.to_string()).collect())
}

fn digit_could_match_char(digit: char, c: char) -> bool {
    match digit {
        '1' => match c {
            'q' | 'a' | 'z' => true,
            _ => false,
        },
        '2' => match c {
            'w' | 's' | 'x' => true,
            _ => false,
        },
        '3' => match c {
            'e' | 'd' | 'c' => true,
            _ => false,
        },
        '4' => match c {
            'r' | 'f' | 'v' => true,
            _ => false,
        },
        '5' => match c {
            't' | 'g' | 'b' => true,
            _ => false,
        },
        '6' => match c {
            'y' | 'h' | 'n' => true,
            _ => false,
        },
        '7' => match c {
            'u' | 'j' | 'm' => true,
            _ => false,
        },
        '8' => match c {
            'i' | 'k' => true,
            _ => false,
        },
        '9' => match c {
            'o' | 'l' => true,
            _ => false,
        },
        '0' => match c {
            'p' => true,
            _ => false,
        },
        _ => panic!("That's not a digit."),
    }
}

fn solve(word_list: &[String], crypt: &str, key_data: (bool, u32)) -> String {
    let mut solution = String::new();

    let mut possibilites = Vec::<&String>::new();
    let mut gathered_initial_possibilities = false;

    // Obtain the crypt if it is keyed or not
    let chars = if key_data.0 {
        // Use the key
        unroll_crypt(crypt.to_owned(), key_data.1)
    } else {
        // Don't use the key
        crypt.to_owned()
    };
    // Get the characters of the crypt
    let chars = chars.chars().enumerate();
    
    for (i, c) in chars {
        if !gathered_initial_possibilities {
            // We collect our initial list of possibilities
            for word in word_list {
                if digit_could_match_char(c, word.chars().nth(0).expect("Please make sure there are no empty lines in `words.txt`.")) {
                    possibilites.push(&word);
                }
            }
            gathered_initial_possibilities = true;
        } else {
            // Process of elimination
            for possibility in possibilites.clone() {
                match possibility.chars().nth(i) {
                    Some(word_c) => if !digit_could_match_char(c, word_c) {
                        possibilites.remove_item(&possibility);
                    },
                    None => {
                        possibilites.remove_item(&possibility);
                    }
                }
            }
        }
    }

    /* If we had two remaining possibilities: "modular" and "modularity",
     * and our crypt could only match "modular", then we choose it.
     *
     * This also works if we just had one possibility in possibilities.
     */

    if !possibilites.is_empty() {
        for possibility in possibilites.clone() {
            if (solution.len() > possibility.len() && possibility.len() >= crypt.len())
                || solution.is_empty()
            {
                solution = possibility.clone();
            }
        }
    } else {
        // We never got a possibility, so we make it all question marks
        for _ in crypt.chars() {
            solution.push('?');
        }
    }

    //eprintln!("{:?}", possibilites);

    solution
}

fn decode(input: String, key_option: Option<u32>) -> String {
    let mut output = String::new();

    let words = get_word_list().expect("The `words.txt` file could not be found in the same directory as this executable.");
    
    let mut current_crypt = String::new();
    // "63999, {12}" evaluates to "hello, 12"
    let mut escape_characters = false;

    let key_data: (bool, u32) = match key_option {
        Some(k) => (true, k),
        None => (false, 0),
    };

    let chars = input.chars();
    for c in chars {
        if c.is_numeric() && !escape_characters {
            current_crypt.push(c);
        } else {
            match c {
                '{' => if !escape_characters {
                    if !current_crypt.is_empty() {
                        // Solve the crypt
                        rayon::scope(|s| s.spawn(|_| output.push_str(solve(&words, current_crypt.as_str(), key_data).as_str())));
                        //output.push_str(solve(&words, current_crypt.as_str(), key_data).as_str());
                        // Reset the current crypt
                        current_crypt = String::new();
                    }
                    escape_characters = true;
                } else {
                    panic!("A '{{' was already found, then another was found before the closing '}}'.");
                },
                '}' => if escape_characters {
                    escape_characters = false;
                },
                _ => {
                    if !current_crypt.is_empty() {
                        // Solve the crypt
                        rayon::scope(|s| s.spawn(|_| output.push_str(solve(&words, current_crypt.as_str(), key_data).as_str())));
                        //output.push_str(solve(&words, current_crypt.as_str(), key_data).as_str());
                        // Reset the current crypt
                        current_crypt = String::new();
                    }
                    // Add whatever we didn't recognize to the output string
                    output.push(c);
                }
            }
            
        }
    }

    if !current_crypt.is_empty() {
        
        output.push_str(solve(&words, current_crypt.as_str(), key_data).as_str());
    }

    output
}

fn encode(input: String, key_option: Option<u32>) -> String {
    let mut output = String::new();

    let mut escape_characters = false;

    let mut key = 0u32;
    let use_key = match key_option {
        Some(k) => {
            key = k;
            true
        },
        None => false,
    };

    let chars = input.chars();
    for c in chars {
        match c {
            '{' => {
                output.push('{');
                escape_characters = true;
            },
            '}' => {
                output.push('}');
                escape_characters = false;
            },
            _ => if !escape_characters && c.is_alphabetic() {
                match c.to_lowercase().next().unwrap() {
                    'q' | 'a' | 'z' => if use_key { output.push(roll_digit('1', key)); } else { output.push('1'); },
                    'w' | 's' | 'x' => if use_key { output.push(roll_digit('2', key)); } else { output.push('2'); },
                    'e' | 'd' | 'c' => if use_key { output.push(roll_digit('3', key)); } else { output.push('3'); },
                    'r' | 'f' | 'v' => if use_key { output.push(roll_digit('4', key)); } else { output.push('4'); },
                    't' | 'g' | 'b' => if use_key { output.push(roll_digit('5', key)); } else { output.push('5'); },
                    'y' | 'h' | 'n' => if use_key { output.push(roll_digit('6', key)); } else { output.push('6'); },
                    'u' | 'j' | 'm' => if use_key { output.push(roll_digit('7', key)); } else { output.push('7'); },
                    'i' | 'k' => if use_key { output.push(roll_digit('8', key)); } else { output.push('8'); },
                    'o' | 'l' => if use_key { output.push(roll_digit('9', key)); } else { output.push('9'); },
                    'p' => if use_key { output.push(roll_digit('0', key)); } else { output.push('0'); },
                    _ => unimplemented!(),
                }
            } else {
                output.push(c);
            },
        }
    }
    output
}

#[derive(StructOpt)]
#[structopt(name = "keycrypto", about = "The digit-based cipher cracker and encoder.")]
enum Options {
    #[structopt(name = "e", help = "Encrypt a message of characters into digits.", about = "Encrypt a message of characters into digits")]
    Encrypt {
        #[structopt(help = "The message to encrypt (Can use quotation marks)")]
        message: String,
        #[structopt(short = "k", long = "key", help = "Add a key to encrypt with")]
        key: Option<u32>,
    },

    #[structopt(name = "d", help = "Decrypt a message of digits into characters.", about = "Decrypt a message of digits into characters")]
    Decrypt {
        #[structopt(help = "The message to decrypt (Can use quotation marks)")]
        message: String,
        #[structopt(short = "k", long = "key", help = "Add a key to decrypt with")]
        key: Option<u32>,
    },

    #[structopt(name = "key", help = "Generate a new key.", about = "Generate a new key")]
    Key {

    },
}

fn main() {
    let options = Options::from_args();
    
    match options {
        Options::Encrypt { message, key } => println!("{}", encode(message, key)),
        Options::Decrypt { message, key } => println!("{}", decode(message, key)),
        Options::Key { } => {
            println!("{}", generate_key());
            println!("If the key is a power of ten (e.g. 1, 10, 100, 1000, 10000, etc.),");
            println!("then please generate a new one. Powers of ten do not encrypt.");
        },
    }
}
