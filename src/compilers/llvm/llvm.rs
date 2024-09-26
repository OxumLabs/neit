use crate::utils::types::{Args, Tokens, FN};

pub fn to_llvm(tokens: Vec<Tokens>) -> String {
    let mut mainll = String::new();
    let mut funs = String::new();

    // Declare libc printf function
    funs.push_str("declare i32 @printf(i8*, ...)\n");

    // Define print functions as inline implementations
    funs.push_str(
        "define void @printInt(i32 %value) {\n\
         entry:\n\
             %fmt = alloca [4 x i8], align 1\n\
             store [4 x i8] c\"%d\\00\", [4 x i8]* %fmt, align 1\n\
             %0 = getelementptr [4 x i8], [4 x i8]* %fmt, i32 0, i32 0\n\
             call i32 @printf(i8* %0, i32 %value)\n\
             ret void\n\
         }\n\
         \n\
         define void @printString(i8* %message) {\n\
         entry:\n\
             call i32 @printf(i8* %message)\n\
             ret void\n\
         }\n",
    );

    // Process each token
    for token in &tokens {
        match token {
            Tokens::Func(fun) => process_function(fun, &mut funs, &mut mainll),
            _ => {}
        }
    }

    // Start main function
    mainll.push_str("define void @main() {\nentry:\n");

    // Add your main function body logic here if needed
    mainll.push_str("ret void\n}\n"); // Close the main function

    // Append function definitions
    mainll.push_str(&funs);

    mainll // Return the generated LLVM IR
}

// Helper function to process function tokens
fn process_function(fun: &FN, funs: &mut String, main: &mut String) {
    let args_str: Vec<String> = fun
        .args
        .iter()
        .map(|arg| match arg {
            Args::Str(_) => format!("i8*"),
            Args::Int(_) => format!("i32"),
            Args::Float(_) => format!("double"),
            _ => String::from("void"), // Default case, modify as needed
        })
        .collect();
    let args = args_str.join(", ");

    // Generate the LLVM IR for the function
    funs.push_str(&format!(
        "define void @{}({}) {{\n\
        entry:\n",
        fun.name, args
    ));

    // Generate the LLVM IR for the function body
    llvm(&fun.code, funs, main);

    funs.push_str("}\n");
}

fn llvm(tokens: &[Tokens], funs: &mut String, main: &mut String) {
    for token in tokens {
        match token {
            Tokens::Func(_) => {
                continue;
            }
            Tokens::Print(text, var_name) => {
                let message = format!("{}: {}", text, var_name);
                funs.push_str(&format!(
                    "call void @printString(i8* getelementptr([{} x i8], [{} x i8]* @str_{} , i32 0, i32 0))\n",
                    message.len() + 1, // +1 for null terminator
                    message.len() + 1,
                    message.len()
                ));
            }
            _ => {}
        }
    }
}
