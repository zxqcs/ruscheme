
pub mod env {
    use crate::{represent::represent::*, scheme_list, tool::tools::{list_length, append}};
    use crate::core_of_interpreter::core_of_interpreter::{Env, Exp, Pair};
    use crate::tool::tools::{scheme_cons, set_cdr, set_car};
     
    const THE_EMPTY_ENVIRONMENT: Exp = Exp::List(Pair::Nil);

    // frame operatons
    #[allow(dead_code)]
    pub fn make_frame(variables: Exp, values: Exp) -> Exp {
        scheme_cons(variables, values) 
    }

    #[allow(dead_code)]
    pub fn frame_variables(frame: Exp) -> Exp { 
        car(frame).unwrap()
    } 

    #[allow(dead_code)]
    pub fn frame_values(frame: Exp) -> Exp {
        cdr(frame).unwrap()
    }
    
    #[allow(dead_code)]
    pub fn add_binding_to_frame(var: Exp, val: Exp, frame: Exp) -> Exp {
        let temp = set_car(frame.clone(), 
            scheme_cons(var, frame_variables(frame.clone()))).unwrap();
        set_cdr(temp, scheme_cons(val, frame_values(frame.clone()))).unwrap()
    }

    // environment operatons
    #[allow(dead_code)]
    pub fn extend_environment(vars: Exp, vals: Exp, base_env :Env) -> Env {
        if list_length(vars.clone()) == list_length(vals.clone()) {
            let env = scheme_cons(make_frame(vars, vals), 
                                      base_env.0);
            Env(env)
        } else {
            panic!("number of args mismatch!")
        }
    }

    #[allow(dead_code)]
    pub fn enclosing_environment(env: Env) -> Env { 
        Env(cdr(env.0).unwrap())
    }

    #[allow(dead_code)]
    pub fn first_frame(env: Env) -> Exp {
        car(env.0).unwrap()
    }

    #[allow(dead_code)]
    fn scan(vars: Exp, vals: Exp, target: Exp) -> Option<Exp> {
        let null = Exp::List(Pair::Nil);
        if vars == null {
            None
        } else if target == car(vars.clone()).unwrap() {
            Some(car(vals).unwrap())
        } else {
            scan(cdr(vars.clone()).unwrap(), cdr(vals.clone()).unwrap(), target)
        }
    }

    #[allow(dead_code)]
    pub fn lookup_variable_value(var: Exp, env: Env) -> Exp {
        if env.0 == THE_EMPTY_ENVIRONMENT {
            println!("var: {:?}", var);
            panic!("unbound variable");
        } else {
            let frame = first_frame(env.clone());
            let s = scan(frame_variables(frame.clone()), 
                                         frame_values(frame.clone()), var.clone());
            match s {
                Some(x) => x,
                None => {
                    let enclosing_environment = enclosing_environment(env);
                    lookup_variable_value(var, enclosing_environment)
                },
            }
        }
    }

    #[allow(dead_code)]
    pub fn scan_and_set(vars: Exp, vals: Exp, target_var: Exp, target_val: Exp, tag: &mut bool) -> Exp {
        let null = Exp::List(Pair::Nil);
        if vars == null {
            null
        } else if target_var == car(vars.clone()).unwrap() {
            *tag = true;
            set_car(vals, target_val).unwrap()
        } else {
            let temp_vals = scan_and_set(cdr(vars.clone()).unwrap(), 
                           cdr(vals.clone()).unwrap(), target_var, target_val, tag);
            set_cdr(vals, temp_vals).unwrap()
        }
    }

    #[allow(dead_code)]
    pub fn set_variable_value(var: Exp, val: Exp, env: Env) -> Env {
        if env.0 == THE_EMPTY_ENVIRONMENT {
            panic!("unbound variable: SET!");
        } else {
            let mut tag = false;
            let frame = first_frame(env.clone());
            let s = scan_and_set(frame_variables(frame.clone()), 
                                            frame_values(frame.clone()), 
                                       var.clone(),
                                       val.clone(),
                                                 &mut tag);
            if tag {
                let temp_frame = set_cdr(frame, s).unwrap();
                Env(set_car(env.0, temp_frame).unwrap())
            } else {
                let enclosing_env = enclosing_environment(env.clone());
                let temp_env = set_variable_value(var, val, enclosing_env);
                Env(set_cdr(env.0, temp_env.0).unwrap())
            }
        }
    }

    #[allow(dead_code)]
    fn scan_and_define(target_var: Exp, target_val: Exp, frame: Exp) -> Exp {
       let vars = frame_variables(frame.clone());
       let vals = frame_values(frame.clone());
       if vars == Exp::List(Pair::Nil) {
           add_binding_to_frame(target_var, target_val, frame)
        } else if target_var == car(vars.clone()).unwrap() {
                let temp_vals = set_car(vals, target_val).unwrap();
                make_frame(vars, temp_vals)
            } else {
                let mut temp_frame = make_frame(cdr(vars.clone()).unwrap(), 
                                                cdr(vals.clone()).unwrap());
                temp_frame = scan_and_define(target_var, target_val, temp_frame);
                let temp_vars = set_cdr(vars,frame_variables(temp_frame.clone())).unwrap();
                let temp_vals = set_cdr(vals,frame_values(temp_frame.clone())).unwrap();
                make_frame(temp_vars, temp_vals)
            }
        }

    #[allow(dead_code)]
    pub fn define_variable(target_var: Exp, target_val: Exp, env: Env) -> Env {
        if env.0 == Exp::List(Pair::Nil) {
            let frame = scheme_list!(scheme_list!(target_var), target_val);
            Env(scheme_list!(frame))
        } else {
            let frame = first_frame(env.clone());
            let temp_frame = scan_and_define(target_var, target_val, frame);
            Env(set_car(env.0, temp_frame).unwrap())
        }
    } 
}

