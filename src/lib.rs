use std::fmt;

struct Pair {
    lhs: Option<Exp>,
    rhs: Option<Exp>,
}

#[allow(dead_code)]
impl Pair {
    fn car(&self) -> &Option<Exp> {
        &self.lhs
    }

    fn cdr(&self) -> &Option<Exp> {
        &self.rhs
    }
} 

fn pretty_print(x: &Option<Exp>) {}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs = &self.lhs;
        let rhs = &self.rhs;

        write!(f, "(")?;
        pretty_print(lhs);
        write!(f, ", ")?;
        pretty_print(rhs);        
        write!(f, ")")
    }
}

struct Exp {}

struct Env {}

struct Procedure {}

fn eval(x: Exp, y: Env) -> Result<Exp, &'static str> {Ok(x)}

fn apply(p: Procedure, args: Exp) -> Result<Exp, &'static str> {Ok(args)}
