
#![allow(unused_variables)]
pub mod  represent{
    use crate::{core_of_interpreter::core_of_interpreter::{ Exp, Pair}, scheme_list};
    use crate::tool::tools::{scheme_cons, append};
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

        pub fn is_primitive_procedure(&self) -> bool {
            if let Exp::Symbol(x) = self {
                match x {
                    t if *t == "*".to_string() => true,
                    t if *t ==  "/".to_string() => true,
                    t if *t == "+".to_string() => true,
                    t if *t == "-".to_string() => true,
                    t if *t == "car".to_string() => true,
                    t if *t =="cdr".to_string() => true,
                    t if *t == "cons".to_string() => true,
                    t if *t == "null?".to_string() => true,
                    t if *t == "=".to_string() => true,
                    t if *t == ">".to_string() => true,
                    t if *t == "<".to_string() => true,
                    t if *t == "display".to_string() => true,
                    _ => false,
                }
            } else { false }
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

        pub fn to_f32(&self) -> f32 {
            match self {
                Exp::FloatNumber(x) => { *x },
                _ => { panic!("not a f32 !"); },
            }
        }
        
        pub fn to_i32(&self) -> i32 {
            match self {
                Exp::Integer(x) => { *x },
                _ => { panic!("not a i32 !"); },
            }
        }

        pub  fn is_bool(&self) -> bool {
            match self {
                Exp::Bool(x) => true,
                _ => false,
            }
        }

        pub fn is_self_evaluating(&self) -> bool {
            self.is_null() || self.is_number() || self.is_string() || self.is_bool() || is_number_combination(self.clone())
        }

        pub fn is_null(&self) -> bool {
            *self == Exp::List(Pair::Nil)
        }

    }

        /* operations on Exp as function */
        #[allow(dead_code)]
        pub fn is_number_combination(exp: Exp) -> bool {
            match exp {
                Exp::FloatNumber(x) => true,
                Exp::Integer(x) => true,
                Exp::Symbol(x) => false,
                Exp::Quote(x) => false,
                Exp::SchemeString(x) => false,
                Exp::Bool(x) => false,
                Exp::List(Pair::Nil) => false,
                Exp::List(Pair::Cons(x, y)) => {
                    let s = is_number_combination(*x);
                    if s {
                        let mut temp = y;
                        while let Pair::Cons(lhs, rhs) = *temp {
                            let s1 = is_number_combination(*lhs);
                            if s1 {
                                if *rhs == Pair::Nil {
                                    break;
                                } else {
                                    temp = rhs;
                                }
                            } else {
                                return false
                            }
                        }
                        return true
                    } else {
                        return false
                    }
                },
            }
        }

        #[allow(dead_code)]
        pub fn is_assignment(exp: Exp) -> bool { 
            is_tagged_list(exp, "set!")
        }

        #[allow(dead_code)]
        pub fn assignment_variable(exp: Exp) -> Exp {
            cadr(exp).unwrap()
        }

        #[allow(dead_code)]
        pub fn assignment_value(exp: Exp) -> Exp {
            caddr(exp).unwrap()
        }
        // definiton
        #[allow(dead_code)]
        pub fn is_definiton(exp: Exp) -> bool { 
            is_tagged_list(exp, "define")
        }

        #[allow(dead_code)]
        pub fn definition_variable(exp: Exp) -> Exp {
            if cadr(exp.clone()).unwrap().is_symbol() {
                cadr(exp.clone()).unwrap()
            } else {
                caadr(exp.clone()).unwrap()
            }
        }
        

        #[allow(dead_code)]
        // (define x 3) 
        // (define (square x) (* x x))
        pub fn definition_value(exp: Exp) -> Exp {
            if cadr(exp.clone()).unwrap().is_symbol() {
                if cddr(exp.clone()).unwrap() == Exp::List(Pair::Nil) {
                    Exp::List(Pair::Nil)
                } else {
                    caddr(exp.clone()).unwrap()
                }
            } else {
                make_lambda(cdadr(exp.clone()).unwrap(),
                                    cddr(exp.clone()).unwrap())
            }
        }
        
        // lambda
        #[allow(dead_code)]
        pub fn is_lambda(exp: Exp) -> bool { 
            is_tagged_list(exp, "lambda")
        }

        #[allow(dead_code)]
        pub fn lambda_parameters(exp: Exp) -> Exp {
            cadr(exp.clone()).unwrap()
        }

