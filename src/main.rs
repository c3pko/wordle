extern crate colored;
use std::io::{ self, BufRead, BufReader, Write};
use std::cmp;
use rand::Rng;
use std::cmp::Ordering;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::ops::Not;
use colorized::*;
use colored::*; //https://github.com/mackwic/colored
use std::fs;
use isahc::ReadResponseExt;
use std::path::Path;
// use crate::io::BufReader;
// use crate::fs::File;
use reqwest;
use reqwest::Client;
use tokio::main;
use ansi_colours::*;
use rgb::*;

// #[derive(Copy, Clone)]
struct FiveLetterDictionary {
    dictionary: Vec<String>,
}

// #[derive(Clone)]
struct UserGuess {
    length: usize,
    word: String,
    real_word: bool,
    guessed_wordle: bool,
}

impl UserGuess {
    fn reset_user_guess_struct(guess: &mut UserGuess, word_length: usize, user_word: String, real: bool, guessed_word: bool) {
        guess.length = word_length;
        guess.word = user_word.to_string();
        guess.real_word = real;
        guess.guessed_wordle = guessed_word;
    }
}

impl UserGuess {

    fn get_five_char_word(users_guess_struct: &mut UserGuess, user_guesses_remaining: i32) {

        let mut counter: i32 = 0;
        let mut users_guess = String::new();
        let mut user_needs_to_guess_again = true;

        println!("You have {} guesses remaining. Please enter your 5 letter guess below: ", user_guesses_remaining);
        
        //check that users_guess is a five letter real word in the English dictionary, otherwise keep prompting for a real five letter word
        while user_needs_to_guess_again == true {
            // println!("counter = {:?}", counter);
            if counter < 5 && counter >0 {
                println!("Please enter your 5 letter guess below: ");
            }
            if counter > 5 {
                println!("As a reminder, your guesses must be: \n");
                println!("\t - five letters in length");
                println!("\t - an English word");
                println!("\t - not a proper noun (though words that are both proper and common nouns are fine)\n");
                println!("Please enter your 5 letter guess below: ");
                counter = 0;
            }

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let users_input = input.trim().to_string(); //this is a String

            // println!("Your guess: {:?}", users_input);
            let mut users_input_len = users_input.len();
            // let mut bytes_to_take = cmp::min(users_input.len(),5); //if users guess > 5 letters, only take first 5
            // let mut users_guess = &users_input[..bytes_to_take].to_string(); //slice to get first five letters of guess and convert to String
            FiveLetterDictionary::check_real_word(users_guess_struct, users_input.clone().to_string());
            let mut new_struct = Self::reset_user_guess_struct(users_guess_struct, users_input_len, users_input.to_string(), users_guess_struct.real_word, false);
            if users_guess_struct.length != 5 || users_guess_struct.real_word == false {
                Self::print_error_messages(&users_guess_struct, users_input.to_string());
            }
            else {
                user_needs_to_guess_again = false;
            }
            counter+=1
        }
    }

    fn print_error_messages(users_guess_struct: &UserGuess, users_input: String) {
        
        println!("Your guess '{}' is not valid for the following reason: ", users_input);
        
        if users_guess_struct.length < 5 {
            println!("\t - your guess is under 5 letters.");
        }
        else if users_guess_struct.length > 5 {
            println!("\t -your guess is over 5 letters.");
        }
        else if users_guess_struct.real_word ==false {
            println!("\t -your guess is not a valid five letter English word.");
        }
        println!("\n");
    }

    fn get_five_char_word_second_way(user_guesses_remaining: i32) -> String {

        println!("You have {} guesses remaining. Please enter your 5 letter guess below: ", user_guesses_remaining);
        let mut input = String::new();
        let mut buffer = [0;5];
        let mut buf = vec![0u8; 5];

        io::stdin()
            .read_exact(&mut buffer)
            // .read(&mut buffer)
            .expect("Failed to read line");
        let mut buffer_bytes = buffer.len();

        io::stdin()
            .read_line(&mut input);
        
        // let mut input_bytes = input.len();
        let mut temp_guess = String::from_utf8(buffer.to_vec()).unwrap();
        let users_guess = temp_guess.trim().to_string();
        println!("user's guess: {:?}", users_guess);

        return users_guess
    }

    fn user_wins_message(users_guess_vec:Vec<char>, user_guesses: i32) {

        println!("Comparing your guess to the wordle: ");
        println!("___________");
        for (index, element) in users_guess_vec.iter().enumerate() {
            print!("|");
            let mut users_guess_char: String = String::from(users_guess_vec[index]);
            print!("{}", users_guess_char.color(Colors::BrightGreenBg)); 
        }
        print!("|");
        println!("\n-----------");
        println!("\nGreat job you guessed the wordle in {} guesses!\n", user_guesses);

    }

