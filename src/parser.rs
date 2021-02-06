pub mod parser {
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    use std::io;
    use std::io::prelude::*;
    use std::fs::File;
    use std::io::BufReader;

    #[allow(dead_code)]
    pub fn read_scheme_programs_from_stdin(p: &mut Vec<String>) -> io::Result<()> {
        let stdin = io::stdin();
    
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        p.push(line);
                    }
                }
                Err(_e) => break,
        }
    }
        Ok(())
    } 

    #[allow(dead_code)]
    pub fn read_scheme_programs_from_file(p: &mut Vec<String>) -> io::Result<()>{
        let f = File::open("scheme.txt")?;
        let reader = BufReader::new(f);
        
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        p.push(line);
                    }
                }
                Err(_e) => break,
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn tokenize(p: &mut Vec<String>) -> Vec<String> {
        let mut ss: Vec<String> = p.into_iter().map(|x| x.replace("(", " ( ")).collect();
        ss = ss.into_iter().map(|x| x.replace(")", " ) ")).collect();
        let mut tokens: Vec<String> = vec![];
        for  item in ss.iter() {
            let mut v = item.trim().split_whitespace().collect::<Vec<_>>().
                                  into_iter().map(|x| x.to_string()).collect(); 
            tokens.append(&mut v);
        }
        tokens
    }

    #[allow(dead_code)]
    pub fn reverse(s: &mut Vec<String>) -> Vec<String> {
        let mut x = vec![];
        while let Some(token) = s.pop() {
            x.push(token);
        }
        x
    }
/*
    #[allow(dead_code)]
    pub fn assemble_abstract_syntax_tree(tokens: &mut Vec<String>) -> Exp {
        let mut tree = Exp::List(Pair::Nil); 
        let mut exp_buffer: Vec<Exp> = vec![];
        while let Some(t) = tokens.pop() {
            let token = t;
            match token {
                // head of a Exp::List
                x if x == "(".to_string() => {
                    let subtree = assemble_abstract_syntax_tree(tokens);
                },
                // tail of a Exp::List 
                x if x == ")".to_string() => {
                    
                },
                // bool value
                x if x == "true" => { exp_buffer.push(Exp::Bool(true))},
                x if x == "false" => {exp_buffer.push(Exp::Bool(false))},
                // scheme string, for example, "winter is coming!"
                x if x == "\"".to_string() => {
                    let s = read_scheme_string(tokens);
                    exp_buffer.push(s);
                },
                // scheme quote, for example, 'winter
                x if x.chars().nth(0) == Some('\'') => {}, 
                // f32
                x if helper(x.clone()) => {},
                _ => { panic!("unknow token!"); },
            }    
        }
        tree
    }

    fn read_scheme_string(tokens: &mut Vec<String>) -> Exp {
        Exp::SchemeString("hello world!".to_string())
    }

    fn helper(x: String) -> bool {
        let s = x.parse::<f32>();
        match s {
            Ok(_x) => true,
            _ => false,
        }
    }
    */
}

#[cfg(test)]
mod tests {
    use super::parser::*;
    #[test]
    fn test_read_scheme_programs() {
        let mut programs: Vec<String> = vec![];
        read_scheme_programs_from_file(&mut programs);
        let mut item = programs.iter();
        assert_eq!(item.next(), Some(&"(define (fac n)".to_string()));
        assert_eq!(item.next(), Some(&"   (if (= n 1)".to_string()));
        assert_eq!(item.next(), Some(&"        1".to_string()));
        assert_eq!(item.next(), Some(&"       (* n".to_string()));
        assert_eq!(item.next(), Some(&"          (fac (- n 1)))))".to_string()));
        assert_eq!(item.next(), None);
    }
    
    #[test]
    fn test_tokenize() {
        let mut programs: Vec<String> = vec![];
        let mut tokens: Vec<String> = vec![];
        read_scheme_programs_from_file(&mut programs);
        tokens = tokenize(&mut programs);
        let s = vec!["(", "define", "(", "fac", "n", ")",
                      "(", "if", "(", "=", "n", "1", ")",
                        "1", "(", "*", "n", "(", "fac",
                          "(", "-", "n",
                             "1", ")", ")", ")", ")", ")"];
        let mut ss: Vec<String> = vec![];
        ss = s.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(ss, tokens);
    }    

    #[test]
    fn test_reverse() {
        let x = vec!["1", "winter", "n"];
        let mut s1 = x.into_iter().map(|x| x.to_string()).collect();
        let y = reverse(&mut s1);
        let z = vec!["n", "winter", "1"];
        let s2: Vec<String> = z.into_iter().map(|x| x.to_string()).collect();
        assert_eq!(y, s2);
    }
}
