mod core_of_interpreter;
mod represent;
mod parser;
mod display;
mod tool;
mod environment;
use display::display::pretty_print;
use core_of_interpreter::core_of_interpreter::{Exp};
use tool::tools::{generate_test_frames};
use environment::env::add_binding_to_frame;

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
    };
}
fn main() {
    let data = generate_test_frames();
    let a = Exp::Symbol("a");
    let four = Exp::Integer(4);
    let s = add_binding_to_frame(a, four, data.frame.clone());
    let s1 = add_binding_to_frame(Exp::Symbol("b"), Exp::Integer(5), s.clone());
    pretty_print(data.frame);
    pretty_print(s);
    pretty_print(data.extended_frame);
    pretty_print(s1);
}