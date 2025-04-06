use super::err_types::ErrTypes;
use colored::Colorize;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::{rng, thread_rng};
use std::convert::TryInto;

// A large collection of taunting messages to incite frustration.
static TAUNTS: &[&str] = &[
    "Your code is so atrocious, the compiler weeps in despair!",
    "If incompetence were an art, you'd be a masterpiece of disaster!",
    "Your code is a train wreck that even the wrecking ball wouldn't fix!",
    "I wouldn't call that code; it's an utter catastrophe!",
    "Your code is so buggy, it's a public health hazard!",
    "I've seen broken crayons that work better than your logic!",
    "Your code is a dumpster fire that even the fire department can't save!",
    "Your programming skills are so abysmal, even the errors mock you!",
    "Your code is so incomprehensible, a toddler could write better!",
    "Be proud—your code is the definition of chaos!",
    "Your code is a masterpiece of disaster; the computer begs for mercy!",
    "I've seen walls with more structure than your functions!",
    "Your code makes a car crash look like a smooth ride!",
    "Did you even try? That code is a total dumpster fire!",
    "Your errors are so frequent, they're practically your best friends!",
    "Your code is so tangled, it's like you threw spaghetti at your keyboard!",
    "Your logic is so warped, it’s a black hole of bad decisions!",
    "Even your error messages have resigned in despair!",
    "Your code is so pitiful, it deserves to be locked away forever!",
    "If mediocrity were a superpower, you'd be unstoppable!",
    "Your code is a punchline in a joke no one finds funny!",
    "Your programming is so disastrous, it's almost impressive!",
    "You must really love bugs—your code is a breeding ground!",
    "Your code is so horribly written, even the debugger gives up!",
    "Your functions are as useful as a chocolate teapot!",
    "If there were a contest for worst code, you'd win effortlessly!",
    "Your coding skills are a nightmare for compilers!",
    "Every run of your code spawns a new error epidemic!",
    "Your program is a textbook example of what not to do!",
    "Your syntax is a catastrophe, your logic a labyrinth of errors!",
    "Your code is so abysmal, it's almost an art form in failure!",
    "Your errors could fill a book titled 'How Not to Code'!",
    "Your coding is so pitiful, it makes the worst horror movies look uplifting!",
    "Even the simplest task becomes monumental in your code!",
    "Your code is so disorganized, it’s like a tornado hit a server farm!",
    "If code were a language, yours would be indecipherable gibberish!",
    "Your programming is so inept, it's practically a public service announcement!",
    "Your logic is so flawed, it should come with a disclaimer!",
    "Your code is a monument to your failure as a programmer!",
    "Every compile reminds you how much you need to learn!",
    "Your code is a perfect storm of errors and bad decisions!",
    "Your programming style is an affront to every language ever created!",
    "I wouldn't trust your code to run a simple calculator!",
    "Your code is so disastrously written, it makes others look like geniuses!",
    "If coding were a crime, you'd be serving a life sentence!",
    "Your logic is as clear as a murky swamp—dive in if you dare!",
    "Your code is an endless loop of mistakes with no exit condition!",
    "It's a miracle your program even runs—disaster lurks at every turn!",
    "Your code is so embarrassingly bad, even error messages cringe!",
    "Your coding is so appalling, it should be banned by all compilers!",
];

/// Returns a random taunt message from TAUNTS.
fn get_random_taunt() -> &'static str {
    let mut rng = rng();
    TAUNTS.choose(&mut rng).unwrap_or(&"")
}

/// Formats an error message in a compact style using vertical bars.
/// The message includes a header, the adjusted error line number,
/// the corresponding code piece, a hint, and a randomly selected taunt.
fn format_error_msg(header: &str, line: u32, hint: &str, code: &String) -> String {
    // Subtract one from the line number, avoiding underflow if line is 0.
    let adjusted_line = line.saturating_sub(1);
    let code_piece = code
        .lines()
        .nth(adjusted_line as usize)
        .unwrap_or("Code snippet unavailable");
    format!(
        "┌[{}] at line {}\n├ Code Piece: {}\n├ Hint: {}\n└ {}",
        header.red().bold(),
        adjusted_line,
        code_piece,
        hint.cyan(),
        get_random_taunt().yellow().bold()
    )
}

