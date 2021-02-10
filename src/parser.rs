pub mod parser {
    use crate::{core_of_interpreter::core_of_interpreter::{Exp, Pair}, 
                tool::tools::{append, scheme_cons},
                represent::represent::{car},
                scheme_list};
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
                        let tokens = tokenize(p);
                        if syntax_checker(&tokens) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Err(_e) => break,
        }
    }
        Ok(())
    } 

    #[allow(dead_code)]
    pub fn syntax_checker(t: &Vec<String>) -> bool {
        let mut iterator = t.iter();
        let mut left_parenthesis = 0;
        let mut right_parenthesis = 0;
        let mut token = iterator.next();
        loop {
            match token {
                x if x == Some(&("(".to_string())) => {
                    left_parenthesis = left_parenthesis + 1;
                },
                x if x == Some(&(")".to_string())) => {
                    right_parenthesis = right_parenthesis + 1;
                },
                Some(_x)  => {},
                None => {
                    break;
                },
            }
            token = iterator.next();
        }
        if left_parenthesis == right_parenthesis {
            true
        } else {
            false
        }
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
    fn reverse(s: &mut Vec<String>) -> Vec<String> {
        let mut x = vec![];
        while let Some(token) = s.pop() {
            x.push(token);
        }
        x
    }

    #[allow(dead_code)]
    pub fn build_syntax_tree(tokens: &mut Vec<String>) -> Exp {
        let mut tokens = reverse(tokens);
        let tree = build_syntax_tree_helper(&mut tokens);
        if tree != Exp::List(Pair::Nil) {
            car(tree).unwrap()
        } else {
            Exp::List(Pair::Nil)
        }
    }
    #[allow(dead_code)]
    fn build_syntax_tree_helper(tokens: &mut Vec<String>) -> Exp {
        let mut tree_buffer = Exp::List(Pair::Nil);
        while let Some(t) = tokens.pop() {
            let token = t;
            match token {
                // head of a Exp::List
                x if x == "(".to_string() => {
                    let subtree = build_syntax_tree_helper(tokens);
                    tree_buffer = append(tree_buffer, 
                                       scheme_list!(subtree)); 
                },
                // tail of a Exp::List 
                x if x == ")".to_string() => {
                    break;    
                },
                x if x == "Nil".to_string() => {
                    tree_buffer = append(tree_buffer,
                                             Exp::List(Pair::Nil));
                }
                // bool value
                x if x == "true" => {
                    tree_buffer = append(tree_buffer, 
                                         scheme_list!(Exp::Bool(true)));
                },
                x if x == "false" => {
                    tree_buffer = append(tree_buffer, 
                                         scheme_list!(Exp::Bool(false)));
                },
                // symbol value
                x if is_symbol(&x) => {
                    tree_buffer = append(tree_buffer, 
                                         scheme_list!(Exp::Symbol(x)));
                }
                // scheme string, for example, "winter is coming!"
                x if x == "\"".to_string() => {
                    let s = read_scheme_string(tokens);
                    tree_buffer = append(tree_buffer, 
                                        scheme_list!(s));
                },
                // scheme quote, for example, 'winter
                x if x.chars().nth(0) == Some('\'') => {
                    tree_buffer = append(tree_buffer, 
                                         scheme_list!(Exp::Quote(x)));
                }, 
                // i32
                x if is_i32(x.clone()) => {
                    tree_buffer = append(tree_buffer,
                                   scheme_list!(Exp::Integer(x.parse::<i32>().unwrap())));
                }
                // f32
                x if is_f32(x.clone()) => {
                    tree_buffer = append(tree_buffer, 
                                    scheme_list!(Exp::FloatNumber(x.parse::<f32>().unwrap())));
                },
                _ => { panic!("unknow token!"); },
            }    
        }
        tree_buffer
    }

    fn read_scheme_string(_tokens: &mut Vec<String>) -> Exp {
        Exp::SchemeString("hello world!".to_string())
    }

    fn is_symbol(x: &String) -> bool {
        x.chars().nth(0).unwrap().is_alphabetic() || x == "=" || x == "+" || x == "-" || x == "*" || x == "/" || x == ">" || x == "<"
    }

    fn is_f32(x: String) -> bool {
        let s = x.parse::<f32>();
        match s {
            Ok(_x) => true,
            _ => false,
        }
    }

    fn is_i32(x: String) -> bool {
        let s = x.parse::<i32>();
        match s {
            Ok(_x) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{display::display::pretty_print, scheme_list};
    use crate::core_of_interpreter::core_of_interpreter::{Exp, Pair};
    use super::parser::*;
    use crate::tool::tools::{append, scheme_cons};
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
    fn test_syntax_checker() {
        let mut programs: Vec<String> = vec![];
        read_scheme_programs_from_file(&mut programs);
        let tokens = tokenize(&mut programs);
        assert_eq!(syntax_checker(&tokens), true);
    }

    #[test]
    fn test_build_syntax_tree() {
        let mut programs: Vec<String> = vec![];
        let mut tokens: Vec<String> = vec![];
        read_scheme_programs_from_file(&mut programs);
        tokens = tokenize(&mut programs);
        let x = build_syntax_tree(&mut tokens);
        /* test case:
         (define (fac n) 
                (if (= n 1)
                    1
                    (* n  
                       (fac ( - n  1)))))
        */
        let y = scheme_list!(Exp::Symbol("define".to_string()),
                      scheme_list!(Exp::Symbol("fac".to_string()),
                                   Exp::Symbol("n".to_string())),
                      scheme_list!(Exp::Symbol("if".to_string()),
                            scheme_list!(Exp::Symbol("=".to_string()),
                                         Exp::Symbol("n".to_string()),
                                         Exp::Integer(1)),
                                         Exp::Integer(1),
                            scheme_list!(Exp::Symbol("*".to_string()),
                                         Exp::Symbol("n".to_string()),
                                   scheme_list!(Exp::Symbol("fac".to_string()),
                                          scheme_list!(Exp::Symbol("-".to_string()),
                                                       Exp::Symbol("n".to_string()),
                                                       Exp::Integer(1))))));
        assert_eq!(x, y);
    }
}
