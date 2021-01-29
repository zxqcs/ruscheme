
pub mod env {
    use crate::represent::represent::*;
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    use crate::tool::tools::{scheme_cons, append, set_cdr, set_car};

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
            scheme_cons(var, car(frame.clone()).unwrap())).unwrap();
        set_cdr(temp, scheme_cons(val, cdr(frame).unwrap())).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    use crate::tool::tools::{append, scheme_cons};
    use crate::scheme_list;
    use super::env::make_frame;

    struct Frames {
        variables: Exp,
        values: Exp,
        frame: Exp,
    }

    impl Frames {
        fn new(variables: Exp, values: Exp, frame: Exp) -> Self{
            Frames {
                variables: variables,
                values: values,
                frame: frame,
            }
        }
    }
    fn generate_test_frames() -> Frames {
        let x = Exp::Symbol("x");
        let y = Exp::Symbol("y");
        let z = Exp::Symbol("z");
        let one = Exp::Integer(1);
        let two = Exp::Integer(2);
        let three = Exp::Integer(3);
        let variables = scheme_list!(x, y, z);
        let values = scheme_list!(scheme_list!(one.clone(), two.clone(), 
                                                   three.clone()));
        let frame = scheme_list!(variables.clone(), scheme_list!(one, two, three));
        Frames::new(variables, values, frame)
    }
    #[test]
    fn test_make_frame() {
       let data = generate_test_frames();
        assert_eq!(data.frame, make_frame(data.variables, data.values));
    }
}