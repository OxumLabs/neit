use super::err_types::ErrTypes;

pub fn gen_error_msg(err_type: ErrTypes,code : &String) -> String {
    match err_type {
        ErrTypes::UnknownCMD(line) => {
            format!("Error at line {} ~ Unknown refrence , are you sure whatever this is , it exists??\ncode piece : {}",line,code.lines().nth(line as usize).unwrap())
        },
    }
}