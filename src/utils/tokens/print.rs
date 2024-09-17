use crate::utils::types::Tokens;

pub fn process_print(num: &mut i32, text: &str) -> Tokens {
    *num += 1;
    return Tokens::Print(
        text.trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string(),
        format!("p{}", num),
    );
}