        #[allow(dead_code)]
        pub fn lambda_body(exp: Exp) -> Exp {
            cddr(exp.clone()).unwrap()
        }

        #[allow(dead_code)]
        pub fn make_lambda (parameters: Exp, body: Exp) -> Exp {
            scheme_cons(Exp::Symbol("lambda".to_string()),
                          scheme_cons(parameters, body))
        }

        // if 
        #[allow(dead_code)]
        pub fn is_if(exp: Exp) -> bool { 
            is_tagged_list(exp, "if")
        }

      
        #[allow(dead_code)]
        pub fn if_predicate(exp: Exp) -> Exp {
            cadr(exp.clone()).unwrap()
        }

        #[allow(dead_code)]
        pub fn if_consequent(exp: Exp) -> Exp {
            caddr(exp.clone()).unwrap()
        }

        #[allow(dead_code)]
        pub fn if_alternative(exp: Exp) -> Exp {
            let s = Exp::List(Pair::Nil);
            if cdddr(exp.clone()).unwrap() != s {
                cadddr(exp.clone()).unwrap()
            } else {
                panic!("if alternative not exist!");
            }
        }

        #[allow(dead_code)]
        pub fn make_if(predicate: Exp, consequent: Exp, alternative: Exp) -> Exp{
            let tag = Exp::Symbol("if".to_string());
            let null = Exp::List(Pair::Nil);
            scheme_cons(tag,scheme_cons(predicate,
                      scheme_cons(consequent, 
                           scheme_cons(alternative, null))))
        }

        // begin
        #[allow(dead_code)]
        pub fn is_begin(exp: Exp) -> bool { 
            is_tagged_list(exp, "begin")
        }
        
        #[allow(dead_code)]
        pub fn begin_actions(exp: Exp) -> Exp {
            cdr(exp).unwrap()
        }

        #[allow(dead_code)]
        pub fn is_last_exp(seq: Exp) -> bool {
            let null = Exp::List(Pair::Nil);
            cdr(seq).unwrap() == null
        }

        #[allow(dead_code)]
        pub fn first_exp(seq: Exp) -> Exp {
            car(seq).unwrap()
        }

        #[allow(dead_code)]
        pub fn rest_exps(seq: Exp) -> Exp {
            cdr(seq).unwrap()
        }

        #[allow(dead_code)]
        pub fn sequence_to_exp(seq: Exp) -> Exp {
            let null = Exp::List(Pair::Nil);
            if seq == null {
                seq
            } else if is_last_exp(seq.clone()) {
                first_exp(seq)
            } else {
                make_begin(seq)
            }
        }

        #[allow(dead_code)]
        pub fn make_begin(seq: Exp) -> Exp {
            scheme_cons(Exp::Symbol("begin".to_string()), seq)
        }

        // representing procedures
        #[allow(dead_code)]
        pub fn make_procedure(parameters: Exp, body: Exp) -> Exp {
            scheme_list!(Exp::Symbol("procedure".to_string()), parameters, body)
        }

        #[allow(dead_code)]
        pub fn is_compound_procedure(p: Exp) -> bool {
            is_tagged_list(p, "procedure")
        }

        #[allow(dead_code)]
        pub fn is_primitive_procedure(p: Exp) -> bool {
            is_tagged_list(p, "primitive")
        }

        #[allow(dead_code)]
        pub fn procedure_parameters(p: Exp) -> Exp {
            cadr(p).unwrap()
        }

        #[allow(dead_code)]
        pub fn procedure_body(p: Exp) -> Exp {
            caddr(p).unwrap()
        }
        /* procedure_environment to be deleted */
        // #[allow(dead_code)]
        // pub fn procedure_environment(p: Exp) -> Env {
        //    Env(cadddr(p).unwrap())
        // }
        // A procedure application is any compound expression that is 
        // not one of the above expression types
        #[allow(dead_code)]
        pub fn is_application(exp: Exp) -> bool { 
            exp.is_pair()
        }

        #[allow(dead_code)]
        pub fn operator(exp: Exp) -> Exp {
            car(exp).unwrap()
        }

        #[allow(dead_code)]
        pub fn operands(exp: Exp) -> Exp {
            cdr(exp).unwrap()
        }

