use ErrorCode;

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

pub fn result_to_one<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>) -> Result<T, ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok(val)
}

pub fn result_to_two<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>) -> Result<(T1, T2), ErrorCode> {
    if err != ErrorCode::Success {
        return Err(err);
    }

    let (err, val, val2) = receiver.recv().unwrap();

    if err != ErrorCode::Success {
        return Err(err);
    }

    Ok((val, val2))
}
