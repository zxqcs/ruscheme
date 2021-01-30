
pub mod env {
    use crate::represent::represent::*;
    use crate::core_of_interpreter::core_of_interpreter::{Exp};
    use crate::tool::tools::{scheme_cons, set_cdr, set_car};

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