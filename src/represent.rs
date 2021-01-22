
#![allow(unused_variables)]
pub mod  represent{
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    /* operatons on Exp as enum methods */
    #[allow(dead_code)]
    impl Exp {
        pub fn is_pair(&self) -> bool { 
            match self {
                Exp::List(x) => {
                    match x {
                        Pair::Nil => false,
                        _ => true,
                    }
                }
                    _ => false,
            }
        }

        pub fn is_variable(&self) -> bool { 
            self.is_symbol()
        }

        pub fn is_quoted(&self) -> bool { 
            match self {
                Exp::Quote(_x) => true,
                _ => false,   
            }
        }

        pub fn is_string(&self) -> bool {
            match self {
                Exp::SchemeString(_x) => true,
                _ => false,
            }
        }
        
        pub fn is_symbol(&self) -> bool { 
            match self {
                Exp::Symbol(_x) => true,
                _ => false,
            }
        }

        pub fn is_number(&self) -> bool { 
            match self {
                Exp::FloatNumber(_x) => true,
                Exp::Integer(_x) => true,
                _ => false,
            }        
        }

        pub fn is_self_evaluating(&self) -> bool {
            self.is_number() || self.is_string()
        }

        /* operations on Exp as function */
        pub fn is_assignment(exp: Exp) -> bool { 
            is_tagged_list(exp, "set!")
        }

        #[allow(dead_code)]
        pub fn assignment_variable(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn assignment_value(exp: Exp) -> Exp {
            caddr(exp).unwrap()
        }

        #[allow(dead_code)]
        pub fn is_definiton(exp: Exp) -> bool { 
            is_tagged_list(exp, "define")
        }

        #[allow(dead_code)]
        pub fn definition_variable(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }
        

        #[allow(dead_code)]
        pub fn definition_value(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn is_lambda(exp: Exp) -> bool { 
            is_tagged_list(exp, "lambda")
        }

        #[allow(dead_code)]
        pub fn lambda_parameters(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn lambda_body(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn make_lambda (parameters: Exp, body: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }
        
        #[allow(dead_code)]
        pub fn is_if(exp: Exp) -> bool { 
            is_tagged_list(exp, "if")
        }

      
        #[allow(dead_code)]
        pub fn if_predicate(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn if_consequent(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }


        #[allow(dead_code)]
        pub fn if_alternative(exp: Exp) -> Exp {
            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn make_if(predicate: Exp, consequent: Exp, alternative: Exp) -> Exp{
            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn is_begin(exp: Exp) -> bool { 
            is_tagged_list(exp, "begin")
        }
        
        #[allow(dead_code)]
        pub fn begin_actions(exp: Exp) -> Exp {

            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn is_last_exp(seq: Exp) -> bool {
            true
        }

        #[allow(dead_code)]
        pub fn is_first_exp(seq: Exp) -> bool {
            true
        }

        #[allow(dead_code)]
        pub fn rest_exps(seq: Exp) -> Exp {

            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn sequence_to_exp(seq: Exp) -> Exp {

            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn make_begin(seq: Exp) -> Exp {

            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn is_application(exp: Exp) -> bool { 
            exp.is_pair()
        }

        #[allow(dead_code)]
        pub fn operator(exp: Exp) -> Exp {

            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn operands(exp: Exp) -> Exp {

            Exp::List(Pair::Nil)
        }

        #[allow(dead_code)]
        pub fn no_operands(ops: Exp) -> bool {
            true
        }

        #[allow(dead_code)]
        pub fn first_operand(ops: Exp) -> Exp {
            car(ops).unwrap()
        }

        #[allow(dead_code)]
        pub fn rest_operands(ops: Exp) -> Exp {
            cdr(ops).unwrap()
        }
/* note that cond related procedures are ommited */
    }
    /* operations on List variant of Exp */
    pub fn car(exp: Exp) -> Result<Exp, &'static str> {
        match &exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                     if let Exp::List(Pair::Cons(x, _y)) = exp { 
                        Ok(*x.clone()) } else {
                            Err("error happens!")
                        }
                } else {Err("not a pair!")}
            }
            _ => Err("type mismatch, not even a List!")
        }
    }
   
    #[allow(dead_code)]
    pub fn cdr(exp: Exp) -> Result<Exp, &'static str> {
        match &exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                    if let Exp::List(Pair::Cons(_x, y)) = exp { 
                        let z = Exp::List(*y);
                        Ok(z )} else {
                            Err("error happens!")
                        }
                } else {Err("not a pair!")}
            }
            _ => Err("type mismatch, not even a List!")
        }
    }
   
    #[allow(dead_code)]
    pub fn cadr(exp: Exp) -> Result< Exp, &'static str> {
        match &exp {
            Exp::List(_x) => {
                if exp.is_pair() {
                    if let Exp::List(Pair::Cons(_x, y)) = exp { 
                        if let Pair::Cons(a, _b) = *y {
                            Ok(*a.clone())
                        }
                        else {
                            Err("error happens!")
                        }
                    } else {Err("not a pair!")}
            } else { Err ("type mismatch, not a proper List!")} 
        },
            _ => Err("type mismatch, not even a List!")
        }
    }

    #[allow(dead_code)]
    pub fn cddr(exp: Exp) -> Result< Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        cdr(s1)
    }

    #[allow(dead_code)]
    pub fn cdddr(exp: Exp) -> Result< Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        let s2 = cdr(s1).unwrap();
        cdr(s2)
    }
    
