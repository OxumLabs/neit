use super::maths::evaluate_expression;

#[derive(Debug, Clone)]

pub enum Tokens {
    Func(FN),
    FnCall(String),          /*  String -> name of function */
    Print(String, String), /* String -> Text to print stored on | rax:1(sys_write) , rsi:text , rdx:size/len_of_text , rdi:1 (1 for stdout)*/
    Var(Vars, String, bool), /* Vars -> Variable Data | String -> Variable Name | bool -> is change-able*/
}
pub fn get_vars(tokens: &Vec<fvars>) -> Vec<Vars> {
    let mut vrs: Vec<Vars> = Vec::new();
    for i in tokens {
        vrs.push(i.v.clone());
    }
    return vrs;
}
pub fn get_vars_tkns(tokens: &Vec<Tokens>) -> Vec<Vars> {
    let mut vrs: Vec<Vars> = Vec::new();
    for i in tokens {
        match i {
            Tokens::Var(v, _, _) => vrs.push(v.clone()),
            _ => {}
        }
    }
    return vrs;
}

#[derive(Debug, Clone)]

pub struct FN {
    pub name: String,
    pub is_global: bool,
    pub code: Vec<Tokens>,
    pub args: Vec<Args>,
    pub local_vars: Vec<fvars>,
    /* variable system */
}
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct fvars {
    pub v: Vars,
    pub n: String,
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
impl FN {
    pub fn new(
        name: String,
        is_global: bool,
        code: Vec<Tokens>,
        args: Vec<Args>,
        local_vars: Vec<fvars>,
    ) -> Self {
        FN {
            name,
            is_global,
            code,
            args,
            local_vars,
        }
    }

    pub fn add_code(&mut self, tkn: Tokens) {
        self.code.push(tkn);
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Vars {
    STR(String),
    INT(i64),
    F(f64),
    EX(String),
}
#[allow(dead_code)]

impl Vars {
    pub fn to_asm(&self, name: String, counter: i32) -> String {
        match self {
            Vars::STR(value) => format!(
                "\n{name}_{counter} db '{value}', 0\n",
                name = name,
                value = value
            ),
            Vars::INT(value) => format!("\n{name} dq {value}\n", name = name, value = value),
            Vars::F(value) => format!(
                "\n{name} dq {value}\n",
                name = name,
                value = f64_to_bits(*value) // Convert float to its bit representation
            ),
            Vars::EX(_) => String::from("\n"), // We'll skip EX for now since it's more complex
        }
    }
    pub fn new() -> Vars {
        Vars::STR("___|___".to_string())
    }
    pub fn update_type(&mut self, value: &str, vrs: &Vec<Tokens>) -> Result<Vars, String> {
        let value = value.trim();

        // Check if the value is a string (enclosed in quotes)
        if value.starts_with('"') && value.ends_with('"') {
            let str_value = value.trim_matches('"').to_string();
            *self = Vars::STR(str_value);
            return Ok(self.clone());
        }

        // Attempt to parse the value as an integer
        if let Ok(int_value) = value.parse::<i64>() {
            *self = Vars::INT(int_value);
            return Ok(self.clone());
        }

        // Attempt to parse the value as a float
        if let Ok(float_value) = value.parse::<f64>() {
            *self = Vars::F(float_value);
            return Ok(self.clone());
        }

        // Check if the value is a valid expression
        match evaluate_expression(value, vrs) {
            Ok(result) => {
                if result.to_string().contains(".") {
                    *self = Vars::F(result);
                } else {
                    match result.to_string().parse::<i64>() {
                        Ok(int_value) => *self = Vars::INT(int_value),
                        Err(_) => return Err("Error: Unable to parse value as integer".to_string()),
                    }
                }
                return Ok(self.clone());
            }

            Err(e) => {
                return Err(format!(
                "✘ Error: Value '{}' could not be parsed as a valid type.\n\
                Hint: Ensure the value is in a valid format for string (\"string\"), integer (123), float (123.45), or expression (e.g., a+b).\nERROR: {}",
                value,e
            ));
            }
        }

        // If all parsing attempts fail, return an error
    }
}

fn f64_to_bits(f: f64) -> u64 {
    f.to_bits()
}
