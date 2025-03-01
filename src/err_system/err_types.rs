///Different types of errors
/// format is ErrType(line_containing the error)
#[derive(Debug,Clone, Copy)]
pub enum ErrTypes{
    ///Unknown command , bascially we ecnountered Something we dont know how to handle
    UnknownCMD(i32)
}