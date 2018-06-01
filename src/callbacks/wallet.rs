use ErrorCode;

pub type CreateWalletClosure = FnMut(String, String, String) -> ErrorCode + Send;
