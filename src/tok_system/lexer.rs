use super::tokens::Token;

pub trait LexicalAnalysis {
    fn run_lexical_analysis(&mut self, code: &str);
}

impl LexicalAnalysis for Vec<Token> {
    fn run_lexical_analysis(&mut self, code: &str) {
        let mut word = String::new();
        for c in code.chars() {
            match c {
                '=' => {
                    if !word.is_empty(){
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::EqSign);
                }
                ' ' | '\t' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::Space);
                }
                '\n' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::EOL);
                }
                '\r' => { /* ignore carriage return */ }
                '\\' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::BackSlash);
                }
                _ => {
                    word.push(c);
                }
            }
        }
        if !word.is_empty() {
            self.push(Token::Iden(word));
        }
        self.push(Token::EOF);
    }
}
