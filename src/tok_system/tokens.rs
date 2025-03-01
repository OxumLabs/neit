#[derive(Debug,PartialEq, Eq)]
pub enum Token {
    Iden(String),
    Space,
    Quote,
    BackSlash,
    EOL,
    EOF
}
