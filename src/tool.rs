#![allow(unused_variables)]
pub mod tools {
use crate::{core_of_interpreter::core_of_interpreter::{Exp, Pair}, scheme_list};
use crate::represent::represent::{car, cdr};

#[allow(dead_code)]
    pub fn scheme_cons(lhs: Exp, rhs: Exp) -> Exp {
        match rhs {
            Exp::List(x) => {
                let s1 = Box::new(lhs);
                let s2 = Box::new(x);
                let s3 = Pair::Cons(s1, s2);
                Exp::List(s3)
            },
            _ => {
                let s1 = Box::new(Pair::Nil);
                let s2 = Box::new(rhs);
                let s3 = Pair::Cons(s2, s1); 
                let s4 = Box::new(s3);
                let s5 = Box::new(lhs);
                Exp::List(Pair::Cons(s5, s4))
            },
        }
    }

#[allow(dead_code)]
    pub fn append(lhs: Exp, rhs: Exp) -> Exp {
        let null = Exp::List(Pair::Nil);
        if lhs == null {
            rhs
        } else {
            scheme_cons(car(lhs.clone()).unwrap(), 
                    append(cdr(lhs.clone()).unwrap(), rhs))
        }
    }

#[allow(dead_code)]
    pub fn set_car(x: Exp, y: Exp) -> Result<Exp, &'static str> {
        if let Exp::List(Pair::Cons(lhs, rhs)) = x {
            Ok(Exp::List(Pair::Cons(Box::new(y), rhs)))
        } else {
            Err( "error happens!")
        }
    }

#[allow(dead_code)]
    pub fn set_cdr(x: Exp, y: Exp) -> Result<Exp, &'static str> {
        if let Exp::List(Pair::Cons(lhs, rhs)) = x {
            Ok(scheme_cons(*lhs, y))
        } else {
            Err("error happens!")
        }
    }
#[allow(dead_code)]
    pub fn list_length(exp: Exp) -> i32 {
        if exp == Exp::List(Pair::Nil) {
            0
        } else {
            1 + list_length(cdr(exp.clone()).unwrap())
        }
    }
    

#[derive(Debug, Clone)] 
    pub struct TestData{
        pub if_expression: Exp,
        pub begin_expression: Exp,
        pub applicatioin_expressioin: Exp,
        pub lambda_expression: Exp,
    }
        
#[allow(dead_code)]
    pub fn generate_test_data() -> TestData {
        // (if (= n 1)  1  (-  n  1)) 
        let s1 = Exp::Symbol("if");
        let s2 = Exp::Symbol("n");
        let s3 = Exp::Integer(1);
        let s4 = Exp::Symbol("-");
        let s5 = Exp::Symbol("=");
        let x1 = scheme_list!(s5, s2.clone(), s3.clone());
        let x2 = scheme_list!(s4, s2.clone(), s3.clone());
        let if_exp = scheme_list!(s1, x1, s3.clone(), x2);
       
        // (begin (set! x 5) (+ x 1))
        let t1 = Exp::Integer(5);
        let t2 = Exp::Integer(1);
        let t3 = Exp::Symbol("set!");
        let t4 = Exp::Symbol("begin");
        let t5 = Exp::Symbol("x");
        let t6 = Exp::Symbol("+");
        let y1 = scheme_list!(t3, t5.clone(), t1);
        let y2 = scheme_list!(t6, t5.clone(), t2);
        let begin_exp = scheme_list!(t4, y1, y2);

        // (lambda (x) (* x x))
        let r1 = Exp::Symbol("lambda");
        let r2 = Exp::Symbol("x");
        let r3 = Exp::Symbol("*");
        let null =  Exp::List(Pair::Nil);
        let r4 = scheme_cons(r2.clone(), null);
        let r5 = scheme_list!(r3, r2.clone(), r2.clone());
        let lambda_exp = scheme_list!(r1, r4, r5);

        // (procedure 3 4)
        let p1 = Exp::Symbol("procedure");
        let p2 = Exp::Integer(3);
        let p3 = Exp::Integer(4);
        let app_exp = scheme_list!(p1, p2, p3);
        
        let data = TestData {
            if_expression: if_exp,
            begin_expression: begin_exp,
            applicatioin_expressioin: app_exp,
            lambda_expression: lambda_exp,
        };
        data
    }

    pub struct Frames {
        pub variables: Exp,
        pub values: Exp,
        pub frame: Exp,
        pub extended_frame: Exp,
    }

