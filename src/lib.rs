use std::fmt

struct Pair {
    lhs: Option<exp>,
    rhs: Option<exp>,
}

impl Pair {
    fn car(&self) -> Exp {
        self.lhs
    }

    fn cdr(&self) -> Exp {
        self.rhs
    }
} 

fn pretty_print(x: Exp) {}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs = self.lhs;
        let rhs = self.rhs;

        write!(f, "(")?;
        pretty_print(lhs);
        write!(f, ", ")?;
        pretty_print(rhs);        
        write!(f, ")");
    }
}

struct Exp {}

struct Env {}

struct Procedure {}

fn eval(x: Exp, y: Env) -> Result<Exp, &str> {}

fn apply(p: Procedure, args: Exp) -> Result<Exp, &str> {}