#[cfg(test)]
mod test {
    use crate::{core_of_interpreter::core_of_interpreter::{Env, Exp, Pair}};
    use crate::tool::tools::{append, scheme_cons, generate_test_frames};
    use crate::scheme_list;
    use super::env::{add_binding_to_frame, define_variable, frame_values, frame_variables, lookup_variable_value, make_frame, set_variable_value};


    #[test]
    fn test_make_frame() {
       let data = generate_test_frames();
        assert_eq!(data.frame, make_frame(data.variables, data.values));
    }

    #[test]
    fn test_frame_variables() {
        let data = generate_test_frames();
        assert_eq!(data.variables, frame_variables(data.frame));
    }

    #[test]
    fn test_frame_values() {
        let data = generate_test_frames();
        assert_eq!(data.values, frame_values(data.frame));
    }

    #[test]
    fn test_add_binding_to_frame() {
        let a = Exp::Symbol("a".to_string());
        let four = Exp::Integer(4);
        let data = generate_test_frames();
        let s = add_binding_to_frame(a, four, data.frame);
        assert_eq!(s, data.extended_frame); 
    }

    #[test]
    fn test_lookup_variable_value() {
        let frame_one = generate_test_frames().frame;
        let u = Exp::Symbol("u".to_string());
        let v = Exp::Symbol("v".to_string());
        let four = Exp::Integer(4);
        let five = Exp::Integer(5);
        let frame_two = scheme_list!(scheme_list!(u, v), four, five);
        let test_env = Env(scheme_list!(frame_one, frame_two));
        assert_eq!(lookup_variable_value(Exp::Symbol("u".to_string()), test_env.clone()), Exp::Integer(4));
        assert_eq!(lookup_variable_value(Exp::Symbol("v".to_string()), test_env.clone()), Exp::Integer(5));
        assert_eq!(lookup_variable_value(Exp::Symbol("z".to_string()), test_env.clone()), Exp::Integer(3));
        assert_eq!(lookup_variable_value(Exp::Symbol("x".to_string()), test_env), Exp::Integer(1));
    }
    #[test]
    fn test_set_variable_value() {
        let frame_one = generate_test_frames().frame;
        let u = Exp::Symbol("u".to_string());
        let v = Exp::Symbol("v".to_string());
        let four = Exp::Integer(4);
        let five = Exp::Integer(5);
        let frame_two = scheme_list!(scheme_list!(u, v), four, five);
        let test_env = Env(scheme_list!(frame_one, frame_two));
        let modified_env = set_variable_value(Exp::Symbol("v".to_string()),
                                                        Exp::Integer(0),  
                                                            test_env.clone());
        let another_env = set_variable_value(Exp::Symbol("u".to_string()), 
                                                     Exp::Integer(9), modified_env.clone());
        let one_more_env = set_variable_value(Exp::Symbol("z".to_string()), Exp::Integer(1000), another_env.clone());
        assert_eq!(lookup_variable_value(Exp::Symbol("v".to_string()), modified_env), Exp::Integer(0));
        assert_eq!(lookup_variable_value(Exp::Symbol("u".to_string()), another_env), Exp::Integer(9));
        assert_eq!(lookup_variable_value(Exp::Symbol("z".to_string()), one_more_env.clone()), Exp::Integer(1000));
    }

    #[test]
    fn test_define_variable() {
        let frame_one = generate_test_frames().frame;
        let u = Exp::Symbol("u".to_string());
        let v = Exp::Symbol("v".to_string());
        let four = Exp::Integer(4);
        let five = Exp::Integer(5);
        let frame_two = scheme_list!(scheme_list!(u, v), four, five);
        let mut test_env = Env(scheme_list!(frame_one, frame_two));
        test_env = define_variable(Exp::Symbol("x".to_string()), Exp::Integer(0), test_env);
        assert_eq!(lookup_variable_value(Exp::Symbol("x".to_string()), test_env.clone()), Exp::Integer(0));
        test_env = define_variable(Exp::Symbol("s".to_string()), Exp::Integer(101), test_env);
        assert_eq!(lookup_variable_value(Exp::Symbol("s".to_string()), test_env.clone()), Exp::Integer(101));
        test_env = define_variable(Exp::Symbol("y".to_string()), Exp::Integer(333), test_env);
        assert_eq!(lookup_variable_value(Exp::Symbol("y".to_string()), test_env.clone()), Exp::Integer(333));
        let mut another_env = define_variable(Exp::Symbol("p".to_string()), Exp::Integer(100), 
                                         Env(Exp::List(Pair::Nil)));
        assert_eq!(lookup_variable_value(Exp::Symbol("p".to_string()), another_env.clone()), Exp::Integer(100));
        another_env = define_variable(Exp::Symbol("d".to_string()), Exp::Integer(27), another_env.clone());
        assert_eq!(lookup_variable_value(Exp::Symbol("d".to_string()), another_env.clone()), Exp::Integer(27));
        another_env = define_variable(Exp::Symbol("g".to_string()), Exp::Integer(81), another_env.clone());
        assert_eq!(lookup_variable_value(Exp::Symbol("g".to_string()), another_env.clone()), Exp::Integer(81));
    }
/* 
    #[test]
    fn test_extend_environment() {
       let data = generate_test_frames();
       let vars = data.variables;
       let vals = data.values;
       let frame = data.frame;
       let env = Env(scheme_list!(frame));
       extend_environment(vars, vals);
       unsafe{
           assert_eq!(env, ENV);
       }
    }
*/
}