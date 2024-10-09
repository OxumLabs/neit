use super::{token::gentoken, types::Tokens};

pub fn process_case(
    _ln: &str,
    code: Vec<&str>,
    _index: &mut i64,
    ogtkns: &Vec<Tokens>,
    _fc: bool,
) -> Result<Vec<Tokens>, String> {
    let ctkns = gentoken(code, ogtkns.to_vec(), true);
    match ctkns {
        Ok(ctkns) => {
            return Ok(ctkns);
        }
        Err(e) => return Err(format!("Error inside case block : {}", e)),
    }
}