    fn print_comparison(mut comparison_array: Vec<char>, users_guess_vec: Vec<char>) {

        println!("Comparing your guess to the wordle: ");

        println!("___________");
        for (index, element) in comparison_array.iter_mut().enumerate() {
            let mut users_guess_char: String = String::from(users_guess_vec[index]);
            print!("|");
            if *element == 'G' {
                // users_guess_char.color(Colors::GreenBg);
                // users_guess_char.color(Colors::WhiteFg);
                // print!("{}", users_guess_char.color(Colors::BrightGreenBg, Colors::BrightWhiteFg));
                print!("{}", users_guess_char.black().bold().on_green());
                // print!("{}", users_guess_char.black().green(153,255,153));
            }
            if *element == 'Y' {
                // print!("{}", users_guess_char.color(Colors::BrightYellowBg));
                // print!("{}", users_guess_char.color(Colors::BrightYellowBg, Colors::BrightWhiteFg));
                // println!("This is with color:  {} {}", users_guess_char.color(Colors::WhiteFg),  users_guess_char.color(Colors::YellowBg));
                print!("{}", users_guess_char.black().bold().on_yellow());
            }
            if *element == 'B' {
                // print!("{}", users_guess_char.color(Colors::BrightBlackBg));
                // print!("{}", users_guess_char.color(Colors::BrightBlackBg, Colors::BrightWhiteFg));
                // println!("This is with color:  {} {}", users_guess_char.color(Colors::WhiteFg),  users_guess_char.color(Colors::BlackBg));
                print!("{}", users_guess_char.white().bold().on_black());
            }
        }
        print!("|");
        println!("\n-----------\n");

    }

