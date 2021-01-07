/* a Scheme interpreter implemented by Rust */

mod core_of_interpreter {
    pub enum Pair {
        Cons(Box<Exp>, Box<Pair>),
        Nil,
    }

    /* everything is an Exp to be interpreted */
    pub enum Exp {
        FloatNumber(f32),
        Integer(i32),
        List(Pair),
        Symbol(&'static str),
        Quote(&'static str),
        SchemeString(&'static str),
    }

    struct Env {}

    struct Procedure {}

    /* core function of the Scheme interpreter */
    fn eval(exp: Exp, env: Env) -> Result<Exp, &'static str> {Ok(exp)}

    fn apply(p: Procedure, args: Exp) -> Result<Exp, &'static str> {Ok(args)}
}

mod  represent{
    use super::core_of_interpreter::Exp;

    /* operatons on Exp as enum methods */
    #[allow(dead_code)]
    impl Exp {
        pub fn is_pair(&self) -> bool { 
            match self {
                Exp::List(_x) => true,
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

        pub fn is_tagged(exp: &Exp, tag: &'static str) -> bool { true }

        pub fn is_assignment(exp: &Exp) -> bool { true }

        pub fn is_definiton(exp: &Exp) -> bool { true }

        pub fn is_lambda(exp: &Exp) -> bool { true }

        pub fn is_if(exp: &Exp) -> bool { true }

        pub fn is_begin(exp: &Exp) -> bool { true }

        pub fn is_application(exp: &Exp) -> bool { true }

        pub fn is_cond(exp: &Exp) -> bool { true }
    }
    /* operations on List variant of Exp */
    pub fn car(exp: &Exp) -> Option<&Exp> {Some(exp)}

    pub fn cdr(exp: &Exp) -> Option<&Exp> {Some(exp)}

    pub fn cadr(exp: &Exp) -> Option<&Exp> {Some(exp)}
}

mod parser {
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::BufReader;

    pub fn read_scheme_programs_from_stdin() -> Vec<String> {
        let mut programs: Vec<String> = vec![]; 
        let stdin = io::stdin();
    
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => programs.push(line),
                Err(_e) => break,
            }
        }
        programs
    } 

    pub fn read_scheme_programs_from_file(p: &mut Vec<String>) -> io::Result<()>{
        let f = File::open("scheme.txt")?;
        println!("Scheme programs read.");
        let reader = BufReader::new(f);
        
        for line in reader.lines() {
            match line {
                Ok(line) => p.push(line),
                Err(_e) => break,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod representing_tests {
    use super::core_of_interpreter::{Pair::*, Exp};
    use super::represent;

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
        let x = Cons(Box::new(Exp::Integer(1)), 
                     Box::new(Cons(Box::new(Exp::Integer(2)), 
                     Box::new(Cons(Box::new(Exp::Integer(3)), Box::new(Nil))))));
        let y = Exp::List(x);
        assert_eq!(y.is_pair(), true);
    }

    #[test]
    fn test_is_quoted() {
        let x = Exp::Quote("'x");
        assert_eq!(x.is_quoted(), true);
    }
}

#[cfg(test)]
mod parser_tests {
    use super::parser::*;

    #[test]
    fn read_scheme_programs_works() {
        let mut programs: Vec<String> = vec![];
        read_scheme_programs_from_file(&mut programs);
        let mut item = programs.iter();
        assert_eq!(item.next(), Some(&"(define (fac n)".to_string()));
        assert_eq!(item.next(), Some(&"   (if (= n 1)".to_string()));
        assert_eq!(item.next(), Some(&"        1".to_string()));
        assert_eq!(item.next(), Some(&"       (* n".to_string()));

        assert_eq!(item.next(), Some(&"          (fac (- n 1)))))".to_string()));
        assert_eq!(item.next(), None);
    }
}