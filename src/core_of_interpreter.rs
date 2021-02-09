#![allow(unused_variables)]

pub mod core_of_interpreter {
    use crate::{display::display::{pretty_print}, 
                represent::represent::{assignment_value, assignment_variable, begin_actions, 
                                       caar, cdar, definition_value, definition_variable, 
                                       first_exp, if_alternative, if_consequent, if_predicate, 
                                       is_application, is_assignment, is_begin, 
                                       is_compound_procedure, is_definiton, is_if, is_lambda, 
                                       is_last_exp, is_number_combination, 
                                       is_primitive_procedure, lambda_body, lambda_parameters, 
                                       make_procedure, operands, operator, procedure_body,
                                         procedure_parameters, rest_exps}, tool::tools::{list_length, scheme_cons}};
    use crate::represent::represent::{no_operands, first_operand,rest_operands,car,cadr};
    use crate::environment::env::*;

    pub static mut ENV:Env = Env(Exp::List(Pair::Nil));

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
    impl PartialEq for Env {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    /* everything is an Exp to be interpreted */
    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    pub enum Exp {
        FloatNumber(f32),
        Integer(i32),
        List(Pair),
        Symbol(String),
        Quote(String),
        SchemeString(String),
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
    pub fn eval(exp: Exp, env: Env) -> Result<Option<Exp>, &'static str> {
        if exp.is_self_evaluating() {
            Ok(Some(exp))
        } else if is_number_combination(exp.clone()) {
            Ok(Some(exp))
        } else if exp.is_primitive_procedure() {
            Ok(Some(scheme_cons(Exp::Symbol("primitive".to_string()), exp)))
        } else if exp.is_variable(){
            Ok(Some(lookup_variable_value(exp, env)))
        } else if exp.is_quoted() {
            Ok(Some(exp))
        } else if is_assignment(exp.clone()) {
            Ok(eval_assignment(exp, env))
        } else if is_definiton(exp.clone()) {
            Ok(eval_definition(exp, env))
        } else if is_if(exp.clone()) {
            Ok(eval_if(exp, env))
        } else if is_lambda(exp.clone()) {
            Ok(Some(make_procedure(lambda_parameters(exp.clone()), 
                                        lambda_body(exp))))
        } else if is_begin(exp.clone()) {
            Ok(eval_sequence(begin_actions(exp), env))
        } else if is_application(exp.clone()) {
            apply(eval(operator(exp.clone()), env.clone()).unwrap().unwrap(),
                  list_of_values(operands(exp), env))
            
        } else {
            Err("unknow expression, type: EVAL")
        }
    }
 
    #[allow(dead_code)]
    fn apply(p: Exp, args: Exp) -> Result<Option<Exp>, &'static str> {
        if is_primitive_procedure(p.clone()) {
            Ok(apply_primitive_procedure(p, args))
        } else if is_compound_procedure(p.clone()) {
            unsafe {
                 Ok(eval_sequence(procedure_body(p.clone()), 
                                   extend_environment(procedure_parameters(p.clone()),
                                                          args,
                                                       ENV.clone())))
            }
       } else {
            Err("unknow procedure type: APPLY")
        }
    }

    #[allow(dead_code)]
    fn eval_sequence(exps: Exp, env: Env) -> Option<Exp> { 
        if is_last_exp(exps.clone()) {
            eval(first_exp(exps), env).unwrap()
        } else {
            let temp = eval(first_exp(exps.clone()), env.clone()).unwrap();
            eval_sequence(rest_exps(exps), env.clone())
        }
    }

    #[allow(dead_code)]
    fn list_of_values(exps: Exp, env: Env) -> Exp {
        if no_operands(exps.clone()) {
            Exp::List(Pair::Nil)
        } else {
            scheme_cons(eval(first_operand(exps.clone()), env.clone()).unwrap().unwrap(), 
                            list_of_values(rest_operands(exps), env.clone()))
        }
    }

    #[allow(dead_code)]
    fn eval_if(exp: Exp, env: Env) -> Option<Exp> {
        if eval(if_predicate(exp.clone()), env.clone()).unwrap().unwrap() == Exp::Bool(true) {
            eval(if_consequent(exp), env).unwrap()
        } else {
            eval(if_alternative(exp), env).unwrap()
        }
    }    

