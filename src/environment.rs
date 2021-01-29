
pub mod env {
    use crate::represent::represent::*;
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    use crate::tool::tools::{scheme_cons, append};

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
        Exp::Symbol("yes")
    }

}

#[cfg(test)]
mod test {
    #[test]
    fn test_make_frmae() {
        assert_eq!(1, 1);
    }
}