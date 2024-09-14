#[derive(Debug)]
pub enum Tokens {
    Func(FN),
}

#[derive(Debug)]

pub struct FN {
    pub name: String,
    pub is_global: bool,
    pub code: Vec<Tokens>,
    pub args: Vec<Args>,
    /* variable system */
}

#[allow(unused)]
#[derive(Debug)]

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
