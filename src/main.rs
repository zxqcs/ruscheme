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
use crate::represent::represent::{cadr, car, cdr};

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
    let mut env = Env(Exp::List(Pair::Nil));
    let x = scheme_list!(Exp::Symbol("define".to_string()), 
                             Exp::Symbol("x".to_string()),
                             Exp::FloatNumber(3.0));
    env = Env(eval(x, env).unwrap());
    let y = scheme_list!(Exp::Symbol("define".to_string()),
                      scheme_list!(Exp::Symbol("fac".to_string()),
                                   Exp::Symbol("n".to_string())),
                      scheme_list!(Exp::Symbol("if".to_string()),
                            scheme_list!(Exp::Symbol("=".to_string()),
                                         Exp::Symbol("n".to_string()),
                                         Exp::FloatNumber(1.0)),
                            Exp::FloatNumber(1.0),
                            scheme_list!(Exp::Symbol("*".to_string()),
                                         Exp::Symbol("n".to_string()),
                                   scheme_list!(Exp::Symbol("fac".to_string()),
                                          scheme_list!(Exp::Symbol("-".to_string()),
                                                       Exp::Symbol("n".to_string()),
                                                       Exp::FloatNumber(1.0))))));
    env = Env(eval(y, env).unwrap());
    println!("fac defined");
    let s = scheme_list!(Exp::Symbol("fac".to_string()), Exp::FloatNumber(20.0));
    pretty_print(eval(s, env).unwrap());
}