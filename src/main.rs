/*
Simple Rust calculator command line application
November 2018 by Jacob Bogner
*/
use std::io::prelude::*;
use std::collections::HashMap;
use std::io;

fn readinput() -> string { //Get string from
    print!("Input string: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut string: String = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string;
}

fn setup(){
    let mut variables = HashMap::new();
}

fn prelimeval(input: String) -> string{ //Pull out any variable assignments and replace their value into input
    if input.contains('='){
        let v: Vec<&str> = input.split('=').collect();
    }
}

fn getvalue(input: int) -> value{
    return variables
}

fn main(){
    setup(); //All necessary setup functions
    let mut input = readinput(); // Get input from the keyboard
    while(input !=  "quit"){
        input = prelimeval(input); //Remove variable assignments/repalce variables with their value
        output = evaluate(input); //Evaluate the remaining expression
        print!(output); //Print the output to the screen
        input = readinput(); //Check for new input
    }


}