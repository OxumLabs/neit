use super::tokens::Token;

pub trait LexicalAnalysis {
    fn run_lexical_analysis(&mut self, code: &str);
}

impl LexicalAnalysis for Vec<Token> {
    fn run_lexical_analysis(&mut self, code: &str) {
        let mut word = String::new();
        let mut chars = code.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::LSmallBrac);
                }
                ')' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::RSmallBracket);
                }
                '!' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::Not);
                }
                '|' if chars.peek() == Some(&'|') => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::Or);
                }
                '>' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::GreaterThan);
                }
                '<' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::LessThan);
                }
                '&' if chars.peek() == Some(&'&') => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    chars.next();
                    self.push(Token::And);
                }
                '{' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::LCurly);
                }
                '}' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::RCurly);
                }
                '%' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::PercentSign);
                }
                '=' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        self.push(Token::DoubleEqSign);
                    } else {
                        self.push(Token::EqSign);
                    }
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
                '\r' => {}
                '\\' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::BackSlash);
                }
                '+' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::ADDOP);
                }
                '-' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::SUBOP);
                }
                '/' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::DIVOP);
                }
                '*' => {
                    if !word.is_empty() {
                        self.push(Token::Iden(word.clone()));
                        word.clear();
                    }
                    self.push(Token::MULTIOP);
                }
                _ => word.push(c),
            }
        }
        if !word.is_empty() {
            self.push(Token::Iden(word));
        }
        self.push(Token::EOF);
    }
}
