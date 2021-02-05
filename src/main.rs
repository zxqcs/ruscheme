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
    /* test self-evaluating expressions and quoted expression*/
    let x1 =  Exp::Integer(1);
    let x2 = Exp::FloatNumber(3.14);
    let x3 = Exp::Quote("winter");
    let x4 = Exp::SchemeString("WINTER IS COMING!");
    let x5 = Exp::Bool(true);
    let x6 = Exp::Bool(false);
    let mut env = Env(Exp::List(Pair::Nil));
    pretty_print(eval(x1, env.clone()).unwrap());
    pretty_print(eval(x2, env.clone()).unwrap());
    pretty_print(eval(x3, env.clone()).unwrap());
    pretty_print(eval(x4, env.clone()).unwrap());
    pretty_print(eval(x5, env.clone()).unwrap());
    pretty_print(eval(x6, env.clone()).unwrap());
    println!("=================================");
    /* test look up simple variable */
   env = extend_environment(scheme_list!(Exp::Symbol("x")), 
                            scheme_list!(Exp::Integer(8)), env).unwrap(); 
    pretty_print(eval(Exp::Symbol("x"), env.clone()).unwrap());

    pretty_print(env.0.clone());
    /* test assignment */
    // (define x 101)
    let assignment = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(101));
    env = Env(eval(assignment.clone(), env).unwrap()); 
    // pretty_print(eval(Exp::Symbol("x"), env.clone()).unwrap());

    /* test definiton for single symbol */
    // (define x  999)
    let definition = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(999));
    env = Env(eval(definition, env).unwrap()); 
    // pretty_print(eval(Exp::Symbol("x"), env.clone()).unwrap());

    /* test if  */
    // (if false x 10000)
    let if_exp = scheme_list!(Exp::Symbol("if"), Exp::Bool(false),
                          Exp::Symbol("x"), Exp::Integer(10000));
    // pretty_print(eval(if_exp.clone(), env.clone()).unwrap());

    /* test lambad */
    // (lambda (x) (* x x))
    let lambda_exp = generate_test_data().lambda_expression;
    pretty_print(eval(lambda_exp.clone(), env.clone()).unwrap());

    /* test begin */
    // a serious issue-> any operations that has influence on env can't be 
    //  handled properly if not put in the tail of begin
    let begin_exp = make_begin(scheme_list!(Exp::Integer(1), Exp::Integer(2),
                                 Exp::Symbol("x"),
                                if_exp.clone(),
                            lambda_exp.clone(),
                           assignment.clone(),
                                Exp::Symbol("x")));
    pretty_print(eval(begin_exp, env.clone()).unwrap());

    /* test definiton for a procedure */
    // (define (square x) (* x x))
    let another_definition = scheme_list!(Exp::Symbol("define"), 
                                    scheme_list!(Exp::Symbol("square"),
                                                 Exp::Symbol("x")),
                                    scheme_list!(Exp::Symbol("*"),
                                                 Exp::Symbol("x"),
                                                 Exp::Symbol("x")));
    pretty_print(env.0.clone());
    env = Env(eval(another_definition.clone(), env.clone()).unwrap());
    pretty_print(env.0.clone());

    let second_definiton = scheme_list!(Exp::Symbol("define"), Exp::Symbol("y"), Exp::Integer(0));
    env = Env(eval(second_definiton.clone(), env.clone()).unwrap());
    pretty_print(env.0.clone());
    /* test application */
    let app_exp = scheme_list!(Exp::Symbol("square"), Exp::Integer(3));
    let operator = Exp::Symbol("square");
    pretty_print(eval(operator.clone(), env.clone()).unwrap());
    pretty_print(eval(app_exp.clone(), env.clone()).unwrap());

}