use ruscheme::core_of_interpreter::{Pair::*, Exp};


fn main() {
    let s = &Exp::List(Cons(Box::new(Exp::Integer(3)), Box::new(Cons(Box::new(Exp::Integer(4)),
    Box::new(Nil)))));
    if let Exp::List(Cons(x, y)) = s {
    let z = Box::new(Cons(Box::new(Exp::Integer(4)), Box::new(Nil)));
    println!("{:?}", z);
    println!("{:?}", *y);
    }
}