        #[allow(dead_code)]
        pub fn no_operands(ops: Exp) -> bool {
           let null = Exp::List(Pair::Nil);
           ops == null 
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
/* operations on List variant of Exp */
    #[allow(dead_code)]
    pub fn is_null(exp: &Exp) -> bool {
        *exp == Exp::List(Pair::Nil)
    }

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
    pub fn caar(exp: Exp) -> Result<Exp, &'static str> {
        let s1 = car(exp).unwrap();
        car(s1)
    }

    #[allow(dead_code)]
    pub fn cdar(exp: Exp) -> Result<Exp, &'static str> {
        let s1 = car(exp).unwrap();
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
    use crate::{scheme_list, tool::tools::generate_test_frames};
    use crate::core_of_interpreter::core_of_interpreter::{Env, Exp, Pair};
    use super::represent::*;
    use crate::tool::tools::{append, scheme_cons, generate_test_data };
    #[test]
    fn test_is_number() {
        let x = Exp::Integer(3);
        assert_eq!(x.is_number(), true);
    }

    #[test] 
    fn test_is_string() {
        let str = "summer";
        let x = Exp::Symbol(str.to_string());
        assert_eq!(x.is_symbol(), true);
    }

    #[test]
    fn test_is_self_evaluating() {
        let x = Exp::FloatNumber(3.14);
        let y = Exp::SchemeString("Winter".to_string());
        assert_eq!(x.is_self_evaluating() && y.is_self_evaluating(), true);
    }

    #[test]
    fn test_is_symbol() {
        let x = Exp::Symbol("item".to_string());
        assert_eq!(x.is_symbol(), true);
    }

    #[test]
    fn test_is_pair() {
        let a = Box::new(Exp::Integer(1));
        let b = Box::new(Exp::Integer(2));
        let c = Box::new(Exp::Integer(3));
        let d = Box::new(Pair::Nil); 
        let x = Pair::Cons(c, d);
        let y = Pair::Cons(b, Box::new(x));
        let z = Pair::Cons(a, Box::new(y));
        let s = Exp::List(z);
        assert_eq!(s.is_pair(), true);
    }

    #[test]
    fn test_is_quoted() {
        let x = Exp::Quote("'x".to_string());
        assert_eq!(x.is_quoted(), true);
    }

    #[test]
    fn test_list_operatioins() {
        // It's painful to build List in Rust...
        // (define (square x) (* x  x))
        let f1 = Box::new(Exp::Symbol("define".to_string()));
        let y = Box::new(Exp::Symbol("square".to_string()));
        let z = Box::new(Exp::Symbol("x".to_string()));
        let a = Box::new(Exp::Symbol("*".to_string()));
        let b = Box::new(Exp::Symbol("x".to_string()));
        let c = Box::new(Exp::Symbol("x".to_string()));
        let d1 = Box::new(Pair::Nil);
        let d2 = Box::new(Pair::Nil);
        let d3 = Box::new(Pair::Nil);
        // represent (* x x)
        let s1 = Pair::Cons(c, d1);
        let s2 = Pair::Cons(b, Box::new(s1));
        let t1 = Pair::Cons(a, Box::new(s2)); 
        let t2 = Exp::List(t1);
        let x4 = t2.clone();
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = Pair::Cons(z, d2);
        let x5 = s3.clone();
        let t3 = Box::new(s3);
        let t4 = Pair::Cons(y, t3);
        let v = Exp::List(t4);
        let x2 = v.clone();
        let f2 = Box::new(v);
        // represent (define (square x) (* x x))
        let t5 = Pair::Cons(f3, d3);
        let x3 = t5.clone();
        let t6 = Box::new(t5);
        let t7 = Pair::Cons(f2, t6);
        let x1 = Exp::List(t7.clone());
        let t8 = Box::new(t7);
        let t9 = Pair::Cons(f1, t8);
        let exp = Exp::List(t9);
        if let Ok(Exp::Symbol(x)) = car(exp.clone()) {
            assert_eq!(x.to_string(), "define");
        };
        assert_eq!(caadr(exp.clone()).unwrap(), Exp::Symbol("square".to_string()));
        assert_eq!(car(exp.clone()).unwrap(), Exp::Symbol("define".to_string()));
        assert_eq!(cdr(exp.clone()).unwrap(), x1);
        assert_eq!(cadr(exp.clone()).unwrap(), x2);
        assert_eq!(cddr(exp.clone()).unwrap(), Exp::List(x3));
        assert_eq!(cdddr(exp.clone()).unwrap(), Exp::List(Pair::Nil));
        assert_eq!(caddr(exp.clone()).unwrap(), x4);
        assert_eq!(cdadr(exp.clone()).unwrap(), Exp::List(x5));
    }

