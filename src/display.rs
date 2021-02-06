pub mod display {
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    use crate::tool::tools::scheme_cons;
    
    #[allow(dead_code)]
    pub fn print(exp: Exp) {
        match exp {
            Exp::FloatNumber(x) => print!("{}", x),
            Exp::Integer(x) => print!("{}", x),
            Exp::Symbol(x) => print!("{}", x),
            Exp::Quote(x) => print!("{}", x),
            Exp::SchemeString(x) => print!("{}", x),
            Exp::Bool(x) => print!("{}", x),
            Exp::List(Pair::Nil) => {
                print!("()");
            },
            Exp::List(Pair::Cons(x, y)) => {
                print!("(");
                print(*x);
                if *y != Pair::Nil {
                    print!(" ");
                }
                let mut temp = y;
                while let Pair::Cons(lhs, rhs) = *temp {
                    print(*lhs);
                    if *rhs == Pair::Nil {
                        break;
                    }
                    print!(" "); 
                    temp = rhs;
                }
                print!(")");
            },
        }    
    }

    #[allow(dead_code)]
    pub fn pretty_print(exp: Exp) {
        print!("value: ");
        print(exp);
        print!("\n");
    }


    #[allow(dead_code)]
    pub fn test_display() {
        let plus = Exp::Symbol("+".to_string());
        let x = Exp::Symbol("x".to_string());
        let null = Exp::List(Pair::Nil);
        //  parameters: (x)
        let parameters = scheme_cons(x.clone(), null.clone());
        // (x)
        let s1 = scheme_cons(x.clone(), null.clone());
        // (x x)
        let s2 = scheme_cons(x.clone(), s1.clone());
        // (+ x x)
        let s3 = scheme_cons(plus.clone(), s2.clone());
        // body: ((+ x x))
        let body = scheme_cons(s3.clone(), null.clone());
        // exp: ((x) (+ x x))
        let exp = scheme_cons(parameters.clone(), body.clone());
        // (1 2 3) 
        let n1 = Exp::Integer(1);
        let n2 = Exp::Integer(2);
        let n3 = Exp::Integer(3);
        let t1 = scheme_cons(n3, null.clone());
        let t2 = scheme_cons(n2, t1.clone());
        let t3 = scheme_cons(n1, t2.clone());
        println!("{:?}", null);
        pretty_print(null);
        println!("{:?}", parameters);
        pretty_print(parameters);
        println!("{:?}", body);
        pretty_print(body);
        println!("{:?}", exp);
        pretty_print(exp);
        println!("{:?}", t3.clone());
        pretty_print(t3);
    }
}
