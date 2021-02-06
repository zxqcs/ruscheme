mod core_of_interpreter;
mod represent;
mod parser;
mod display;
mod tool;
mod environment;
use display::display::pretty_print;
use core_of_interpreter::core_of_interpreter::{Exp, Pair, Env, eval};
use environment::env::{extend_environment};
use represent::represent::make_begin;
use tool::tools::{append, generate_test_data, scheme_cons};

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

fn main() {
    println!("Hello Rust world!");
}