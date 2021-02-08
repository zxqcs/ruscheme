mod core_of_interpreter;
mod represent;
mod parser;
mod display;
mod tool;
mod environment;
use display::display::pretty_print;
use crate::parser::parser::*;
use core_of_interpreter::core_of_interpreter::{Exp, Pair, eval};
use tool::tools::{append, scheme_cons};
use std::io::{self, Write};

#[macro_export]
macro_rules! scheme_list {
    ( $( $x:expr ),* ) => {
        {
            let null = Exp::List(Pair::Nil);
            let mut temp_list = null.clone(); 
            $(
                temp_list = append(temp_list, scheme_cons($x, null.clone()));
            )*
            temp_list
        }
    }
}

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
    println!("\n");
    let output = eval(exp).unwrap();
    match output {
        Some(x) => {
            print!("=> ");
            pretty_print(x);
        },
        None => println!("OK"),
    }
    driver_loop();
}

fn main() {
    driver_loop();
}