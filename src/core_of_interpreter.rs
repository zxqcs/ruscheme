#![allow(unused_variables)]

pub mod core_of_interpreter {
    use crate::{represent::represent::{assignment_variable, begin_actions, definition_value, definition_variable, first_exp, if_alternative, if_consequent, if_predicate, is_application, is_assignment, is_begin, is_compound_procedure, is_definiton, is_if, is_lambda, is_last_exp, is_primitive_procedure, lambda_body, lambda_parameters, make_procedure, operands, operator, procedure_body, procedure_environment, procedure_parameters, rest_exps}, tool::tools::{list_length, scheme_cons}};
    use crate::represent::represent::{no_operands, first_operand,rest_operands,car,cadr,cdr};
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

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub struct Env(pub Exp); 

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
    pub fn eval(exp: Exp, env: Env) -> Result<Exp, &'static str> {
        if exp.is_self_evaluating() {
            Ok(exp)
        } else if exp.is_primitive_procedure() {
            Ok(scheme_cons(Exp::Symbol("primitive"), exp))
        } else if exp.is_variable(){
            Ok(lookup_variable_value(exp, env))
        } else if exp.is_quoted() {
            Ok(exp)
        } else if is_assignment(exp.clone()) {
            Ok(eval_assignment(exp, env))
        } else if is_definiton(exp.clone()) {
            Ok(eval_definition(exp, env))
        } else if is_if(exp.clone()) {
            Ok(eval_if(exp, env))
        } else if is_lambda(exp.clone()) {
            Ok(make_procedure(lambda_parameters(exp.clone()), 
                                     lambda_body(exp), env))
        } else if is_begin(exp.clone()) {
            Ok(eval_sequence(begin_actions(exp), env))
        } else if is_application(exp.clone()) {
           Ok(apply(eval(operator(exp.clone()), env.clone()).unwrap(),
                     list_of_values(operands(exp), env)).unwrap())
        } else {
            Err("unknow expression, type: EVAL")
        }
    }

    #[allow(dead_code)]
    fn list_of_values(exps: Exp, env: Env) -> Exp {
        if no_operands(exps.clone()) {
            Exp::List(Pair::Nil)
        } else {
            scheme_cons(eval(first_operand(exps.clone()), env.clone()).unwrap(), 
                            list_of_values(rest_operands(exps), env))
        }
    }

    #[allow(dead_code)]
    fn eval_if(exp: Exp, env: Env) -> Exp {
        if eval(if_predicate(exp.clone()), env.clone()).unwrap() == Exp::Bool(true) {
            eval(if_consequent(exp), env).unwrap()
        } else {
            eval(if_alternative(exp), env).unwrap()
        }
    }    

    #[allow(dead_code)]
    fn eval_assignment(exp: Exp, env: Env) -> Exp {
        let temp = set_variable_value(assignment_variable(exp.clone()), 
        eval(definition_value(exp), env.clone()).unwrap(), env);
        temp.0
    }

    #[allow(dead_code)]
    fn eval_definition(exp: Exp, env: Env) -> Exp {
        let temp = define_variable(definition_variable(exp.clone()), 
                       eval(definition_value(exp), env.clone()).unwrap(), 
                                 env);
        temp.0
    }

    #[allow(dead_code)]
    fn eval_sequence(exps: Exp, env: Env) -> Exp {
        if is_last_exp(exps.clone()) {
            eval(first_exp(exps),env).unwrap()
        } else {
            let temp = eval(first_exp(exps.clone()), env.clone()).unwrap();
            eval_sequence(rest_exps(exps), env)
        }
    }

    #[allow(dead_code)]
    fn apply(p: Exp, args: Exp) -> Result<Exp, &'static str> {
        if is_primitive_procedure(p.clone()) {
            Ok(apply_primitive_procedure(p, args))
        } else if is_compound_procedure(p.clone()) {
            Ok(eval_sequence(procedure_body(p.clone()), extend_environment(
                                procedure_parameters(p.clone()), args,
                              procedure_environment(p)).unwrap()))
        } else {
            Err("unknow procedure type: APPLY")
        }
    }

    fn apply_primitive_procedure(p: Exp, args: Exp) -> Exp {
        if let Exp::Symbol(x) = cadr(p).unwrap() {
            match x {
                "*" => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        Exp::FloatNumber(lhs.to_number() * rhs.to_number())
                    } else { panic!("wrong number of args!");}
                },
                "/" => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        if rhs.to_number() == 0.0 {
                            panic!("divide by zero!");
                        } else {
                            Exp::FloatNumber(lhs.to_number() / rhs.to_number())
                        }
                    } else { panic!("wrong number of args!"); }
                },
                "+" => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        Exp::FloatNumber(lhs.to_number() + rhs.to_number())
                    } else { panic!("wrong number of args!"); }
                },
                "-" => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        Exp::FloatNumber(lhs.to_number() - rhs.to_number())
                    } else { panic!("wrong number of args!"); }
                },
                "car" => {
                    if args.is_pair() {
                        car(args).unwrap()
                    } else { panic!("not a proper schemem list: car"); }
                }
                "cdr" => {
                    if args.is_pair() {
                        cdr(args).unwrap()
                    } else { panic!("not a proper schemem list: cdr"); }
                },
                "cons" => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        scheme_cons(lhs, rhs)
                    } else { panic!("not a proper schemem list: cons"); }
                },
                "null?" => {
                    if list_length(args.clone()) == 1 {
                        if car(args).unwrap() == Exp::List(Pair::Nil) {
                            Exp::Bool(true)
                        } else {
                            Exp::Bool(false)
                        }
                    } else { panic!("not a proper schemem list: cons"); }
                },
                _ => { panic!("attemp to run a primitive procedure that is not implemented yet!") },
            }
        } else {
            panic!("not a proper primitive procedure!");
        }
    }
}
