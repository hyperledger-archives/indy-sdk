use ErrorCode;

use std::sync::mpsc::Receiver;
use std::time::Duration;

pub struct ResultHandler {}

impl ResultHandler {
    pub fn empty(err: ErrorCode, receiver: Receiver<ErrorCode>) -> Result<(), ErrorCode> {
        err.try_err()?;
        match receiver.recv() {
            Ok(err) => err.try_err(),
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn empty_timeout(err: ErrorCode, receiver: Receiver<ErrorCode>, timeout: Duration) -> Result<(), ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok(err) => err.try_err(),
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn one<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>) -> Result<T, ErrorCode> {
        err.try_err()?;

        let (err, val) = receiver.recv()?;

        err.try_err()?;

        Ok(val)
    }

    pub fn one_timeout<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>, timeout: Duration) -> Result<T, ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok((err, val)) =>  {
                err.try_err()?;
                Ok(val)
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn two<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>) -> Result<(T1, T2), ErrorCode> {
        err.try_err()?;

        let (err, val, val2) = receiver.recv()?;

        err.try_err()?;

        Ok((val, val2))
    }

    pub fn two_timeout<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>, timeout: Duration) -> Result<(T1, T2), ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok((err, val1, val2)) =>  {
                err.try_err()?;
                Ok((val1, val2))
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }

    pub fn three<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>) -> Result<(T1, T2, T3), ErrorCode> {
        err.try_err()?;

        let (err, val, val2, val3) = receiver.recv()?;

        err.try_err()?;

        Ok((val, val2, val3))
    }

    pub fn three_timeout<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>, timeout: Duration) -> Result<(T1, T2, T3), ErrorCode> {
        err.try_err()?;

        match receiver.recv_timeout(timeout) {
            Ok((err, val1, val2, val3)) =>  {
                err.try_err()?;
                Ok((val1, val2, val3))
            },
            Err(e) => Err(ErrorCode::from(e))
        }
    }
}
