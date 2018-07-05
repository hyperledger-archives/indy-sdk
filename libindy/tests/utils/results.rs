use indy::api::ErrorCode;

use std::sync::mpsc::Receiver;

pub fn result_to_empty(err: ErrorCode, receiver: Receiver<ErrorCode>) -> Result<(), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let err = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok(())
}

pub fn result_to_int(err: ErrorCode, receiver: Receiver<(ErrorCode, i32)>) -> Result<i32, ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok(val)
}

pub fn result_to_bool(err: ErrorCode, receiver: Receiver<(ErrorCode, bool)>) -> Result<bool, ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok(val)
}

pub fn result_to_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String)>) -> Result<String, ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok(val)
}

pub fn result_to_string_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String, String)>) -> Result<(String, String), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val, val2) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok((val, val2))
}

pub fn result_to_string_string_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String, String, String)>) -> Result<(String, String, String), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val, val2, val3) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok((val, val2, val3))
}

pub fn result_to_string_opt_string_opt_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String, Option<String>, Option<String>)>) -> Result<(String, Option<String>, Option<String>), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val, val2, val3) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok((val, val2, val3))
}

pub fn result_to_opt_string(err: ErrorCode, receiver: Receiver<(ErrorCode, Option<String>)>) -> Result<Option<String>, ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok(val)
}

pub fn result_to_string_opt_string(err: ErrorCode, receiver: Receiver<(ErrorCode, String, Option<String>)>) -> Result<(String, Option<String>), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val, val2) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok((val, val2))
}

pub fn result_to_vec_u8(err: ErrorCode, receiver: Receiver<(ErrorCode, Vec<u8>)>) -> Result<Vec<u8>, ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, vec) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok(vec)
}

pub fn result_to_string_vec_u8(err: ErrorCode, receiver: Receiver<(ErrorCode, String, Vec<u8>)>) -> Result<(String, Vec<u8>), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, str, vec) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok((str, vec))
}


pub fn result_to_string_string_u64(err: ErrorCode, receiver: Receiver<(ErrorCode, String, String, u64)>) -> Result<(String, String, u64), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val, val2, val3) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok((val, val2, val3))
}