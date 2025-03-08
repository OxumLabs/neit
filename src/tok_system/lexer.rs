use super::tokens::Token;

pub trait LexicalAnalysis {
    fn run_lexical_analysis(&mut self, code: &str);
}

impl LexicalAnalysis for Vec<Token> {
    #[inline(always)]
    fn run_lexical_analysis(&mut self, code: &str) {
        let mut word = String::with_capacity(16);
        let mut chars = code.chars().peekable();

        #[inline(always)]
        fn flush_word(word: &mut String, tokens: &mut Vec<Token>) {
            if !word.is_empty() {
                tokens.push(Token::Iden(std::mem::take(word)));
            }
        }

        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    flush_word(&mut word, self);
                    self.push(Token::LSmallBrac);
                }
                ')' => {
                    flush_word(&mut word, self);
                    self.push(Token::RSmallBracket);
                }
                '!' => {
                    flush_word(&mut word, self);
                    self.push(Token::Not);
                }
                '|' if chars.peek() == Some(&'|') => {
                    flush_word(&mut word, self);
                    chars.next();
                    self.push(Token::Or);
                }
                '>' => {
                    flush_word(&mut word, self);
                    self.push(Token::GreaterThan);
                }
                '<' => {
                    flush_word(&mut word, self);
                    self.push(Token::LessThan);
                }
                '&' if chars.peek() == Some(&'&') => {
                    flush_word(&mut word, self);
                    chars.next();
                    self.push(Token::And);
                }
                '{' => {
                    flush_word(&mut word, self);
                    self.push(Token::LCurly);
                }
                '}' => {
                    flush_word(&mut word, self);
                    self.push(Token::RCurly);
                }
                '%' => {
                    flush_word(&mut word, self);
                    self.push(Token::PercentSign);
                }
                '=' => {
                    flush_word(&mut word, self);
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        self.push(Token::DoubleEqSign);
                    } else {
                        self.push(Token::EqSign);
                    }
                }
                ' ' | '\t' => {
                    flush_word(&mut word, self);
                    self.push(Token::Space);
                }
                '\n' => {
                    flush_word(&mut word, self);
                    self.push(Token::EOL);
                }
                '\r' => {}
                '\\' => {
                    flush_word(&mut word, self);
                    self.push(Token::BackSlash);
                }
                '+' => {
                    flush_word(&mut word, self);
                    self.push(Token::ADDOP);
                }
                '-' => {
                    flush_word(&mut word, self);
                    self.push(Token::SUBOP);
                }
                '/' => {
                    flush_word(&mut word, self);
                    self.push(Token::DIVOP);
                }
                '*' => {
                    flush_word(&mut word, self);
                    self.push(Token::MULTIOP);
                }
                _ => word.push(c),
            }
        }
        flush_word(&mut word, self);
        self.push(Token::EOF);
    }
}
