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

#[cfg(test)]
mod test {
    use crate::environment::env::{extend_environment};  
    use crate::scheme_list;
    use crate::tool::tools::{append, scheme_cons};
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Env, Pair, eval};
    #[test]
    fn test_eval_self_evaluating_exp() {
        let x1 =  Exp::Integer(1);
        let x2 = Exp::FloatNumber(3.14);
        let x3 = Exp::Quote("winter");
        let x4 = Exp::SchemeString("WINTER IS COMING!");
        let x5 = Exp::Bool(true);
        let x6 = Exp::Bool(false);
        let env = Env(Exp::List(Pair::Nil));
        assert_eq!(x1.clone(), eval(x1, env.clone()).unwrap());
        assert_eq!(x2.clone(), eval(x2, env.clone()).unwrap());
        assert_eq!(x3.clone(), eval(x3, env.clone()).unwrap());
        assert_eq!(x4.clone(), eval(x4, env.clone()).unwrap());
        assert_eq!(x5.clone(), eval(x5, env.clone()).unwrap());
        assert_eq!(x6.clone(), eval(x6, env.clone()).unwrap());
    }

    #[test]
    fn test_eval_single_variable() {
        let mut env = Env(Exp::List(Pair::Nil));
        env = extend_environment(scheme_list!(Exp::Symbol("x")), 
                                 scheme_list!(Exp::Integer(8)), env).unwrap(); 
        assert_eq!(eval(Exp::Symbol("x"), env.clone()).unwrap(), Exp::Integer(8));
    }

    #[test]
    fn test_eval_assignment() {
        let mut env = Env(Exp::List(Pair::Nil));
        env = extend_environment(scheme_list!(Exp::Symbol("x")), 
                                 scheme_list!(Exp::Integer(8)), env).unwrap(); 
        
        let assignment = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(101));
        env = Env(eval(assignment.clone(), env).unwrap()); 
        assert_eq!(eval(Exp::Symbol("x"), env.clone()).unwrap(), Exp::Integer(101));
    }

    #[test]
    fn test_eval_definition_single_variable() {
        let mut env = Env(Exp::List(Pair::Nil));
        env = extend_environment(scheme_list!(Exp::Symbol("x")), 
                                 scheme_list!(Exp::Integer(8)), env).unwrap(); 
        
        let assignment = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(101));
        env = Env(eval(assignment.clone(), env).unwrap()); 
        let definition = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(999));
        env = Env(eval(definition, env).unwrap()); 
        let second_definition = scheme_list!(Exp::Symbol("define"), Exp::Symbol("y"), Exp::Integer(333));
        env = Env(eval(second_definition, env).unwrap()); 
        assert_eq!(eval(Exp::Symbol("x"), env.clone()).unwrap(), Exp::Integer(999));
        assert_eq!(eval(Exp::Symbol("y"), env.clone()).unwrap(), Exp::Integer(333));
    }

    #[test]
    fn test_eval_definition_compoud_procedure() {
        let mut env = Env(Exp::List(Pair::Nil));
        env = extend_environment(scheme_list!(Exp::Symbol("x")), 
                                 scheme_list!(Exp::Integer(8)), env).unwrap(); 
        
        let assignment = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(101));
        env = Env(eval(assignment.clone(), env).unwrap()); 
        let definition = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(999));
        env = Env(eval(definition, env).unwrap()); 
        let second_definition = scheme_list!(Exp::Symbol("define"), Exp::Symbol("y"), Exp::Integer(333));
        env = Env(eval(second_definition, env).unwrap()); 
        let another_definition = scheme_list!(Exp::Symbol("define"), 
                                    scheme_list!(Exp::Symbol("square"),
                                                 Exp::Symbol("x")),
                                    scheme_list!(Exp::Symbol("*"),
                                                 Exp::Symbol("x"),
                                                 Exp::Symbol("x")));
        env = Env(eval(another_definition.clone(), env.clone()).unwrap());
        let app_exp = scheme_list!(Exp::Symbol("square"), Exp::Integer(3));
        assert_eq!(eval(app_exp, env).unwrap(), Exp::FloatNumber(9.0));
    }

    #[test]
    fn test_eval_primitive_procedure() {
        let mut env = Env(Exp::List(Pair::Nil));
        env = extend_environment(scheme_list!(Exp::Symbol("x")), 
                                 scheme_list!(Exp::Integer(8)), env).unwrap(); 
        
        let assignment = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(101));
        env = Env(eval(assignment.clone(), env).unwrap()); 
        let definition = scheme_list!(Exp::Symbol("define"), Exp::Symbol("x"), Exp::Integer(999));
        env = Env(eval(definition, env).unwrap()); 
        let second_definition = scheme_list!(Exp::Symbol("define"), Exp::Symbol("y"), Exp::Integer(333));
        env = Env(eval(second_definition, env).unwrap()); 
        let another_definition = scheme_list!(Exp::Symbol("define"), 
                                    scheme_list!(Exp::Symbol("square"),
                                                 Exp::Symbol("x")),
                                    scheme_list!(Exp::Symbol("*"),
                                                 Exp::Symbol("x"),
                                                 Exp::Symbol("x")));
        env = Env(eval(another_definition.clone(), env.clone()).unwrap());
        let app_exp = scheme_list!(Exp::Symbol("square"), Exp::Integer(3));

        let s = scheme_list!(scheme_list!(Exp::Integer(1), Exp::Integer(2)),
                                 scheme_list!(Exp::Integer(3), Exp::Integer(4)),
                                 Exp::Integer(5));
        let s_definition = scheme_list!(Exp::Symbol("define"),
                                           Exp::Symbol("s"),
                                           s);
        env = Env(eval(s_definition.clone(), env).unwrap());
        let car_exp = scheme_list!(Exp::Symbol("car"), Exp::Symbol("s"));
        assert_eq!(eval(car_exp, env.clone()).unwrap(), scheme_list!(Exp::Integer(1),
                                                             Exp::Integer(2)));
        let cdr_exp = scheme_list!(Exp::Symbol("cdr"), Exp::Symbol("s"));
        assert_eq!(eval(cdr_exp, env.clone()).unwrap(),
                   scheme_list!(scheme_list!(Exp::Integer(3), Exp::Integer(4)),
                                Exp::Integer(5)));
    }
}