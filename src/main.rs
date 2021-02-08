mod core_of_interpreter;
mod represent;
mod parser;
mod display;
mod tool;
mod environment;
use display::display::pretty_print;
use core_of_interpreter::core_of_interpreter::{Exp, Pair, eval};
use tool::tools::{append, scheme_cons};

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
    let x = scheme_list!(Exp::Symbol("define".to_string()), 
                             Exp::Symbol("x".to_string()),
                             Exp::FloatNumber(3.0));
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
    let z = scheme_list!(Exp::Symbol("fac".to_string()),
                             Exp::FloatNumber(4.0));
    println!("Hello Rust!");
    let _s1 = eval(x);
    let _s2 = eval(y);
    pretty_print(eval(z).unwrap().unwrap());
}