/// Generates an error message based on the error type and source code.
/// The returned message includes the adjusted line number, the code piece, and error details.
pub fn gen_error_msg(err_type: ErrTypes, _code: &String) -> String {
    println!("code:\n{}", _code);
    match err_type {
        ErrTypes::SyntaxError(line) => format_error_msg(
            "Syntax Error",
            line.try_into().unwrap(),
            "Check your syntax and try again",
            _code,
        ),
        ErrTypes::DivisionByZero(line) => format_error_msg(
            "Division By Zero",
            line.try_into().unwrap(),
            "Ensure the denominator is not zero",
            _code,
        ),
        ErrTypes::MissingOperator(line) => format_error_msg(
            "Missing Operator",
            line.try_into().unwrap(),
            "Insert the appropriate operator",
            _code,
        ),
        ErrTypes::UnexpectedToken(line) => format_error_msg(
            "Unexpected Token",
            line.try_into().unwrap(),
            "Review your tokens",
            _code,
        ),
        ErrTypes::TypeMismatch(line) => format_error_msg(
            "Type Mismatch",
            line.try_into().unwrap(),
            "Ensure types match as expected",
            _code,
        ),
        ErrTypes::MissingValue(line) => format_error_msg(
            "Missing Value",
            line.try_into().unwrap(),
            "Provide the missing value",
            _code,
        ),
        ErrTypes::ReservedKeyword(line) => format_error_msg(
            "Reserved Keyword",
            line.try_into().unwrap(),
            "Avoid using reserved keywords",
            _code,
        ),
        ErrTypes::UnbalancedParentheses(line) => format_error_msg(
            "Unbalanced Parentheses",
            line.try_into().unwrap(),
            "Balance your parentheses",
            _code,
        ),
        ErrTypes::VarNotFound(line) => format_error_msg(
            "Variable Not Found",
            line.try_into().unwrap(),
            "Declare or check the variable",
            _code,
        ),
        ErrTypes::UnknownCMD(line) => format_error_msg(
            "Unknown Command",
            line.try_into().unwrap(),
            "Check the command and try again",
            _code,
        ),
        ErrTypes::UnsupportedVarType(line) => format_error_msg(
            "Unsupported Variable Type",
            line.try_into().unwrap(),
            "Use a supported variable type",
            _code,
        ),
        ErrTypes::VarAlreadyExists(line) => format_error_msg(
            "Variable Already Exists",
            line.try_into().unwrap(),
            "Rename or remove the duplicate",
            _code,
        ),
        ErrTypes::CharVarLen(line) => format_error_msg(
            "Char Variable Length Error",
            line.try_into().unwrap(),
            "Check the character length",
            _code,
        ),
        ErrTypes::InvalidMathUsage(line) => format_error_msg(
            "Invalid Math Usage",
            line.try_into().unwrap(),
            "Review your math operations",
            _code,
        ),
        ErrTypes::DuplicateOperator(line) => format_error_msg(
            "Duplicate Operator",
            line.try_into().unwrap(),
            "Remove the extra operator",
            _code,
        ),
        ErrTypes::InvalidConditionSyntax(line) => format_error_msg(
            "Invalid Condition Syntax",
            line.try_into().unwrap(),
            "Correct the condition syntax",
            _code,
        ),
        ErrTypes::InvalidNumberFormat(line) => format_error_msg(
            "Invalid Number Format",
            line.try_into().unwrap(),
            "Ensure the number is correctly formatted",
            _code,
        ),
        ErrTypes::MissingLeftOperand(line) => format_error_msg(
            "Missing Left Operand",
            line.try_into().unwrap(),
            "Provide the left operand",
            _code,
        ),
        ErrTypes::MissingRightOperand(line) => format_error_msg(
            "Missing Right Operand",
            line.try_into().unwrap(),
            "Provide the right operand",
            _code,
        ),
        ErrTypes::UnexpectedEndOfInput(line) => format_error_msg(
            "Unexpected End Of Input",
            line.try_into().unwrap(),
            "Complete the input",
            _code,
        ),
        ErrTypes::UnsupportedOperator(line) => format_error_msg(
            "Unsupported Operator",
            line.try_into().unwrap(),
            "Use a supported operator",
            _code,
        ),
        ErrTypes::VarISConst(line) => format_error_msg(
            "Constant Variable Error",
            line.try_into().unwrap(),
            "Constants cannot be modified",
            _code,
        ),
    }
}
