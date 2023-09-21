extern crate clap;
extern crate rand;
extern crate clipboard;

use clap::{App, Arg};
use rand::Rng;
use rand::distributions::Slice;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    // Define command-line arguments and options using clap
    let matches = App::new("Password Generator")
        .version("1.0")
        .author("Your Name")
        .about("Generates random passwords.")
        .arg(
            Arg::with_name("length")
                .short("l")
                .long("length")
                .value_name("LENGTH")
                .help("Sets the length of the password")
                .required(false)
                .takes_value(true)
                .default_value("12"),
                
        )
        .arg(
            Arg::with_name("uppercase")
                .short("u")
                .long("uppercase")
                .help("Include uppercase letters in the password"),
        )
        .arg(
            Arg::with_name("numbers")
                .short("n")
                .long("numbers")
                .help("Include numbers in the password"),
        )
        .arg(
            Arg::with_name("special")
                .short("s")
                .long("special")
                .help("Include special characters in the password"),
        )
        .arg(
            Arg::with_name("no clipboard")
                .short("c")
                .long("no-clipboard")
                .help("do not copy password to clipboard"),
        )
        .get_matches();

    // Parse command-line arguments
    let password_length: usize = matches.value_of("length").unwrap().parse().expect("Invalid length");
    let use_uppercase = matches.is_present("uppercase");
    let use_numbers = matches.is_present("numbers");
    let use_special = matches.is_present("special");
    let no_clipboard = matches.is_present("no clipboard");
    
    // Define character sets
    let lowercase_chars: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let uppercase_chars: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let numeric_chars: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let special_chars: [char; 17] = ['!', '@', '#', '$', '%', '&', '^', '*', '(', ')', '-', '_', '+', '=', '<', '>', '?'];

    let mut available_chars: Vec<char> = lowercase_chars.to_vec();

    if use_uppercase {
        available_chars.extend(uppercase_chars.iter());
    }
    if use_numbers {
        available_chars.extend(numeric_chars.iter());
    }
    if use_special {
        available_chars.extend(special_chars.iter());
    }

    let available_chars_dist = Slice::new(&available_chars).unwrap();

    // Generate the password
    let password: String = rand::thread_rng()
        .sample_iter(&available_chars_dist)
        .take(password_length)
        .collect();

    // Print the generated password
    println!("Generated Password: {}", password);

    // Copy the password to the clipboard
    if !no_clipboard {
        let mut ctx: ClipboardContext = ClipboardProvider::new().expect("Error initializing clipboard");
        ctx.set_contents(password).expect("Error copying password to clipboard");
    }
}
