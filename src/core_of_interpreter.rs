#[allow(dead_code)]
pub mod core_of_interpreter {
    #[derive(Debug)]
    pub enum Pair<'a> {
        Cons(Box<&'a Exp<'a>>, Box<&'a Pair<'a>>),
        Nil,
    }

    impl<'a> PartialEq for Pair<'a> {
        fn eq(&self, other: &Self) -> bool {
            match self{
                Pair::Nil => {
                    match other {
                        Pair::Nil => true,
                        _ => false,
                    }
                },
                Pair::Cons(x, y) => {
                    match other {
                        Pair::Nil => false,
                        Pair::Cons(x1, y1) => {
                            if x == x1 && y == y1 { true } else { false }
                        },
                    }
                },
            }
        }
    }

    /* everything is an Exp to be interpreted */
    #[derive(Debug)]
    pub enum Exp<'a> {
        FloatNumber(f32),
        Integer(i32),
        List(&'a Pair<'a>),
        Symbol(&'static str),
        Quote(&'static str),
        SchemeString(&'static str),
    }

    impl<'a> PartialEq for Exp<'a> {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Exp::FloatNumber(x) => {
                    match other {
                        Exp::FloatNumber(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },
                
                Exp::Integer(x) => {
                    match other {
                        Exp::Integer(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },

                Exp::List(x) => {
                    match other {
                        Exp::List(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },
                
                Exp::Symbol(x) => {
                    match other {
                        Exp::Symbol(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },

                Exp::Quote(x) => {
                    match other {
                        Exp::Quote(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },

                Exp::SchemeString(x) => {
                    match other {
                        Exp::SchemeString(y) => {
                            if x == y { true } else { false }
                        },
                        _ => false,
                    }
                },
            }
        }
    }

    struct Env {}

    struct Procedure {}

    /* core function of the Scheme interpreter */
    fn eval(exp: Exp, env: Env) -> Result<Exp, &'static str> {Ok(exp)}

    fn apply(p: Procedure, args: Exp) -> Result<Exp, &'static str> {Ok(args)}
}
