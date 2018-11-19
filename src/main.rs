/*
Simple Rust calculator command line application
November 2018 by Jacob Bogner
*/
use std::io::prelude::*;
use std::collections::HashMap;
use std::io;

fn readinput() -> String { //Get string from
    print!("Input string: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input = input.trim().to_string();
    return input;
}

fn setup(){

    let mut variable_map = HashMap::new();
    variable_map.insert(String::from("pi"), 3.14159);
}

fn prelimeval(input: String) { //Pull out any variable assignments or variable value requests and replace their value into input
    if input.contains('='){
        let v: Vec<&str> = input.split('=').collect();

    }

}

fn evaluate(input: String) { //Take an expression and evaluate, return an answer
    if input.contains('+'){
        let v: Vec<&str> = input.split('+').collect();

    }
    if input.contains('*'){
        let v: Vec<&str> = input.split('*').collect();

    }
    if input.contains('/'){
        let v: Vec<&str> = input.split('/').collect();

    }
    if input.contains('-'){
        let v: Vec<&str> = input.split('-').collect();

    }

}

fn main(){
    setup(); //All necessary setup functions
    let mut input = readinput(); // Get input from the keyboard
    while input != "quit" {
        //input = prelimeval(input); //Remove variable assignments/replace variables with their value
        //output = evaluate(input); //Evaluate the remaining expression
        //Print the output to the screen
        print!("{}\n", input);
        input = readinput(); //Check for new input
    }


}