//! # ruscheme
//! 
//! ruscheme is a Scheme interpreter implemented in Rust which runs
//! rather fast.
mod core_of_interpreter;
mod represent;
mod parser;
mod display;
mod tool;
mod environment;
use display::display::pretty_print;
use crate::parser::parser::*;
use core_of_interpreter::core_of_interpreter::{ENV, Exp, eval};
use std::io::{self, Write};

fn prompt_for_input(s: String) {
    print!("{}", s);
    io::stdout().flush().unwrap();
}

fn input() -> Exp {
    let mut programs: Vec<String> = vec![]; 
    let _input = read_scheme_programs_from_stdin(&mut programs);
    let mut tokens = tokenize(&mut programs);
    let x = build_syntax_tree(&mut tokens);
    x
}
fn driver_loop() {
    let input_prompt = String::from("|-> ");
    prompt_for_input(input_prompt);
    let exp = input();
    unsafe {
        let output = eval(exp, ENV.clone()).unwrap();
        match output {
            Some(x) => {
                print!("=> ");
                pretty_print(x);
            },
            None => println!("=> value: OK"),
        }
    }
    driver_loop();
}

fn main() {
    println!("This is an interpreter for a subset of Scheme language implemented in Rust.");
    println!("Author: Yi; Image saved on Monday February 8, 2021 at 7:35 PM");
    println!("Happy New Year ^_^");
    driver_loop();
}