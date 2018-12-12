macro_rules! result_to_err_code {
    ($result:ident) => {
        match $result {
            Ok(_) => ErrorCode::Success,
            Err(err) => err.to_error_code()
        };
    }
}

macro_rules! result_to_err_code_1 {
    ($result:ident, $default_value:expr) => {
        match $result {
            Ok(res) => (ErrorCode::Success, res),
            Err(err) => (err.to_error_code(), $default_value)
        };
    }
}

macro_rules! result_to_err_code_2 {
    ($result:ident, $default_value1:expr, $default_value2:expr) => {
        match $result {
            Ok((res1, res2)) => (ErrorCode::Success, res1, res2),
            Err(err) => (err.to_error_code(), $default_value1, $default_value2)
        };
    }
}

macro_rules! result_to_err_code_3 {
    ($result:ident, $default_value1:expr, $default_value2:expr, $default_value3:expr) => {
        match $result {
            Ok((res1, res2, res3)) => (ErrorCode::Success, res1, res2, res3),
            Err(err) => (err.to_error_code(), $default_value1, $default_value2, $default_value3)
        };
    }
}

macro_rules! result_to_err_code_4 {
    ($result:ident, $default_value1:expr, $default_value2:expr, $default_value3:expr, $default_value4:expr) => {
        match $result {
            Ok((res1, res2, res3, res4)) => (ErrorCode::Success, res1, res2, res3, res4),
            Err(err) => (err.to_error_code(), $default_value1, $default_value2, $default_value3, $default_value4)
        };
    }
}

macro_rules! unwrap_opt_or_return {
    ($opt:expr, $err:expr) => {
        match $opt {
            Some(val) => val,
            None => return $err
        };
    }
}

macro_rules! unwrap_or_return {
    ($result:expr, $err:expr) => {
        match $result {
            Ok(res) => res,
            Err(_) => return $err
        };
    }
}
