#[derive(Debug, Clone, Copy)]
pub enum ErrTypes {
    /// Unknown command encountered.
    UnknownCMD(i32),
    /// Unsupported variable type.
    UnsupportedVarType(i32),
    /// Variable already exists.
    VarAlreadyExists(i32),
    /// Char type variable shall have a length of 1.
    CharVarLen(i32),
    /// Invalid math usage (e.g. multiple operators in sequence).
    InvalidMathUsage(i32),
    /// Referenced variable not found.
    VarNotFound(i32),
    /// Generic syntax error.
    SyntaxError(i32),
    /// Expected operator is missing.
    MissingOperator(i32),
    /// An unexpected token was encountered.
    UnexpectedToken(i32),
    /// A required value is missing.
    MissingValue(i32),
    /// Parentheses or similar grouping symbols are unbalanced.
    UnbalancedParentheses(i32),
    /// Attempted division by zero.
    DivisionByZero(i32),
    /// Mismatched types in an operation or assignment.
    TypeMismatch(i32),
    /// Reserved keyword used as identifier.
    ReservedKeyword(i32),
    UnexpectedEndOfInput(i32),
    InvalidNumberFormat(i32),
    DuplicateOperator(i32),
    MissingLeftOperand(i32),
    MissingRightOperand(i32),
    UnsupportedOperator(i32),
    InvalidConditionSyntax(i32),
}
