#![allow(unused_variables)]

pub mod core_of_interpreter {
    use crate::{represent::represent::{assignment_variable, definition_value, definition_variable, if_alternative, if_consequent, if_predicate, }, tool::tools::scheme_cons};
    use crate::represent::represent::{no_operands, first_operand,rest_operands};
    use crate::environment::env::*;

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub enum Pair {
        Cons(Box<Exp>, Box<Pair>),
        Nil,
    }

    impl PartialEq for Pair {
        fn eq(&self, other: &Self) -> bool {
            match self{
                Pair::Nil => {
                    match other {
                        Pair::Nil => true,
                        _ => false,
                    }
                },
                Pair::Cons(x, y) => {
                    match other {
                        Pair::Nil => false,
                        Pair::Cons(x1, y1) => {
                            if x == x1 && y == y1 { true } else { false }
                        },
                    }
                },
            }
        }
    }

    /* everything is an Exp to be interpreted */
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub enum Exp {
        FloatNumber(f32),
        Integer(i32),
        List(Pair),
        Symbol(&'static str),
        Quote(&'static str),
        SchemeString(&'static str),
        Bool(bool),
    }

    impl PartialEq for Exp {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Exp::FloatNumber(x) => {
                    match other {
                        Exp::FloatNumber(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },
                
                Exp::Integer(x) => {
                    match other {
                        Exp::Integer(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },

                Exp::List(x) => {
                    match other {
                        Exp::List(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },
                
                Exp::Symbol(x) => {
                    match other {
                        Exp::Symbol(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },

                Exp::Quote(x) => {
                    match other {
                        Exp::Quote(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },

                Exp::Bool(x) => {
                    match other {
                        Exp::Bool(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },

                Exp::SchemeString(x) => {
                    match other {
                        Exp::SchemeString(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },
            }
        }
    }

    /* core function of the Scheme interpreter */
    #[allow(dead_code)]
    fn eval(exp: Exp, env: Exp) -> Result<Exp, &'static str> {Ok(exp)}

    #[allow(dead_code)]
    fn list_of_values(exps: Exp, env: Exp) -> Exp {
        if no_operands(exps.clone()) {
            Exp::List(Pair::Nil)
        } else {
            scheme_cons(eval(first_operand(exps.clone()), env.clone()).unwrap(), 
                            list_of_values(rest_operands(exps), env))
        }
    }

    #[allow(dead_code)]
    fn eval_if(exp: Exp, env: Exp) -> Exp {
        if eval(if_predicate(exp.clone()), env.clone()).unwrap() == Exp::Bool(true) {
            eval(if_consequent(exp), env).unwrap()
        } else {
            eval(if_alternative(exp), env).unwrap()
        }
    }    

    #[allow(dead_code)]
    fn eval_assignment(exp: Exp, env: Exp) -> Exp {
        set_variable_value(assignment_variable(exp.clone()), 
        eval(definition_value(exp), env.clone()).unwrap(), env)
    }

    #[allow(dead_code)]
    fn eval_definition(exp: Exp, env: Exp) -> Exp {
        define_variable(definition_variable(exp.clone()), 
                       eval(definition_value(exp), env.clone()).unwrap(), 
                                 env)
    }
    // to be implemented later
    #[allow(dead_code)]
    fn eval_sequence(exps: Exp, env: Exp) -> Exp {
        Exp::List(Pair::Nil)
    }

    #[allow(dead_code)]
    fn apply(p: Exp, args: Exp) -> Result<Exp, &'static str> {Ok(args)}
}