    #[allow(dead_code)]
    pub fn cadddr(exp: Exp) -> Result< Exp, &'static str> {
        let s1 = cdddr(exp).unwrap();
        car(s1)
    }

    #[allow(dead_code)]
    pub fn caddr(exp: Exp) -> Result< Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        let s2 = cdr(s1).unwrap();
        car(s2)
    }

    #[allow(dead_code)]
    pub fn caadr(exp: Exp) -> Result< Exp, &'static str> {
        let s1 = cdr(exp).unwrap();
        let s2 = car(s1).unwrap();
        car(s2)
    }

    #[allow(dead_code)]
    pub fn cdadr(exp: Exp) -> Result< Exp, &'static str> {
        let s1 = cadr(exp).unwrap();
        let s2 = cdr(s1);
        s2
    }
/* 
    #[allow(dead_code)]
    pub fn scheme_cons<'a>(lhs: Exp<'a>, rhs: Exp<'a>) -> Exp<'a> {
        match rhs {
            Exp::List(&Pair::Nil) => {
                let s1 = Box::new(&Pair::Nil);
                let s2 = Box::new(&lhs);
                Exp::List(&Pair::Cons(s2, s1))
            }
            _ => {
                let d = Box::new(&Pair::Nil);
                let s1 = Box::new(&rhs);
                let s2 = Pair::Cons(s1, d);
                Exp::List(&s2) 
            }
        }
    }

*/
    #[allow(dead_code)]
    pub fn is_tagged_list(exp: Exp, tag: &'static str) -> bool {
        if exp.is_pair() {
            if let Ok(Exp::Symbol(x)) = car(exp) {
                match x {
                    t if t == tag => true,
                    _ => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair::*};
    use super::represent::*;
    #[test]
    fn test_is_number() {
        let x = Exp::Integer(3);
        assert_eq!(x.is_number(), true);
    }

    #[test] 
    fn test_is_string() {
        let str = "summer";
        let x = Exp::Symbol(str);
        assert_eq!(x.is_symbol(), true);
    }

    #[test]
    fn test_is_self_evaluating() {
        let x = Exp::FloatNumber(3.14);
        let y = Exp::SchemeString("Winter");
        assert_eq!(x.is_self_evaluating() && y.is_self_evaluating(), true);
    }

    #[test]
    fn test_is_symbol() {
        let x = Exp::Symbol("item");
        assert_eq!(x.is_symbol(), true);
    }

    #[test]
    fn test_is_pair() {
        let a = Box::new(Exp::Integer(1));
        let b = Box::new(Exp::Integer(2));
        let c = Box::new(Exp::Integer(3));
        let d = Box::new(Nil); 
        let x = Cons(c, d);
        let y = Cons(b, Box::new(x));
        let z = Cons(a, Box::new(y));
        let s = Exp::List(z);
        assert_eq!(s.is_pair(), true);
    }

    #[test]
    fn test_is_quoted() {
        let x = Exp::Quote("'x");
        assert_eq!(x.is_quoted(), true);
    }

    #[test]
    fn test_list_operatioins() {
        // It's painful to build List in Rust...
        // (define (square x) (* x  x))
        let f1 = Box::new(Exp::Symbol("define"));
        let y = Box::new(Exp::Symbol("square"));
        let z = Box::new(Exp::Symbol("x"));
        let a = Box::new(Exp::Symbol("*"));
        let b = Box::new(Exp::Symbol("x"));
        let c = Box::new(Exp::Symbol("x"));
        let d1 = Box::new(Nil);
        let d2 = Box::new(Nil);
        let d3 = Box::new(Nil);
        // represent (* x x)
        let s1 = Cons(c, d1);
        let s2 = Cons(b, Box::new(s1));
        let t1 = Cons(a, Box::new(s2)); 
        let t2 = Exp::List(t1);
        let x4 = t2.clone();
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = Cons(z, d2);
        let x5 = s3.clone();
        let t3 = Box::new(s3);
        let t4 = Cons(y, t3);
        let v = Exp::List(t4);
        let x2 = v.clone();
        let f2 = Box::new(v);
        // represent (define (square x) (* x x))
        let t5 = Cons(f3, d3);
        let x3 = t5.clone();
        let t6 = Box::new(t5);
        let t7 = Cons(f2, t6);
        let x1 = Exp::List(t7.clone());
        let t8 = Box::new(t7);
        let t9 = Cons(f1, t8);
        let exp = Exp::List(t9);
        if let Ok(Exp::Symbol(x)) = car(exp.clone()) {
            assert_eq!(x.to_string(), "define");
        };
        assert_eq!(caadr(exp.clone()).unwrap(), Exp::Symbol("square"));
        assert_eq!(car(exp.clone()).unwrap(), Exp::Symbol("define"));
        assert_eq!(cdr(exp.clone()).unwrap(), x1);
        assert_eq!(cadr(exp.clone()).unwrap(), x2);
        assert_eq!(cddr(exp.clone()).unwrap(), Exp::List(x3));
        assert_eq!(cdddr(exp.clone()).unwrap(), Exp::List(Nil));
        assert_eq!(caddr(exp.clone()).unwrap(), x4);
        assert_eq!(cdadr(exp.clone()).unwrap(), Exp::List(x5));
    }

    #[test]
    fn test_equlity() {
        let f1 = Box::new(Exp::Symbol("define"));
        let y = Box::new(Exp::Symbol("square"));
        let z = Box::new(Exp::Symbol("x"));
        let a = Box::new(Exp::Symbol("*"));
        let b = Box::new(Exp::Symbol("x"));
        let c = Box::new(Exp::Symbol("x"));
        let d1 = Box::new(Nil);
        let d2 = Box::new(Nil);
        let d3 = Box::new(Nil);
        // represent (* x x)
        let s1 = Cons(c, d1);
        let s2 = Cons(b, Box::new(s1));
        let t1 = Cons(a, Box::new(s2)); 
        let t2 = Exp::List(t1);
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = Cons(z, d2);
        let t3 = Box::new(s3);
        let t4 = Cons(y, t3);
        let v = Exp::List(t4);
        let f2 = Box::new(v);
        // represent (define (square x) (* x x))
        let t5 = Cons(f3, d3);
        let t6 = Box::new(t5);
        let t7 = Cons(f2, t6);
        let t8 = Box::new(t7);
        let t9 = Cons(f1, t8);
        let exp = Exp::List(t9);
        let ref lrh = Nil;
        let rhs = &Nil;
        assert_eq!(lrh , rhs);
    }

    #[test]
    fn test_tagged_list() {
        let f1 = Box::new(Exp::Symbol("define"));
        let y = Box::new(Exp::Symbol("square"));
        let z = Box::new(Exp::Symbol("x"));
        let a = Box::new(Exp::Symbol("*"));
        let b = Box::new(Exp::Symbol("x"));
        let c = Box::new(Exp::Symbol("x"));
        let d1 = Box::new(Nil);
        let d2 = Box::new(Nil);
        let d3 = Box::new(Nil);
        // represent (* x x)
        let s1 = Cons(c, d1);
        let s2 = Cons(b, Box::new(s1));
        let t1 = Cons(a, Box::new(s2)); 
        let t2 = Exp::List(t1);
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = Cons(z, d2);
        let t3 = Box::new(s3);
        let t4 = Cons(y, t3);
        let v = Exp::List(t4);
        let f2 = Box::new(v);
        // represent (define (square x) (* x x))
        let t5 = Cons(f3, d3);
        let t6 = Box::new(t5);
        let t7 = Cons(f2, t6);
        let t8 = Box::new(t7);
        let t9 = Cons(f1, t8);
        let exp = Exp::List(t9);
        let tag1 = "define";
        assert_eq!(is_tagged_list(exp.clone(), tag1), true);
        assert_ne!(is_tagged_list(exp.clone(), "apple"), true);
    }
}

