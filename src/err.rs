use colored::*;

#[derive(Clone, Debug)]
#[allow(unused)]
/// Enum representing various error types that can occur during parsing.
#[allow(unused)]
/// Enum representing various error types that can occur during parsing.
pub enum ErrT {
    /// Invalid file import statement
    /// Holds:
    /// - `usize`: Line number where the error occurred
    /// - `String`: The invalid file path
    InvFILE(usize, String),
    /// Represents an invalid value assigned to a variable.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The problematic value or expression causing the error.
    InValidVarVal(usize, String),

    /// Represents a variable not found in the current scope.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The name of the variable that was not found.
    VNF(usize, String),

    /// Represents a string with unmatched quotes.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The string with the unmatched quotes.
    UnMQ(usize, String),

    /// Represents a missing or misplaced equal sign in a variable declaration.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The code fragment where the issue was detected.
    EqNF(usize, String),

    /// Invalid Time Value for 'wait' command
    InVTimeVal(usize, String),

    /// Represents unmatched parentheses in the code.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    UnmatchedParen(usize, String),

    /// Represents an invalid condition used in a control structure.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The invalid condition.
    InVCond(usize, String),

    /// Represents an empty condition in a control structure.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    EmptyCond(usize, String),

    /// Represents an incorrect "if" statement format.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    NIF(usize, String),

    /// Represents an invalid variable read operation.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    VarRD(usize, String),

    /// Represents an invalid conditional operator.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    InvalidCondOp(usize, String),
    /// Represents an invalid operand for a conditional operator
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The invalid operand
    InvalidOperand(usize, String),
    /// Represents an invalid value
    /// Holds:
    /// - `usize`: Line number where the error occurred
    /// - `String`: Invalid value was used for?
    /// - `String`: The invalid value itself
    InvVal(usize, String, String),
}
#[allow(unused)]
pub fn generr(err: ErrT, codes: &Vec<&str>) {
    match err {
        ErrT::InValidVarVal(line, value) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Invalid Variable Assignment".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The value `{}` assigned to the variable is ambiguous or incompatible.",
                    value
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "This error typically occurs when a value's type cannot be inferred,".bright_cyan()
            );
            println!(
                " │   {}",
                "or the value violates type rules (e.g., assigning a string to an integer)."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Check the assignment value for typos, or ensure it's compatible with the variable's type."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::InvFILE(ln, path) => {
            let codeline = &codes[ln - 1];
            println!("{}", "ERROR: Invalid File Import".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", ln).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!("Invalid file path: `{}`", path).yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "The file path provided for import is either incorrect or inaccessible."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure the file path is correct and the file is accessible from the current directory\nNOTE: you can type '..' to go back a directory and so on"
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::VNF(line, var_name) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Undeclared Variable Reference".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The variable `{}` has not been declared in the current scope.",
                    var_name
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "This occurs when attempting to use a variable name that is undefined or misspelled."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Declare the variable before using it, and ensure there are no typos in the name."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::UnMQ(line, string_literal) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Unmatched Quotes Detected".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The string `{}` starts or ends with a quote but lacks a matching pair.",
                    string_literal
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "String literals must start and end with matching quotes (either single or double)."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Check for missing or mismatched quotes in the provided string literal."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::EqNF(line, fragment) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Equal Sign Not Found".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The `=` sign is missing or misplaced in the statement: `{}`.",
                    fragment
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "The equal sign (`=`) is used to assign a value to a variable.".bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure that the `=` is placed after the variable name and before the value."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::InVTimeVal(line, fragment) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Invalid Time Value".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The time value is either missing or incorrectly formatted in the statement: `{}`.",
                    fragment
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "Ensure that the time value is correctly specified with a valid format."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "For example, use time values like `1s`, `500ms`, `2m`, `1hr` for seconds, milliseconds, minutes, or hours"
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::UnmatchedParen(line, code) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Unmatched Parentheses".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!("Unmatched parentheses found in: `{}`", code).yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "Ensure that all parentheses are properly paired.".bright_cyan()
            );
            println!(
                " │   {}",
                "Check for missing closing parentheses or misplaced opening parentheses."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::InVCond(line, cond) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Invalid Condition".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!("Invalid condition: `{}`", cond).yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "This error occurs when the condition used in a control structure is invalid."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure the condition is properly formed and returns a boolean result."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::EmptyCond(line, cond) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Empty Condition".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!("Empty condition found in: `{}`", cond).yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "A control structure cannot have an empty condition.".bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure that the condition is not missing or improperly formatted.".bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::NIF(line, code) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Incorrect If Statement".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!("Incorrect if statement structure: `{}`", code).yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "The if statement is not properly formatted.".bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure the condition and body are correctly placed inside the `if` statement."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::VarRD(line, code) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Variable Read Error".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!("Invalid variable read in: `{}`", code).yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "Attempted to read from a variable in an unsupported way.".bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure the variable is declared and read properly.".bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::InvalidCondOp(line, code) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Invalid Conditional Operator".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!("Invalid conditional operator in: `{}`", code).yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "An invalid conditional operator (`==`, `!=`, `<`, `>`, etc.) was used."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure that the operator is a valid conditional operator for comparisons."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::InvVal(line, vfor, value) => {
            let codeline = if line > 0 && line <= codes.len() {
                &codes[line - 1]
            } else {
                "<unknown code>"
            };

            if value.is_empty() {
                println!("{}", "ERROR: Missing Value".bold().red());
                println!(
                    " ├─ {} {}",
                    "Line:".bright_white(),
                    format!("{}", line).yellow().bold()
                );
                println!(
                    " ├─ {} {}",
                    "Cause:".bright_white(),
                    format!(
                        "The required value for `{}` is missing or not provided.",
                        vfor
                    )
                    .yellow()
                );
                println!(" ├─ {}", "Explanation:".bright_white());
                println!(
                    " │   {}",
                    "This error occurs when a required value is omitted.".bright_cyan()
                );
                println!(
                    " │   {}",
                    "Ensure you specify a valid value for the required operation or entity."
                        .bright_cyan()
                );
                println!(
                    " │   {}",
                    format!(
                        "For example, if `{}` expects a value, ensure it is defined.",
                        vfor
                    )
                    .bright_cyan()
                );
                println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
            } else {
                println!("{}", "ERROR: Invalid Value".bold().red());
                println!(
                    " ├─ {} {}",
                    "Line:".bright_white(),
                    format!("{}", line).yellow().bold()
                );
                println!(
                    " ├─ {} {}",
                    "Cause:".bright_white(),
                    format!(
                        "The value `{}` provided for `{}` is invalid or incompatible.",
                        value, vfor
                    )
                    .yellow()
                );
                println!(" ├─ {}", "Explanation:".bright_white());
                println!(
                    " │   {}",
                    "This error occurs when the provided value does not meet the expected criteria."
                        .bright_cyan()
                );
                println!(
                    " │   {}",
                    "Ensure the value is valid, compatible, and follows the required format."
                        .bright_cyan()
                );
                println!(
                    " │   {}",
                    format!(
                        "For example, `{}` should only accept values that meet specific criteria.",
                        vfor
                    )
                    .bright_cyan()
                );
                println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
            }
        }

        _ => {}
    }
}
