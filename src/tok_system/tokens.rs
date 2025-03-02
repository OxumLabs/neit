use crate::parse_systems::Variables;

#[derive(Debug,PartialEq)]
pub enum Token {
    Iden(String),
    Var(Variables),
    Space,
    Quote,
    BackSlash,
    EqSign,
    ADDOP,
    SUBOP,
    DIVOP,
    MULTIOP,
    EOL,
    EOF,
    PercentSign,
}
