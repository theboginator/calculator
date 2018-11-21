/*
Simple Rust calculator command line application
November 2018 by Jacob Bogner
*/
use std::io::prelude::*;
use std::collections::HashMap;
use std::io;


fn readinput() -> String { //Get string from keyboard
    print!("Input string: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input = input.trim().to_string();
    input.retain(|c| c != ' ');
    return input;
}



fn evaluate(input: String) -> f32 { //Take an expression and evaluate, return an answer
     //Split the input then we will extract the numbers and operator
    let mut answer: f32 = 0.0;
    if input.contains('+'){
        let v: Vec<&str> = input.split('+').collect();
        //print!("{:?}", v);
        let a = v[0].parse::<f32>().unwrap();
        let b = v[1].parse::<f32>().unwrap();
        answer = a+b;

    }
    if input.contains('*'){
        let v: Vec<&str> = input.split('*').collect();
        let a = v[0].parse::<f32>().unwrap();
        let b = v[1].parse::<f32>().unwrap();
        answer = a*b;
    }
    if input.contains('/'){
        let v: Vec<&str> = input.split('/').collect();
        let a = v[0].parse::<f32>().unwrap();
        let b = v[1].parse::<f32>().unwrap();
        answer = a/b;
    }
    if input.contains('-'){
        let v: Vec<&str> = input.split('-').collect();
        let a = v[0].parse::<f32>().unwrap();
        let b = v[1].parse::<f32>().unwrap();
        answer = a-b;
    }
    return answer;

}

fn main(){
    let mut variable_map = HashMap::new(); //declare the variable hashmap
    variable_map.insert(String::from("pi"), 3.14159); //make our first constant, pi

    let mut input = readinput(); // Get input from the keyboard

    while input != "quit" {
        //input = prelimeval(input); //Remove variable assignments/replace variables with their value

        let output = evaluate(input); //Evaluate the remaining expression

        //Print the output to the screen

        print!("{}\n", output);

        input = readinput(); //Check for new input
    }

    for (key, value) in variable_map{
        print!("{} / {}", key, value);
    }

    fn prelimeval(input: String) -> String { //Pull out any variable assignments or variable value requests and replace their value into input
        if input.contains('='){
            let v: Vec<&str> = input.split('=').collect();
            let key: &str = v[0];
            let valtostr: &str = v[1];
            let val = valtostr.parse::<i32>().unwrap();

            //variable_map.insert(key, val);
        }

        return input;

    }

}