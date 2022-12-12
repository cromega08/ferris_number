use ferris_says::*;
use emojis::get_by_shortcode;
use rand::Rng;
use std::collections::HashMap;
use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {

    let to_guess = rand::thread_rng().gen_range(1..=100);

    let mut emojis:HashMap<&str, &str> = HashMap::new();
    emojis.insert(
        "thinking",
        get_by_shortcode("thinking").unwrap().as_str()
    );
    emojis.insert(
        "smile",
        get_by_shortcode("smile").unwrap().as_str()
    );
    emojis.insert(
        "dizzy_face",
        get_by_shortcode("dizzy_face").unwrap().as_str()
    );
    emojis.insert(
        "melting_face",
        get_by_shortcode("melting_face").unwrap().as_str()
    );
    emojis.insert(
        "no_mouth",
        get_by_shortcode("no_mouth").unwrap().as_str()
    );
    emojis.insert(
        "slightly_smiling_face",
        get_by_shortcode("slightly_smiling_face").unwrap().as_str()
    );
    emojis.insert(
        "upside_down_face",
        get_by_shortcode("upside_down_face").unwrap().as_str()
    );
    emojis.insert(
        "star_struck",
        get_by_shortcode("star_struck").unwrap().as_str()
    );
    emojis.insert(
        "monocle_face",
        get_by_shortcode("monocle_face").unwrap().as_str()
    );

    let mut ferris_lines:HashMap<&str, String> = HashMap::new();
    ferris_lines.insert(
        "greeting",
        "The Ferris Number!".to_owned()
        + "\n\nWill you be able to guess my lucky number? " + emojis.get("thinking").unwrap()
        + "\n\nPress enter to start"
    );
    ferris_lines.insert(
        "start",
        "Great! ".to_owned() + emojis.get("star_struck").unwrap()
        + "\n\nWhat number am I thinking of? " + emojis.get("monocle_face").unwrap()
    );
    ferris_lines.insert(
        "equals",
        "Your answer was right! ".to_owned() + emojis.get("dizzy_face").unwrap()
        + "\n\nI was thinking in that number!" + emojis.get("smile").unwrap()
    );
    ferris_lines.insert(
        "higher",
        "Your answer was wrong! ".to_owned() + emojis.get("melting_face").unwrap()
        + "\n\nYour number was higher " + emojis.get("slightly_smiling_face").unwrap()
    );
    ferris_lines.insert(
        "lower",
        "Your answer was wrong! ".to_owned() + emojis.get("melting_face").unwrap()
        + "\n\nYour number was lower " + emojis.get("upside_down_face").unwrap()
    );
    ferris_lines.insert(
        "error",
        "Only numbers are allowed... ".to_owned() + emojis.get("no_mouth").unwrap()
    );

    let mut guess: String;
    let mut number:u32;

    ferris_speak(ferris_lines.get("greeting"));

    get_input("");

    ferris_speak(ferris_lines.get("start"));

    loop {
        guess = get_input("\nYour answer: ");

        if guess.eq("exit") || guess.eq("quit") {break;}

        number = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                ferris_speak(ferris_lines.get("error"));
                continue;
            }
        };

        match number.cmp(&to_guess) {
            Ordering::Less => ferris_speak(ferris_lines.get("lower")),
            Ordering::Equal => {
                ferris_speak(ferris_lines.get("equals"));
                println!("\n-> {number} was the right answer!");
                break;
            },
            Ordering::Greater => ferris_speak(ferris_lines.get("higher"))
        }    
    }
}

fn ferris_speak(line: Option<&String>) {
    clear();
    say(line.unwrap(), 40, io::stdout())
        .expect("Error showing Ferris");
}

fn get_input(to_print: &str) -> String {
    print!("{to_print}"); io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
            .read_line(&mut input)
            .expect("Error reading user input");
    return input;
}

fn clear() {print!("{esc}c", esc = 27 as char);}