    #[test]
    fn test_equlity() {
        let f1 = Box::new(Exp::Symbol("define".to_string()));
        let y = Box::new(Exp::Symbol("square".to_string()));
        let z = Box::new(Exp::Symbol("x".to_string()));
        let a = Box::new(Exp::Symbol("*".to_string()));
        let b = Box::new(Exp::Symbol("x".to_string()));
        let c = Box::new(Exp::Symbol("x".to_string()));
        let d1 = Box::new(Pair::Nil);
        let d2 = Box::new(Pair::Nil);
        let d3 = Box::new(Pair::Nil);
        // represent (* x x)
        let s1 = Pair::Cons(c, d1);
        let s2 = Pair::Cons(b, Box::new(s1));
        let t1 = Pair::Cons(a, Box::new(s2)); 
        let t2 = Exp::List(t1);
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = Pair::Cons(z, d2);
        let t3 = Box::new(s3);
        let t4 = Pair::Cons(y, t3);
        let v = Exp::List(t4);
        let f2 = Box::new(v);
        // represent (define (square x) (* x x))
        let t5 = Pair::Cons(f3, d3);
        let t6 = Box::new(t5);
        let t7 = Pair::Cons(f2, t6);
        let t8 = Box::new(t7);
        let t9 = Pair::Cons(f1, t8);
        let exp = Exp::List(t9);
        let ref lrh = Pair::Nil;
        let rhs = &Pair::Nil;
        assert_eq!(lrh , rhs);
    }

    #[test]
    fn test_tagged_list() {
        let f1 = Box::new(Exp::Symbol("define".to_string()));
        let y = Box::new(Exp::Symbol("square".to_string()));
        let z = Box::new(Exp::Symbol("x".to_string()));
        let a = Box::new(Exp::Symbol("*".to_string()));
        let b = Box::new(Exp::Symbol("x".to_string()));
        let c = Box::new(Exp::Symbol("x".to_string()));
        let d1 = Box::new(Pair::Nil);
        let d2 = Box::new(Pair::Nil);
        let d3 = Box::new(Pair::Nil);
        // represent (* x x)
        let s1 = Pair::Cons(c, d1);
        let s2 = Pair::Cons(b, Box::new(s1));
        let t1 = Pair::Cons(a, Box::new(s2)); 
        let t2 = Exp::List(t1);
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = Pair::Cons(z, d2);
        let t3 = Box::new(s3);
        let t4 = Pair::Cons(y, t3);
        let v = Exp::List(t4);
        let f2 = Box::new(v);
        // represent (define (square x) (* x x))
        let t5 = Pair::Cons(f3, d3);
        let t6 = Box::new(t5);
        let t7 = Pair::Cons(f2, t6);
        let t8 = Box::new(t7);
        let t9 = Pair::Cons(f1, t8);
        let exp = Exp::List(t9);
        let tag1 = "define";
        assert_eq!(is_tagged_list(exp.clone(), tag1), true);
        assert_ne!(is_tagged_list(exp.clone(), "apple"), true);
    }

    #[test]
    fn test_if() {
        let data = generate_test_data();
        let if_exp = data.if_expression;
        assert_eq!(is_if(if_exp.clone()), true);
        
        let s1 = Exp::Symbol("if".to_string());
        let s2 = Exp::Symbol("n".to_string());
        let s3 = Exp::Integer(1);
        let s4 = Exp::Symbol("-".to_string());
        let s5 = Exp::Symbol("=".to_string());
        let x1 = scheme_list!(s5, s2.clone(), s3.clone());
        let x2 = scheme_list!(s4, s2.clone(), s3.clone());
        assert_eq!(if_predicate(if_exp.clone()), x1);
        assert_eq!(if_consequent(if_exp.clone()), s3);
        assert_eq!(if_alternative(if_exp.clone()), x2);
        assert_eq!(make_if(if_predicate(if_exp.clone()), if_consequent(if_exp.clone()),
                   if_alternative(if_exp.clone())), if_exp);
    }