#[allow(dead_code)]
    impl Frames {
        fn new(variables: Exp, values: Exp, frame: Exp, extended_frame:Exp) -> Self{
            Frames {
                variables: variables,
                values: values,
                frame: frame,
                extended_frame: extended_frame,
            }
        }
    }

#[allow(dead_code)]
    pub fn generate_test_frames() -> Frames {
        let x = Exp::Symbol("x");
        let y = Exp::Symbol("y");
        let z = Exp::Symbol("z");
        let one = Exp::Integer(1);
        let two = Exp::Integer(2);
        let three = Exp::Integer(3);
        let a = Exp::Symbol("a");
        let four = Exp::Integer(4);
        let variables = scheme_list!(x.clone(), y.clone(), z.clone());
        let values = scheme_list!(one.clone(), two.clone(), 
                                                   three.clone());
        let extended_variables = scheme_list!(a, x, y, z);
        let extended_values = scheme_list!(four, one.clone(), two.clone(), three.clone());
        let frame = scheme_cons(variables.clone(), scheme_list!(one, two, three));
        let extended_frame= scheme_cons(extended_variables, extended_values);
        Frames::new(variables, values, frame, extended_frame)
    }
}
#[cfg(test)]
mod test {
    use super::tools::{append, list_length, scheme_cons, set_car, set_cdr};
    use crate::{core_of_interpreter::core_of_interpreter::{Exp, Pair}, scheme_list};
    use crate::represent::represent::{cadr, cddr, caadr};
    #[test]
    fn test_scheme_cons() {
    // (lambda (x) (+ x x))
    // ((lambda (x) (+ x x)) 4) =>  8
    // lambda prameters: (x) 
    // lambda body: ((+ x x))
        let plus = Exp::Symbol("+");
        let x = Exp::Symbol("x");
        let null = Exp::List(Pair::Nil);
        //  parameters: (x)
        let parameters = scheme_cons(x.clone(), null.clone());
        // (x)
        let s1 = scheme_cons(x.clone(), null.clone());
        // (x x)
        let s2 = scheme_cons(x.clone(), s1.clone());
        // (+ x x)
        let s3 = scheme_cons(plus.clone(), s2.clone());
        // body: ((+ x x))
        let body = scheme_cons(s3.clone(), null.clone());
        // exp: ((x) (+ x x))
        let exp = scheme_cons(parameters.clone(), body.clone());
        assert_eq!(cadr(exp.clone()).unwrap(), s3.clone());
        assert_eq!(cddr(exp.clone()).unwrap(), null.clone());
        assert_eq!(caadr(exp.clone()).unwrap(), plus.clone());
    }

    #[test]
    fn test_append() {
        let null = Exp::List(Pair::Nil);
        let n1 = Exp::Integer(1);
        let n2 = Exp::Integer(2);
        let n3 = Exp::Integer(3);
        let s1 = scheme_cons(n3.clone(), null);
        let s2 = scheme_cons(n1.clone(), n2.clone());
        let s3 = scheme_cons(n1.clone(), 
                   scheme_cons(n2.clone(), n3.clone()));
        assert_eq!(s3, append(s2, s1));
    }

    #[test]
    fn test_set_cdr() {
        // ("hello" "world")  -> ("hello" "fool")
        let hello = Exp::Symbol("hello");
        let world = Exp::Symbol("world");
        
        let fool = Exp::Symbol("fool");
        let s1 = scheme_list!(hello.clone(), world);
        let s2 = scheme_list!(hello.clone(), fool.clone());
        assert_eq!(s2, set_cdr(s1, fool).unwrap());
    }

    #[test]
    fn test_set_car() {
        let hello = Exp::Symbol("hello");
        let world = Exp::Symbol("world");
        let fool = Exp::Symbol("fool");
         
        let s1 = scheme_list!(hello.clone(), world.clone());
        let s2 = scheme_list!(fool.clone(), world.clone());
        assert_eq!(s2, set_car(s1, fool).unwrap());
    }

    #[test]
    fn test_list_length() {
        let s1 = scheme_list!(Exp::Integer(1));
        let s2 = scheme_list!(Exp::Integer(2), Exp::Integer(3));
        let s3 = scheme_list!(Exp::Integer(4), Exp::Integer(5));
        let s = scheme_list!(s1, s2, s3);
        assert_eq!(list_length(s), 3);
        assert_eq!(list_length(Exp::List(Pair::Nil)), 0);
    }
}