    fn compare_words(users_guess_struct: &mut UserGuess, wordle_word:String, user_guesses: i32) -> bool {
        let mut users_guess = &users_guess_struct.word;
        let mut user_guessed_word = false;
        
        let users_guess_vec: Vec<char> = users_guess.chars().collect();
        let wordle_word_vec: Vec<char> = wordle_word.chars().collect();
        let initalizing = String::from("BBBBB");
        let mut comparison_array: Vec<char> = initalizing.chars().collect();
        let mut char_counter_hashmap = HashMap::new();
        
        if users_guess_vec ==  wordle_word_vec {
            Self::user_wins_message(users_guess_vec, user_guesses);
            users_guess_struct.guessed_wordle = true;
            user_guessed_word = true;
            return user_guessed_word;
        }

        for (index, element) in wordle_word_vec.iter().enumerate() {
            if wordle_word_vec[index] == users_guess_vec[index] {
                //if match, print in green text
                comparison_array[index] = 'G';
            }
            else {
                //increment or add char to hashmap
                //*char_counter_hashmap.entry(&wordle_word_vec[index]).or_insert(1) +=1;
                match char_counter_hashmap.get(&wordle_word_vec[index]) {
                    Some(count) => { char_counter_hashmap.insert(wordle_word_vec[index], count + 1); }
                    None => { char_counter_hashmap.insert(wordle_word_vec[index], 1); }
                }
            }
        }

        //iterate through non-matched elements and check if user word contains any of those chars
        //if yes, decrement hashmap counter and continue through word

        for (index, element) in comparison_array.iter_mut().enumerate() {
            if *element != 'G' {
                let mut char_to_check = users_guess_vec[index];
                match char_counter_hashmap.get(&users_guess_vec[index]) {
                    Some(&count) => {
                        if count == 1 {
                            char_counter_hashmap.remove(&users_guess_vec[index]);
                            *element = 'Y';
                        }
                        else {
                            char_counter_hashmap.insert(users_guess_vec[index], count-1);
                            *element = 'Y';
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
        }

        Self::print_comparison(comparison_array, users_guess_vec);
        return user_guessed_word;

    }
}

fn hello_prompt() {

    let green = String::from("green");
    let yellow = String::from("yellow");
    let black = String::from("black");

    let example_users_guess = String::from("shear");
    let mut test_struct = UserGuess {
        word: String::from(example_users_guess.clone()),
        length: example_users_guess.len(),
        real_word: true,
        guessed_wordle: false,
    };

    let example_wordle = String::from("where");
    let example_guess_vec: Vec<char> = example_users_guess.chars().collect();
    let example_wordle_vec: Vec<char> = example_wordle.chars().collect();
    let one_guess: i32 = 1;

    println!("Welcome to the poor man's version of wordle: where the words aren't random and the points are all made up!\n");
    println!("You have 6 guesses to guess the 5 letter word of the day. You will get color coded feedback on each guess like so:");
    println!("If you guess the right character in the right place your character will be printed as {}", green.color(Colors::GreenFg));
    println!("If you guess the right character in the wrong place your character will be printed as {}", yellow.color(Colors::YellowFg));
    println!("Otherwise, your character will be printed as {}", black.color(Colors::BlackFg));
    println!("\nFor example, if you guess 'shear' and the wordle is 'where' you will see: ");
    let user_guessed_word = UserGuess::compare_words(&mut test_struct, example_wordle, one_guess);
    println!("This means 'h' and 'e' are correctly placed, 's' and 'a' are not in the wordle, and 'r' is in the wordle but not in the fifth position.");
    println!("Now that you know the rules, let's play!\n\n");

}


impl FiveLetterDictionary {

    fn read_words_from_file(filename: impl AsRef<Path>) -> Vec<String> {
        let file = File::open(filename).expect("no such file");
        let buf = BufReader::new(file);
        buf.lines()
            .map(|l| l.expect("Could not parse line"))
            .collect()    
    }

    fn get_wordle_dictionary() -> Vec<String> {
        let lines = Self::read_words_from_file("dictionary.txt");
        let mut word_dictionary = FiveLetterDictionary {
            dictionary: lines,
        };
        word_dictionary.dictionary
    }

    fn generate_wordle() -> String {
        let dictionary = Self::get_wordle_dictionary();
        // let other_dictionary = self.dictionary;
        let dictionary_length = dictionary.len();
        let random_number = rand::thread_rng().gen_range(1..=dictionary_length);
        let random_word = &dictionary[random_number];
        random_word.to_string()
    }

    fn line_to_words(line: &str) -> Vec<String> {
        line.split_whitespace().map(str::to_string).collect()
    }


    fn write_to_file(string: &str) {
        //error handling if already saved
        let mut file = File::create("wordle_dictionary.txt");
        write!(file.expect("REASON"), "{}", string);
    }

    async fn get_new_words() {
        let response = reqwest::get("https://www-cs-faculty.stanford.edu/~knuth/sgb-words.txt")
            .await
            // each response is wrapped in a `Result` type
            // we'll unwrap here for simplicity
            .unwrap()
            .text_with_charset("utf-8")
            .await;
        let t = response.unwrap(); //this is a string
        Self::write_to_file(&t);
    }

    fn check_real_word(users_guess_struct: &mut UserGuess, word_to_check: String) {
        let word_dictionary = Self::get_wordle_dictionary();
        let partial_dictionary = &word_dictionary[0..10];
        if word_dictionary.contains(&word_to_check) {
            users_guess_struct.real_word = true;
        }
        else {
            users_guess_struct.real_word = false;
        }
    }

}



fn tests() {
    test_real_words();
    // test_getting_new_words();
    // test_user_guesses_words_wrong_length();
}

fn test_real_words() {
    let words_to_check = ["one", "fiver", "steam", "liven", "barren", "boxey"];

    let mut users_guess_struct = UserGuess {
        word: String::from(""),
        length: 0,
        real_word: false,
        guessed_wordle: false,
    };
    
    for (index, element) in words_to_check.iter().enumerate() {
        users_guess_struct.word = element.to_string();
        FiveLetterDictionary::check_real_word(&mut users_guess_struct, element.to_string());
        if users_guess_struct.real_word == true {
            println!("{} in dictionary ", element.to_string());
        }
        else {
            println!("{} NOT in dictionary ", element.to_string()); 
        }
    }
}


// #[tokio::main]
// async fn main() {
fn main() {

    let wordle_word = FiveLetterDictionary::generate_wordle();
    // println!("wordle word = {:?}", wordle_word);
    
    // let mut user_guessed_wordle: bool = false;
    let mut user_guesses: i32 = 0;
    let mut user_guesses_remaining: i32 = 6;

    let mut users_guess_struct = UserGuess {
        word: String::from(""),
        length: 0,
        real_word: false,
        guessed_wordle: false,
    };

    //add cmd arg to bypass hello prompt
    hello_prompt();
    while user_guesses_remaining > 0  && users_guess_struct.guessed_wordle == false {
        // println!("user_guessed_wordle at top: {}", users_guess_struct.guessed_wordle);
        let temp = UserGuess::get_five_char_word(&mut users_guess_struct, user_guesses_remaining);
        let user_won = UserGuess::compare_words(&mut users_guess_struct, wordle_word.clone(), user_guesses);
        user_guesses_remaining-=1;
        user_guesses+=1;
    }
    println!("Thanks for playing!");
 
}
