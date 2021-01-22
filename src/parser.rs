pub mod parser {
    use crate::core_of_interpreter::core_of_interpreter::Exp;
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
    pub fn assemble_abstract_syntax_tree(_v: &Vec<String>) -> Exp {
        let x: Exp = Exp::FloatNumber(9.0);
        x
    }
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
}
