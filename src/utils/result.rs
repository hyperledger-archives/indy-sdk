macro_rules! result_to_err_code {
    ($result:ident) => {
        match $result {
            Ok(ref res) => ErrorCode::Success,
            Err(ref err) => err.to_error_code()
        };
    }
}