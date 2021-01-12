use ruscheme::core_of_interpreter::{Pair::*, Exp};


fn main() {
    let x = Box::new(&Exp::Integer(3));
    let y = Box::new(&Exp::Integer(4));
    let z = Box::new(&Nil);
    let a = &Cons(y, z);
    let b = &Cons(x, Box::new(a));
    let s = Exp::List(b);
    if let Exp::List(Cons(x, y)) = s {
    let z = Box::new(Cons(Box::new(&Exp::Integer(4)), Box::new(&Nil)));
    println!("{:?}", z);
    println!("{:?}", *y);
    }
}