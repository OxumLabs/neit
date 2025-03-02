///Different types of errors
/// format is ErrType(line_containing the error)
#[derive(Debug,Clone, Copy)]
pub enum ErrTypes{
    ///Unknown command , bascially we ecnountered Something we dont know how to handle
    UnknownCMD(i32),
    ///Unsupported variable type
    UnsupportedVarType(i32),
    ///Variable already exists in the public
    VarAlreadyExists(i32),
    ///Char type variable shall have length of 1
    CharVarLen(i32),
    ///Invalid math usage
    InvalidMathUsage(i32),
}