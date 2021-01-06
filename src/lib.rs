
mod interpreter_core {
    enum Pair {
        Cons(i32, Box<Pair>),
        Nil,
    }

    /* almost everything is Exp to be interpreted */
    pub enum Exp {
        FloatNumber(f32),
        Integer(i32),
        List(Pair),
        Symbol(&'static str),
    }

    /* implement methods for Exp */
    #[allow(dead_code)]
    impl Exp {
        fn is_pair(exp: &Exp) -> bool { true }

        fn is_variable(exp: &Exp) -> bool { true }

        fn is_quoted(exp: &Exp) -> bool { true }

        fn is_tagged(exp: &Exp, tag: &'static str) -> bool { true }

        fn is_assignment(exp: &Exp) -> bool { true }

        fn is_definiton(exp: &Exp) -> bool { true }

        fn is_symbol(exp: &Exp) -> bool { true }

        fn is_lambda(exp: &Exp) -> bool { true }

        fn is_if(exp: &Exp) -> bool { true }

        fn is_begin(exp: &Exp) -> bool { true }

        fn is_application(exp: &Exp) -> bool { true }

        fn is_cond(exp: &Exp) -> bool { true }
    }
    /* operations on List variant of Exp */
    fn car(exp: &Exp) -> Option<&Exp> {Some(exp)}

    fn cdr(exp: &Exp) -> Option<&Exp> {Some(exp)}

    fn cadr(exp: &Exp) -> Option<&Exp> {Some(exp)}

    struct Env {}

    struct Procedure {}

    /* core function of the Scheme interpreter */
    fn eval(exp: Exp, env: Env) -> Result<Exp, &'static str> {Ok(exp)}

    fn apply(p: Procedure, args: Exp) -> Result<Exp, &'static str> {Ok(args)}
}

mod  represent{
    use super::interpreter_core::Exp;

    /* operations on Exp which is not treated as enum methods */
    pub fn is_number(exp: &Exp) -> bool { 
        match exp {
            Exp::FloatNumber(_x) => true,
            Exp::Integer(_x) => true,
            _ => false,
        }        
    }

    pub fn is_string(exp: &Exp) -> bool { 
        match exp {
            Exp::Symbol(_x) => true,
            _ => false,
        }
    }

    pub fn is_self_evaluating(x: &Exp) -> bool {
        if is_number(x) {
            true
        } else if is_string(x) {
            true
        } else { false }
    }
} 

#[cfg(test)]
mod tests {
    use super::interpreter_core::Exp;
    use super::represent;

    #[test]
    fn test_is_number() {
        let x = Exp::Integer(3);
        assert_eq!(represent::is_number(&x), true);
    }

    #[test] 
    fn test_is_string() {
        let str = "'symbol";
        let x = Exp::Symbol(str);
        assert_eq!(represent::is_string(&x), true);
    }
}