/*
Simple Rust calculator command line application
November 2018 by Jacob Bogner
*/
use std::io::prelude::*;
use std::collections::HashMap;
use std::io;

fn readinput() -> string {
    print!("Input string: ");
    io::stdout().flush().ok().expect("Could not flush stdout");
    let mut string: String = String::new();
    let _ = std::io::stdin().read_line(&mut string);
    return string;
}

fn setup(){
    let mut variables = HashMap::new();
}

fn evaluateinput(input: String){

}

fn getvalue(input: int) -> value{
    return variables
}