mod core_of_interpreter;
mod represent;
mod parser;
use core_of_interpreter::core_of_interpreter::{Exp, Pair::*};


fn main() {
    let x = Exp::List(Nil);
    println!("{:?}", x);
    println!("Hello world!");
}