    #[allow(dead_code)]
    fn eval_assignment(exp: Exp, env: Env) -> Option<Exp> {
        unsafe {
            let temp = set_variable_value(assignment_variable(exp.clone()), 
                   eval(assignment_value(exp), env.clone()).unwrap().unwrap(), env.clone());
            ENV = temp;
        }
        None
    }

    #[allow(dead_code)]
    fn eval_definition(exp: Exp, env: Env) -> Option<Exp> {
        unsafe {
            let temp = define_variable(definition_variable(exp.clone()), 
                                          eval(definition_value(exp), env.clone()).unwrap().unwrap(), 
                                                    env.clone());
            ENV = temp;
        }
        None
    }

    fn apply_primitive_procedure(p: Exp, args: Exp) -> Option<Exp> {
        if let Exp::Symbol(x) = cadr(p).unwrap() {
            match x {
                t if t =="*".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        match lhs {
                            Exp::Integer(x) => {
                                match rhs {
                                    Exp::Integer(y) => {
                                        let r = x * y;
                                        Some(Exp::Integer(r))
                                    },
                                    Exp::FloatNumber(y) => {
                                        let r = x as f32 * y;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    _ => panic!("wrong type for multiply!"),
                                }
                            },
                            Exp::FloatNumber(x) => {
                                 match rhs {
                                    Exp::Integer(y) => {
                                        let r = x * y as f32;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    Exp::FloatNumber(y) => {
                                        let r = x * y;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    _ => panic!("wrong type for multiply!"),
                                }

                            }
                            _ => panic!("wrong type for multiply!"),
                        }
                    } else { panic!("wrong number of args!");}
                },
                t if t == "/".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        match rhs {
                            Exp::Integer(x) => {
                                if x == 0 {
                                    panic!("divide by zero!");
                                } else {
                                    match lhs {
                                        Exp::Integer(y) => {
                                            Some(Exp::FloatNumber((y as f32) / (x as f32)))
                                        },
                                        Exp::FloatNumber(y) => {
                                            Some(Exp::FloatNumber(y / (x as f32)))
                                        },
                                        _ => panic!("wrong type for division!"),
                                    }
                                }
                            },
                            Exp::FloatNumber(x) => {
                                if x == 0.0 {
                                    panic!("divide by zero!");
                                } else {
                                     match lhs {
                                        Exp::Integer(y) => {
                                            Some(Exp::FloatNumber((y as f32) / x))
                                        },
                                        Exp::FloatNumber(y) => {
                                            Some(Exp::FloatNumber(y / x))
                                        },
                                        _ => panic!("wrong type for division!"),
                                    }

                                }
                            },
                            _ => panic!("wrong type for division!"),
                        }
                    } else { 
                        panic!("wrong number of args!"); 
                    }
                },
                t if t == "+".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                         match lhs {
                            Exp::Integer(x) => {
                                match rhs {
                                    Exp::Integer(y) => {
                                        let r = x + y;
                                        Some(Exp::Integer(r))
                                    },
                                    Exp::FloatNumber(y) => {
                                        let r = x as f32 + y;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    _ => panic!("wrong type for add!"),
                                }
                            },
                            Exp::FloatNumber(x) => {
                                 match rhs {
                                    Exp::Integer(y) => {
                                        let r = x + y as f32;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    Exp::FloatNumber(y) => {
                                        let r = x + y;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    _ => panic!("wrong type for add!"),
                                }

                            }
                            _ => panic!("wrong type for add!"),
                        }
                    } else { 
                        panic!("wrong number of args!"); 
                    }
                },
                t if t == "-".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        match lhs {
                            Exp::Integer(x) => {
                                match rhs {
                                    Exp::Integer(y) => {
                                        let r = x - y;
                                        Some(Exp::Integer(r))
                                    },
                                    Exp::FloatNumber(y) => {
                                        let r = x as f32 - y;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    _ => panic!("wrong type for substract!"),
                                }
                            },
                            Exp::FloatNumber(x) => {
                                 match rhs {
                                    Exp::Integer(y) => {
                                        let r = x - y as f32;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    Exp::FloatNumber(y) => {
                                        let r = x - y;
                                        Some(Exp::FloatNumber(r))
                                    },
                                    _ => panic!("wrong type for substract!"),
                                }

                            }
                            _ => panic!("wrong type for substract!"),
                        }
                    } else { 
                        panic!("wrong number of args!"); 
                    }
                },
                t if t == "car".to_string() => {
                    if args.is_pair() {
                        Some(caar(args).unwrap())
                    } else { panic!("not a proper schemem list: car"); }
                }
                t if t == "cdr".to_string() => {
                    if args.is_pair() {
                        Some(cdar(args).unwrap())
                    } else { panic!("not a proper schemem list: cdr"); }
                },
                t if t == "cons".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        Some(scheme_cons(lhs, rhs))
                    } else { panic!("not a proper schemem list: cons"); }
                },
                t if t == "null?".to_string() => {
                    if list_length(args.clone()) == 1 {
                        if car(args).unwrap() == Exp::List(Pair::Nil) {
                            Some(Exp::Bool(true))
                        } else {
                            Some(Exp::Bool(false))
                        }
                    } else { panic!("not a proper schemem list: cons"); }
                },
                t if t == "=".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        if lhs == rhs {
                            Some(Exp::Bool(true))
                        } else {
                            Some(Exp::Bool(false))
                        }
                    } else { panic!("wrong number of args!");} 
                },
                t if t == ">".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        match lhs {
                            Exp::Integer(x) => {
                                match rhs {
                                    Exp::Integer(y) => {
                                        if x > y {
                                            Some(Exp::Bool(true))
                                        } else {
                                            Some(Exp::Bool(false))
                                        }
                                    },
                                    Exp::FloatNumber(y) => {
                                        let t = x as f32;
                                        if t > y {
                                            Some(Exp::Bool(true))
                                        } else {
                                            Some(Exp::Bool(false))
                                        }
                                    },
                                    _ => panic!("type mismatch for comparision!"),
                                }
                            },
                            Exp::FloatNumber(x) => {
                                match rhs {
                                    Exp::Integer(y) => {
                                        let t = y as f32;
                                        if x > t {
                                            Some(Exp::Bool(true))
                                        } else {
                                            Some(Exp::Bool(false))
                                        }
                                    },
                                    Exp::FloatNumber(y) => {
                                        if x > y {
                                            Some(Exp::Bool(true))
                                        } else {
                                            Some(Exp::Bool(false))
                                        }
                                    },
                                    _ => panic!("type mismatch for comparision!"),
                                }
 
                            }
                            _ => panic!("type mismatch for comparision"),
                        }
                    } else {
                        panic!("wrong number of ars!");
                    }
                },
                t if t == "<".to_string() => {
                    if list_length(args.clone()) == 2 {
                        let lhs = car(args.clone()).unwrap();
                        let rhs = cadr(args.clone()).unwrap();
                        match lhs {
                            Exp::Integer(x) => {
                                match rhs {
                                    Exp::Integer(y) => {
                                        if x > y {
                                            Some(Exp::Bool(false))
                                        } else {
                                            Some(Exp::Bool(true))
                                        }
                                    },
                                    Exp::FloatNumber(y) => {
                                        let t = x as f32;
                                        if t > y {
                                            Some(Exp::Bool(false))
                                        } else {
                                            Some(Exp::Bool(true))
                                        }
                                    },
                                    _ => panic!("type mismatch for comparision!"),
                                }
                            },
                            Exp::FloatNumber(x) => {
                                match rhs {
                                    Exp::Integer(y) => {
                                        let t = y as f32;
                                        if x > t {
                                            Some(Exp::Bool(false))
                                        } else {
                                            Some(Exp::Bool(true))
                                        }
                                    },
                                    Exp::FloatNumber(y) => {
                                        if x > y {
                                            Some(Exp::Bool(false))
                                        } else {
                                            Some(Exp::Bool(true))
                                        }
                                    },
                                    _ => panic!("type mismatch for comparision!"),
                                }
 
                            }
                            _ => panic!("type mismatch for comparision"),
                        }
                    } else {
                        panic!("wrong number of ars!");
                    }
                },
                t if t == "display".to_string() => {
                    let x = car(args).unwrap();
                    pretty_print(x);
                    None
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
    use crate::scheme_list;
    use crate::tool::tools::{append, scheme_cons};
    use crate::core_of_interpreter::core_of_interpreter::{ENV, Exp, Pair, eval};
    #[test]
    fn test_eval_self_evaluating_exp() {
        let x1 =  Exp::Integer(1);
        let x2 = Exp::FloatNumber(3.14);
        let x3 = Exp::Quote("winter".to_string());
        let x4 = Exp::SchemeString("WINTER IS COMING!".to_string());
        let x5 = Exp::Bool(true);
        let x6 = Exp::Bool(false);
        unsafe {
        assert_eq!(x1.clone(), eval(x1, ENV.clone()).unwrap().unwrap());
        assert_eq!(x2.clone(), eval(x2, ENV.clone()).unwrap().unwrap());
        assert_eq!(x3.clone(), eval(x3, ENV.clone()).unwrap().unwrap());
        assert_eq!(x4.clone(), eval(x4, ENV.clone()).unwrap().unwrap());
        assert_eq!(x5.clone(), eval(x5, ENV.clone()).unwrap().unwrap());
        assert_eq!(x6.clone(), eval(x6, ENV.clone()).unwrap().unwrap());
        }
   }

    #[test]
    fn test_eval_single_variable() {
         unsafe {
            eval(scheme_list!(Exp::Symbol("define".to_string()),
                              Exp::Symbol("x".to_string()),
                              Exp::Integer(8)), ENV.clone());
            assert_eq!(eval(Exp::Symbol("x".to_string()), ENV.clone()).unwrap().unwrap(), Exp::Integer(8));
         }
    }

    #[test]
    fn test_eval_assignment() {
        unsafe {
        eval(scheme_list!(Exp::Symbol("define".to_string()),
                          Exp::Symbol("x".to_string()),
                          Exp::Integer(8)), ENV.clone());
        let assignment = scheme_list!(Exp::Symbol("define".to_string()), 
                                          Exp::Symbol("x".to_string()), Exp::Integer(101));
        let x = eval(assignment.clone(), ENV.clone()); 
        assert_eq!(eval(Exp::Symbol("x".to_string()), ENV.clone()).unwrap().unwrap(), Exp::Integer(101));
        }
    }

    #[test]
    fn test_eval_definition_single_variable() {
        
        unsafe {
            eval(scheme_list!(Exp::Symbol("define".to_string()),
                              Exp::Symbol("x".to_string()),
                              Exp::Integer(8)), ENV.clone());
            let assignment = scheme_list!(Exp::Symbol("define".to_string()), 
                                              Exp::Symbol("x".to_string()), Exp::Integer(101));
            let x = eval(assignment.clone(), ENV.clone()); 
            assert_eq!(eval(Exp::Symbol("x".to_string()), ENV.clone()).unwrap().unwrap(), Exp::Integer(101));
            
            let definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                         Exp::Symbol("x".to_string()), 
                                         Exp::Integer(999));
            let s = eval(definition.clone(), ENV.clone()); 
            let second_definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                                Exp::Symbol("y".to_string()), 
                                                Exp::Integer(333));
            let t = eval(second_definition, ENV.clone()); 
            assert_eq!(eval(Exp::Symbol("x".to_string()), ENV.clone()).unwrap().unwrap(), 
                   Exp::Integer(999));
            assert_eq!(eval(Exp::Symbol("y".to_string()), ENV.clone()).unwrap().unwrap(),  
                   Exp::Integer(333));
        }
    }

    #[test]
    fn test_eval_definition_compoud_procedure() {
        unsafe {
        eval(scheme_list!(Exp::Symbol("define".to_string()),
                              Exp::Symbol("x".to_string()),
                              Exp::Integer(8)), ENV.clone()); 
        let assignment = scheme_list!(Exp::Symbol("define".to_string()), 
                                          Exp::Symbol("x".to_string()), 
                                          Exp::Integer(101));
        let x = eval(assignment.clone(), ENV.clone()); 
        let definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                         Exp::Symbol("x".to_string()), Exp::Integer(999));
        let y = eval(definition, ENV.clone()); 
        let second_definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                                Exp::Symbol("y".to_string()), 
                                                Exp::Integer(333));
        let z= eval(second_definition, ENV.clone()); 
        let another_definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                    scheme_list!(Exp::Symbol("square".to_string()),
                                                 Exp::Symbol("x".to_string())),
                                    scheme_list!(Exp::Symbol("*".to_string()),
                                                 Exp::Symbol("x".to_string()),
                                                 Exp::Symbol("x".to_string())));
        let s = eval(another_definition.clone(), ENV.clone());
        let app_exp = scheme_list!(Exp::Symbol("square".to_string()), 
                                       Exp::Integer(3));
        assert_eq!(eval(app_exp, ENV.clone()).unwrap().unwrap(), Exp::FloatNumber(9.0));
        }
   }

    #[test]
    fn test_eval_primitive_procedure() {
        unsafe {
            eval(scheme_list!(Exp::Symbol("define".to_string()),
            Exp::Symbol("x".to_string()),
            Exp::Integer(8)), ENV.clone()); 
            let assignment = scheme_list!(Exp::Symbol("define".to_string()), 
                                              Exp::Symbol("x".to_string()), 
                                              Exp::Integer(101));
            let x = eval(assignment.clone(), ENV.clone()); 
            let definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                              Exp::Symbol("x".to_string()), 
                                              Exp::Integer(999));
            let y = eval(definition, ENV.clone()); 
            let second_definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                                    Exp::Symbol("y".to_string()), 
                                                    Exp::Integer(333));
            let z= eval(second_definition, ENV.clone()); 
            let another_definition = scheme_list!(Exp::Symbol("define".to_string()), 
                                        scheme_list!(Exp::Symbol("square".to_string()),
                                                     Exp::Symbol("x".to_string())),
                                        scheme_list!(Exp::Symbol("*".to_string()),
                                                     Exp::Symbol("x".to_string()),
                                                     Exp::Symbol("x".to_string())));
            let t = eval(another_definition.clone(), ENV.clone());
            let s = scheme_list!(scheme_list!(Exp::Integer(1), Exp::Integer(2)),
                                     scheme_list!(Exp::Integer(3), Exp::Integer(4)),
                                                  Exp::Integer(5));
            let s_definition = scheme_list!(Exp::Symbol("define".to_string()),
                                               Exp::Symbol("s".to_string()),
                                                s);
            let q = eval(s_definition.clone(),ENV.clone());
            let car_exp = scheme_list!(Exp::Symbol("car".to_string()), 
                                           Exp::Symbol("s".to_string()));
            assert_eq!(eval(car_exp, ENV.clone()).unwrap().unwrap(), 
            scheme_list!(Exp::Integer(1), Exp::Integer(2)));
            let cdr_exp = scheme_list!(Exp::Symbol("cdr".to_string()), 
                                           Exp::Symbol("s".to_string()));
            assert_eq!(eval(cdr_exp, ENV.clone()).unwrap().unwrap(),
            scheme_list!(scheme_list!(Exp::Integer(3), 
                                      Exp::Integer(4)),
                                      Exp::Integer(5)));
            let null_exp = scheme_list!(Exp::Symbol("null?".to_string()), 
                                            Exp::List(Pair::Nil));
            assert_eq!(eval(null_exp.clone(), ENV.clone()).unwrap().unwrap(), Exp::Bool(true));

            let add_exp = scheme_list!(Exp::Symbol("+".to_string()), 
                                           Exp::FloatNumber(3.15),
                                           Exp::FloatNumber(1.85));
            assert_eq!(eval(add_exp.clone(), ENV.clone()).unwrap().unwrap(), 
                            Exp::FloatNumber(5.0));

            let substract_exp = scheme_list!(Exp::Symbol("-".to_string()), 
                                                 Exp::Integer(8), 
                                                 Exp::FloatNumber(2.5));
            assert_eq!(eval(substract_exp.clone(), ENV.clone()).unwrap().unwrap(), 
                            Exp::FloatNumber(5.5));

            let multiply_exp = scheme_list!(Exp::Symbol("*".to_string()), 
                                                Exp::FloatNumber(2.5),
                                                Exp::FloatNumber(2.5));
            assert_eq!(eval(multiply_exp.clone(), ENV.clone()).unwrap().unwrap(), 
                            Exp::FloatNumber(6.25));

            let divide_exp = scheme_list!(Exp::Symbol("/".to_string()), 
                                              Exp::FloatNumber(25.0),
                                              Exp::FloatNumber(2.5));
            assert_eq!(eval(divide_exp.clone(), ENV.clone()).unwrap().unwrap(), 
                            Exp::FloatNumber(10.0));

            let cons_exp = scheme_list!(Exp::Symbol("cons".to_string()), 
                               scheme_list!(Exp::Integer(1), Exp::Integer(2)),
                               scheme_list!(Exp::Integer(3), Exp::Integer(4)));
            assert_eq!(eval(cons_exp.clone(), ENV.clone()).unwrap().unwrap(), 
            scheme_list!(scheme_list!(Exp::Integer(1), Exp::Integer(2)), 
                                      Exp::Integer(3), Exp::Integer(4)));
        }        
    }
}