    #[test]
    fn test_begin() {
        let data = generate_test_data();
        //  (begin (set! x 5) (+ x 1))
        let begin_exp = data.begin_expression;
        let t1 = Exp::Integer(5);
        let t2 = Exp::Integer(1);
        let t3 = Exp::Symbol("set!".to_string());
        let t4 = Exp::Symbol("begin".to_string());
        let t5 = Exp::Symbol("x".to_string());
        let t6 = Exp::Symbol("+".to_string());
        // (set! x 5) 
        let y1 = scheme_list!(t3, t5.clone(), t1);
        // (+ x 1)
        let y2 = scheme_list!(t6, t5.clone(), t2);

        assert_eq!(is_begin(begin_exp.clone()), true);
        assert_eq!(begin_actions(begin_exp.clone()), scheme_list!(y1.clone(), y2.clone()));
        assert_eq!(is_last_exp(begin_exp.clone()), false);
        assert_eq!(first_exp(scheme_list!(y1.clone(), y2.clone())), y1.clone());
        assert_eq!(rest_exps(scheme_list!(y1.clone(), y2.clone())), scheme_list!(y2.clone()));
        assert_eq!(sequence_to_exp(scheme_list!(y1.clone(), y2.clone())), begin_exp.clone());
    }

    #[test]
    fn test_application() {
        let data = generate_test_data();
        let app_exp = data.applicatioin_expressioin;
        let p1 = Exp::Symbol("procedure".to_string());
        let p2 = Exp::Integer(3);
        let p3 = Exp::Integer(4);
        assert_eq!(is_application(app_exp.clone()), true);
        assert_eq!(operator(app_exp.clone()), p1.clone());
        assert_eq!(operands(app_exp.clone()), scheme_list!(p2.clone(), p3.clone()));
        assert_eq!(first_operand(operands(app_exp.clone()).clone()), p2.clone());
        assert_eq!(rest_operands(operands(app_exp.clone())), scheme_list!(p3));
    }

    #[test]
    fn test_lambda() {
        let data = generate_test_data();
        let lambda_exp = data.lambda_expression;

        // (lambda (x) (* x x))
        let r1 = Exp::Symbol("lambda".to_string());
        let r2 = Exp::Symbol("x".to_string());
        let r3 = Exp::Symbol("*".to_string());

        let parameters = scheme_list!(r2.clone());
        let body = scheme_list!(scheme_list!(r3, r2.clone(), r2.clone()));
        
        assert_eq!(is_lambda(lambda_exp.clone()), true);
        assert_eq!(lambda_parameters(lambda_exp.clone()), parameters.clone());
        assert_eq!(lambda_body(lambda_exp.clone()), body);
        assert_eq!(make_lambda(parameters, body), lambda_exp.clone());
    }

    #[test]
    fn test_procedure() {
        // primitive procedure: ('primitive +)
        // compound procedure: ('primitive (x y) (+ x y) env))
        // env:  (((x y z) 1 2 3) ((u v) 4 5))
        let frame = generate_test_frames().frame;
        let env = Env(scheme_list!(frame));
        let parameters = scheme_list!(Exp::Symbol("x".to_string()), Exp::Symbol("y".to_string()));
        let body = scheme_list!(Exp::Symbol("+".to_string()), Exp::Symbol("x".to_string()), 
                                Exp::Symbol("y".to_string()));
        let procedure = scheme_list!(Exp::Symbol("procedure".to_string()), 
                                         parameters.clone(),
                                         body.clone(),
                                         env.0.clone());
        assert_eq!(is_compound_procedure(procedure.clone()), true);
        assert_eq!(procedure_parameters(procedure.clone()), parameters);
        assert_eq!(procedure_body(procedure.clone()), body);
    }

    #[test]
    fn test_is_number_combination() {
        let mut x = scheme_list!(scheme_list!(Exp::Integer(1),
                                              Exp::Integer(2)),
                                 scheme_list!(Exp::FloatNumber(3.1),
                                              Exp::Symbol("s".to_string()),
                                              Exp::Integer(4)),
                                 scheme_list!(Exp::Integer(5),
                                              Exp::Integer(6)));
        assert_eq!(is_number_combination(x.clone()), false);
        x = scheme_list!(scheme_list!(Exp::Integer(1),
                                              Exp::Integer(2)),
                                 scheme_list!(Exp::FloatNumber(3.1),
                                              Exp::Integer(0),
                                              Exp::Integer(4)),
                                 scheme_list!(Exp::Integer(5),
                                              Exp::Integer(6)));
        assert_eq!(is_number_combination(x.clone()), true);
    }
}
