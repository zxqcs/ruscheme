/* a Scheme interpreter implemented by Rust */

mod core_of_interpreter {
    pub enum Pair {
        Cons(i32, Box<Pair>),
        Nil,
    }

    /* almost everything is Exp to be interpreted */
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

#[cfg(test)]
mod tests {
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
        let x = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        let y = Exp::List(x);
        assert_eq!(y.is_pair(), true);
    }

    #[test]
    fn test_is_quoted() {
        let x = Exp::Quote("'x");
        assert_eq!(x.is_quoted(), true);
    }
}