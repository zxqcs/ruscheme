
pub mod env {
    use crate::{represent::represent::*, scheme_list, tool::tools::list_length};
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
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
    pub fn extend_environment(vars: Exp, vals: Exp, base_env: Exp) -> Result<Exp, &'static str> {
        if list_length(vars.clone()) == list_length(vals.clone()) {
            Ok(scheme_cons(make_frame(vars, vals), 
                    base_env))
        } else {
            Err("number of args mismatch!")
        }
    }

    #[allow(dead_code)]
    pub fn enclosing_environment(env: Exp) -> Exp { 
        cdr(env).unwrap()
    }

    #[allow(dead_code)]
    pub fn first_frame(env: Exp) -> Exp {
        car(env).unwrap()
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
    pub fn lookup_varaible_value(var: Exp, env: Exp) -> Exp {
        if env == THE_EMPTY_ENVIRONMENT {
            panic!("unbound variable");
        } else {
            let frame = first_frame(env.clone());
            let s = scan(frame_variables(frame.clone()), 
                                         frame_values(frame.clone()), var.clone());
            match s {
                Some(x) => x,
                None => {
                    let enclosing_environment = enclosing_environment(env);
                    lookup_varaible_value(var, enclosing_environment)
                },
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_variable_value(var: Exp, val: Exp, env: Exp) {}

    #[allow(dead_code)]
    pub fn define_variable(var: Exp, val: Exp, env: Exp) {}

}

#[cfg(test)]
mod test {
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    use crate::tool::tools::{append, scheme_cons, generate_test_frames};
    use crate::scheme_list;
    use super::env::{add_binding_to_frame, frame_values, frame_variables, make_frame};


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
        let a = Exp::Symbol("a");
        let four = Exp::Integer(4);
        let data = generate_test_frames();
        let s = add_binding_to_frame(a, four, data.frame);
        assert_eq!(s, data.extended_frame); 
    }
}