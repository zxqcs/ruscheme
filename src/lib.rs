use std::fmt;

mod core {
/*
struct Pair {
    lhs: Option<Exp>,
    rhs: Option<Exp>,
}
*/

/*
#[allow(dead_code)]
impl Pair {
    fn car(&self) -> &Option<Exp> {
        &self.lhs
    }

    fn cdr(&self) -> &Option<Exp> {
        &self.rhs
    }
} 

fn is_pair(x: &Exp) -> bool {
   true 
}

fn pretty_print(x: &Option<Exp>) {
    match &x {
        None => println!("()"),
        &Some(exp) => {
            if is_pair(&exp) {
                pretty_print(&x);
            }      
        }
    }
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lhs = &self.car();
        let rhs = &self.cdr();

        write!(f, "(")?;
        pretty_print(lhs);
        write!(f, ", ")?;
        pretty_print(rhs);        
        write!(f, ")")
    }
}
*/
enum Pair {
    Cons(i32, Box<Pair>),
    Nil,
}

pub enum Exp {
    FloatNumber(f32),
    Integer(i32),
    List(Pair),
    Symbol(&'static str),
}

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
/* operations on List variant  */
fn car(exp: &Exp) -> Option<&Exp> {Some(exp)}

fn cdr(exp: &Exp) -> Option<&Exp> {Some(exp)}

fn cadr(exp: &Exp) -> Option<&Exp> {Some(exp)}

struct Env {}

struct Procedure {}

fn eval(exp: Exp, env: Env) -> Result<Exp, &'static str> {Ok(exp)}

fn apply(p: Procedure, args: Exp) -> Result<Exp, &'static str> {Ok(args)}
}

mod  represent{
    use super::core::*; 
    use std::any::Any;

    fn is_number(exp: &dyn Any) -> bool { 
        if exp.is::<i32>() || exp.is::<f64>() {
            true
        } else { false }
    }

    fn is_string(exp: &dyn Any) -> bool { 
        if exp.is::<String>() {
            true
        } else {
            false
        }
    }

    fn is_self_evaluating(x: &Exp) -> bool {
        if is_number(x) {
            true
        } else if is_string(x) {
            true
        } else { false }
    }
} 