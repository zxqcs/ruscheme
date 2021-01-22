mod core_of_interpreter;
mod represent;
mod parser;
use core_of_interpreter::core_of_interpreter::{Exp, Pair::*};


fn main() {
    let f1 = Box::new(&Exp::Symbol("define"));
        let y = Box::new(&Exp::Symbol("square"));
        let z = Box::new(&Exp::Symbol("x"));
        let a = Box::new(&Exp::Symbol("*"));
        let b = Box::new(&Exp::Symbol("x"));
        let c = Box::new(&Exp::Symbol("x"));
        let d1 = Box::new(&Nil);
        let d2 = Box::new(&Nil);
        let d3 = Box::new(&Nil);
        // represent (* x x)
        let s1 = &Cons(c, d1);
        let s2 = &Cons(b, Box::new(s1));
        let t1 = &Cons(a, Box::new(s2)); 
        let t2 = &Exp::List(t1);
        let f3 = Box::new(t2);
        // represent (square x)
        let s3 = &Cons(z, d2);
        let t3 = Box::new(s3);
        let t4 = &Cons(y, t3);
        let v =  Exp::List(t4);
        let f2 = Box::new(&v);
        // represent (define (square x) (* x x))
        let t5 = &Cons(f3, d3);
        let t6 = Box::new(t5);
        let t7 = &Cons(f2, t6);
        let t8 = Box::new(t7);
        let t9 = &Cons(f1, t8);
        let _exp = Exp::List(t9);
}