use super::{token::gentoken, types::Tokens};

pub fn process_case(
    _ln: &str,
    code: Vec<String>,
    _index: &mut i64,
    ogtkns: &Vec<Tokens>,
    _fc: bool,
) -> Result<Vec<Tokens>, String> {
    //println!("codes in case : \n{:?}", code);
    let ctkns = gentoken(code, ogtkns.to_vec(), true);
    match ctkns {
        Ok(ctkns) => {
            return Ok(ctkns);
        }
        Err(e) => {
            return Err(format!(
                "✘ Error: Issue Inside Case Block\n\
                Error encountered in case block: {}\n\
                ➔ Reason: An unexpected issue occurred during case execution.\n\
                ➔ Suggested Action: Review your case conditions for correctness.\n\
                ➔ Hint: Ensure expressions inside the case block are valid and free from syntax errors.\n\
                Let’s debug this to get it sorted out!",
                e
            ));
        }
    }
}
