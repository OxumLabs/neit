// Define the new enum for logical joiners.
#[derive(Debug, Clone, PartialEq)]
pub enum LogicalJoin {
    And,
    Or,
}

/// Represents a value used in a condition: a literal string, a numeric literal, or a variable name.
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    /// A string literal (e.g. "hello")
    Literal(String),
    /// A numeric literal (e.g. 3.14)
    Numeric(f64),
    /// A variable (e.g. x)
    Variable(String),
}

/// Tokens for conditional operators.
#[derive(Debug, Clone, PartialEq)]
pub enum CondToks {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

/// Represents a single condition (or comparison) in a compound condition.
/// For example: x >= 10
#[derive(Debug, Clone)]
pub struct ChildCond {
    pub left: Operand,
    pub operator: CondToks,
    pub right: Operand,
    /// The logical joiner (e.g. AND/OR) following this condition, if any.
    pub joiner: Option<LogicalJoin>,
}

/// Represents a complete condition block that may contain one or more child conditions.
#[derive(Debug, Clone)]
pub struct Condition {
    pub child_conditions: Vec<ChildCond>,
}
pub mod condition_parser;
pub mod c_condmk;
