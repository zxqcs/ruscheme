mod core_of_interpreter;
mod represent;
mod parser;
mod display;
mod tool;
use display::display::pretty_print;
use core_of_interpreter::core_of_interpreter::{Exp, Pair};
use tool::tools::{scheme_cons, append, generate_test_data};
use represent::represent::*;
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
    let data = generate_test_data();
    let if_exp = data.if_expression;
    assert_eq!(is_if(if_exp.clone()), true);
    let s1 = Exp::Symbol("if");
    let s2 = Exp::Symbol("n");
    let s3 = Exp::Integer(1);
    let s4 = Exp::Symbol("-");
    let s5 = Exp::Symbol("=");
    let x1 = scheme_list!(s5, s2.clone(), s3.clone());
    let x2 = scheme_list!(s4, s2.clone(), s3.clone());
    assert_eq!(if_predicate(if_exp.clone()), x1);
    assert_eq!(if_consequent(if_exp.clone()), s3);
    assert_eq!(if_alternative(if_exp.clone()), x2);
    pretty_print(if_exp.clone());
    let s = make_if(if_predicate(if_exp.clone()), if_consequent(if_exp.clone()),
                   if_alternative(if_exp.clone()));
    pretty_print(s);
    pretty_print(x2.clone());
    pretty_print(s3.clone());
    pretty_print(scheme_cons(s3, x2.clone()));
    pretty_print(scheme_cons(x1.clone(), x2.clone()));

}