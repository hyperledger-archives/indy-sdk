use utils::error;


pub fn indy_error_to_cxs_error_code(err: i32) ->  u32 {
    match err {
        0 => error::SUCCESS.code_num,
        _ => error::UNKNOWN_ERROR.code_num,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_error(){
        let indy_error = 0;
        let cxs_error = &error::SUCCESS;
        assert_eq!(indy_error_to_cxs_error_code(indy_error), cxs_error.code_num);

        let indy_error = 206;
        let cxs_error = &error::UNKNOWN_ERROR;
        assert_eq!(indy_error_to_cxs_error_code(indy_error), cxs_error.code_num);

    }


}