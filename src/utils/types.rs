#[derive(Debug, Clone)]

pub enum Tokens {
    Func(FN),
    FnCall(String),        /*  String -> name of function */
    Print(String, String), /* String -> Text to print stored on | rax:1(sys_write) , rsi:text , rdx:size/len_of_text , rdi:1 (0 for stdout)*/
}

#[derive(Debug, Clone)]

pub struct FN {
    pub name: String,
    pub is_global: bool,
    pub code: Vec<Tokens>,
    pub args: Vec<Args>,
    /* variable system */
}

#[allow(unused)]
#[derive(Debug, Clone)]

pub enum Args /* type(name_of_arg)*/ {
    Str(String),
    Int(String),
    Float(String),
    EMP(String),
    E,
}
impl Args {
    pub fn new(name: String, t: &str) -> Args {
        match t {
            "string" => Args::Str(name),
            "int" => Args::Int(name),
            "float" => Args::Float(name),
            "emp" => Args::EMP(name),
            _ => Args::E,
        }
    }
}

// Example implementation
impl FN {
    pub fn new(name: String, is_global: bool, code: Vec<Tokens>, args: Vec<Args>) -> Self {
        FN {
            name,
            is_global,
            code,
            args,
        }
    }

    pub fn add_code(&mut self, tkn: Tokens) {
        self.code.push(tkn);
    }
}
