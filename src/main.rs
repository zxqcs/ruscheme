mod core_of_interpreter;
mod represent;
mod parser;
mod display;
mod tool;
use display::display::pretty_print;
use core_of_interpreter::core_of_interpreter::{Exp, Pair};
use tool::tools::{scheme_cons, append};
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
    let n1 = Exp::Integer(1);
    let n2 = Exp::Integer(2);
    let n3 = Exp::Integer(3);
    pretty_print(scheme_list!(n1, n2 , n3));
}