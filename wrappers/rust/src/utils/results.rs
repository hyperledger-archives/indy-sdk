use ErrorCode;

use std::sync::mpsc::Receiver;
use std::time::Duration;

pub struct ResultHandler {}

impl ResultHandler {
    pub fn empty(err: ErrorCode, receiver: Receiver<ErrorCode>) -> Result<(), ErrorCode> {
        err.try_err()?;
        let err = receiver.recv()?;
        err.try_err()
    }

    pub fn empty_timeout(err: ErrorCode, receiver: Receiver<ErrorCode>, timeout: Duration) -> Result<(), ErrorCode> {
        err.try_err()?;
        let err = receiver.recv_timeout(timeout)?;
        err.try_err()
    }

    pub fn one<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>) -> Result<T, ErrorCode> {
        err.try_err()?;
        let (err, val) = receiver.recv()?;
        err.try_err()?;
        Ok(val)
    }

    pub fn one_timeout<T>(err: ErrorCode, receiver: Receiver<(ErrorCode, T)>, timeout: Duration) -> Result<T, ErrorCode> {
        err.try_err()?;
        let (err, val) = receiver.recv_timeout(timeout)?;
        err.try_err()?;
        Ok(val)
    }

    pub fn two<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>) -> Result<(T1, T2), ErrorCode> {
        err.try_err()?;
        let (err, val, val2) = receiver.recv()?;
        err.try_err()?;
        Ok((val, val2))
    }

    pub fn two_timeout<T1, T2>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2)>, timeout: Duration) -> Result<(T1, T2), ErrorCode> {
        err.try_err()?;
        let (err, val, val2) = receiver.recv_timeout(timeout)?;
        err.try_err()?;
        Ok((val, val2))
    }

    pub fn three<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>) -> Result<(T1, T2, T3), ErrorCode> {
        err.try_err()?;
        let (err, val, val2, val3) = receiver.recv()?;
        err.try_err()?;
        Ok((val, val2, val3))
    }

    pub fn three_timeout<T1, T2, T3>(err: ErrorCode, receiver: Receiver<(ErrorCode, T1, T2, T3)>, timeout: Duration) -> Result<(T1, T2, T3), ErrorCode> {
        err.try_err()?;
        let (err, val, val2, val3) = receiver.recv_timeout(timeout)?;
        err.try_err()?;
        Ok((val, val2, val3))
    }
}
