use crate::p::{VVal, Var, NST};
use colored::*;
use std::collections::HashMap;

pub static INDENTLEV: &str = "    ";

pub fn codegen(nst: &mut Vec<NST>, addh: bool, generate_main: bool, addstrcmp: bool) -> String {
    let mut ccode = String::new();
    let mut vars: HashMap<String, VVal> = HashMap::new();
    let mut func_body = String::new();
    if addstrcmp {
        //println!("[DEBUG] ~ adding strmp");
        ccode.push_str(r#""#);
        ccode.push('\n');
    }

    if addh {
        println!("{}", "-> Adding headers".green().bold());
        ccode.push_str("#include \"nulibc.h\"\n\n\n");
    }

    if nst.iter().any(|mc| matches!(mc, NST::NCLRSCRN)) {
        println!("{}", "-> Clearscreen function detected".blue());
    }

    println!("{}", "-> Walking down NST tree...".green().bold());

    for mc in &mut *nst {
        match mc {
            NST::EX(code) => {
                func_body.push_str(&format!("nexit({});\n",code));
            }
            NST::NCLRSCRN => {
                //if !added_nclrscrn {
                    func_body.push_str("__NCLRSCRN__();\n");
                 //   added_nclrscrn = true;
               // }
            }
            NST::PRINT(txt) => {
                let print_code = generate_print_code(txt, &vars);
                func_body.push_str(&print_code);
            }
            NST::Func(name, _args, nsts) => {
                let body_code = codegen(nsts, false, false, false);
                ccode.push_str(&format!("void {}() {{\n", name));
                ccode.push_str(&body_code);
                ccode.push_str("}\n");
            }
            NST::Var(v) => {
                vars.insert(v.name.clone(), v.value.clone());
                func_body.push_str(&generate_var_code(v));
            }
            NST::Input(v) => {
                func_body.push_str(&format!(
                    "char {}[2048];\nscanf(\"%2047[^\\n]\", {});\n",
                    v, v
                ));
                vars.insert(v.to_string(), VVal::Str(String::from("")));
            }
            NST::VRDInput(v) => {
                func_body.push_str(&format!("   scanf(\"%2047[^\\n]\", {});\n", v));
                vars.insert(v.to_string(), VVal::Str(String::from("")));
            }
            NST::WAIT(t) => {
                func_body.push_str(format!("usleep({}LL*1000);\n", t).as_str());
            }
            NST::NIF(cond, code) => {
                //println!("code : {:?}",code);
                func_body.push_str(
                    format!(
                        "if ({}){{\n{}\n}}\n",
                        cond.c_code,
                        codegen(code, false, false, false)
                    )
                    .as_str(),
                );
            }
            NST::VarRD(n, v) => match v {
                VVal::Str(s) => {
                    func_body.push_str(format!("{} = {};\n", n, s).as_str());
                }
                VVal::Int(i) => {
                    func_body.push_str(format!("{} = {};\n", n, i).as_str());
                }
                VVal::F(f) => {
                    func_body.push_str(format!("{} = {};\n", n, f).as_str());
                }
                VVal::VarRef(n2, _v2) => {
                    func_body.push_str(format!("{} = {};\n", n, n2).as_str());
                }
            },
            NST::NWHILE(cond, body) => {
                let body = codegen(body, false, false, false);
                func_body.push_str(&format!("while ({}) {{\n", cond.c_code));
                func_body.push_str(&body);
                func_body.push_str("}\n");
            }
        }
    }

    if generate_main {
        ccode.push_str("int main() {\n");
        ccode.push_str(&func_body);
        ccode.push_str("\n    return 0;\n}\n");
    } else {
        ccode.push_str(&func_body);
    }

    println!("{}", "-> Code generation completed".green().bold());
    ccode
}

fn generate_print_code(txt: &str, vars: &HashMap<String, VVal>) -> String {
    let mut format_str = String::new();
    let mut var_names: Vec<String> = Vec::new();
    let mut in_var_mode = false;
    let mut escape_mode = false;
    let mut current_var = String::new();

    for c in txt.chars() {
        if escape_mode {
            match c {
                '{' | '}' | '\\' => format_str.push(c),
                _ => {
                    format_str.push('\\');
                    format_str.push(c);
                }
            }
            escape_mode = false;
        } else if c == '\\' {
            escape_mode = true;
        } else if c == '{' {
            if in_var_mode {
                current_var.push(c);
            } else {
                in_var_mode = true;
                current_var.clear();
            }
        } else if c == '}' {
            if in_var_mode {
                in_var_mode = false;

                if let Some(var_val) = vars.get(&current_var) {
                    match var_val {
                        VVal::Str(_) => format_str.push_str("%s"),
                        VVal::Int(_) => format_str.push_str("%d"),
                        VVal::F(_) => format_str.push_str("%f"),
                        VVal::VarRef(_, _) => format_str.push_str("%d"),
                    }
                    var_names.push(current_var.clone());
                }
            } else {
                format_str.push(c);
            }
        } else if in_var_mode {
            current_var.push(c);
        } else {
            format_str.push(c);
        }
    }

    if !var_names.is_empty() {
        format!(
            "    nprintf(STDOUT,\"{}\", {});\n    \n",
            format_str,
            var_names.join(", ")
        )
    } else {
        format!("    nprintf(STDOUT,\"{}\");\n \n", format_str)
    }
}

fn generate_var_code(v: &Var) -> String {
    match &v.value {
        VVal::Str(s) => {
            format!("    char {}[2048] = \"{}\";\n", v.name, s)
        }
        VVal::Int(i) => format!("    int {} = {};\n", v.name, i),
        VVal::F(f) => format!("    float {} = {};\n", v.name, f),
        VVal::VarRef(_, _) => String::new(),
    }
}
