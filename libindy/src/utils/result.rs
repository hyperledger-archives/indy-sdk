macro_rules! prepare_result {
    ($result:ident) => {{
        trace!("prepare_result: >>> {:?}", $result);
        match $result {
            Ok(_) => ErrorCode::Success,
            Err(err) => err.into()
        }
    }}
}

macro_rules! prepare_result_1 {
    ($result:ident, $default_value:expr) => {{
        trace!("prepare_result_1: >>> {:?}", $result);
        match $result {
            Ok(res) => (ErrorCode::Success, res),
            Err(err) => {
                (err.into(), $default_value)
            }
        }
    }}
}

macro_rules! prepare_result_2 {
    ($result:ident, $default_value1:expr, $default_value2:expr) => {{
        trace!("prepare_result_2: >>> {:?}", $result);
        match $result {
            Ok((res1, res2)) => (ErrorCode::Success, res1, res2),
            Err(err) => {
                (err.into(), $default_value1, $default_value2)
            }
        }
    }}
}

macro_rules! prepare_result_3 {
    ($result:ident, $default_value1:expr, $default_value2:expr, $default_value3:expr) => {{
        trace!("prepare_result_3: >>> {:?}", $result);
        match $result {
            Ok((res1, res2, res3)) => (ErrorCode::Success, res1, res2, res3),
            Err(err) => {
                (err.into(), $default_value1, $default_value2, $default_value3)
            }
        }
    }}
}

macro_rules! prepare_result_4 {
    ($result:ident, $default_value1:expr, $default_value2:expr, $default_value3:expr, $default_value4:expr) => {{
        trace!("prepare_result_4: >>> {:?}", $result);
        match $result {
            Ok((res1, res2, res3, res4)) => (ErrorCode::Success, res1, res2, res3, res4),
            Err(err) => {
                (err.into(), $default_value1, $default_value2, $default_value3, $default_value4)
            }
        }
    }}
}

macro_rules! unwrap_opt_or_return {
    ($opt:expr, $err:expr) => {
        match $opt {
            Some(val) => val,
            None => return $err
        }
    }
}

macro_rules! unwrap_or_return {
    ($result:expr, $err:expr) => {
        match $result {
            Ok(res) => res,
            Err(_) => return $err
        }
    }
}
