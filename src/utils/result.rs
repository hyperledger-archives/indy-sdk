use api::ErrorCode;

macro_rules! result_to_err_code {
    ($result:ident) => {
        match $result {
            Ok(res) => ErrorCode::Success,
            Err(err) => From::from(err)
        };
    }
}