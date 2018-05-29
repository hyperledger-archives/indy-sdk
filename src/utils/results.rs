use ErrorCode;

use std::sync::mpsc::Receiver;
use std::time::Duration;

pub struct ResultHandler {}

impl ResultHandler {
    pub fn empty(err: ErrorCode, receiver: Receiver<ErrorCode>) -> Result<(), ErrorCode> {
        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn empty_timeout(err: ErrorCode, receiver: Receiver<ErrorCode>, timeout: Duration) -> Result<(), ErrorCode> {
        if err != ErrorCode::Success {
            return Err(err);
        }

        match receiver.recv_timeout(timeout) {
            Ok(err) => {
                if err != ErrorCode::Success {
                    return Err(err);
                }
                Ok(())
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn one<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>) -> Result<T, ErrorCode> {
        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, val) = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(val)
    }

    pub fn one_timeout<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>, timeout: Duration) -> Result<T, ErrorCode> {
        if err != ErrorCode::Success {
            return Err(err);
        }

        match receiver.recv_timeout(timeout) {
            Ok((err, val)) =>  {
                if err != ErrorCode::Success {
                    return Err(err);
                }
                Ok(val)
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn two<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>) -> Result<(T1, T2), ErrorCode> {
        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, val, val2) = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((val, val2))
    }

    pub fn three<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>) -> Result<(T1, T2, T3), ErrorCode> {
        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, val, val2, val3) = receiver.recv().unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok((val, val2, val3))
    }
}
