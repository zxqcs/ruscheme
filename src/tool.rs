#![allow(unused_variables)]
pub mod tools {
use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
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
}
#[cfg(test)]
mod test {
    use super::tools::{scheme_cons, append};
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair::*};
    use crate::represent::represent::{cadr, cddr, caadr};
    #[test]
    fn test_scheme_cons() {
    // (lambda (x) (+ x x))
    // ((lambda (x) (+ x x)) 4) =>  8
    // lambda prameters: (x) 
    // lambda body: ((+ x x))
        let plus = Exp::Symbol("+");
        let x = Exp::Symbol("x");
        let null = Exp::List(Nil);
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
        let null = Exp::List(Nil);
        let n1 = Exp::Integer(1);
        let n2 = Exp::Integer(2);
        let n3 = Exp::Integer(3);
        let s1 = scheme_cons(n3.clone(), null);
        let s2 = scheme_cons(n1.clone(), n2.clone());
        let s3 = scheme_cons(s2.clone(), s1.clone());
        assert_eq!(s3, append(s2, s1));
    }
}