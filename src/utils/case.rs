use super::{token::gentoken, types::Tokens};

pub fn process_case(
    _ln: &str,
    code: Vec<&str>,
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
            "✘ Error: Issue Inside Case Block\n\n\
            Something went wrong inside the case block: {}\n\n\
            ➔ What Happened: This error suggests there was an unexpected issue when executing the case logic. \n\
            ➔ Suggested Action: Review the case conditions and ensure that they are correctly implemented. \n\
            ➔ Hint: Make sure the expressions inside the case block are valid and that there are no syntax errors.\n\n\
            Let’s debug this and get it sorted out!"
        , e));
        }
    }
}
