use error::{VcxErrorKind, VcxError, VcxResult};

pub fn get_or_default(config: &Option<String>, default: &str) -> String {
    config.to_owned().unwrap_or(default.to_string())
}

pub fn get_or_err(config: &Option<String>, err: Option<VcxErrorKind>) -> VcxResult<String> {
    let e = VcxError::from(err.unwrap_or(VcxErrorKind::InvalidOption));
    config.to_owned().ok_or(e)
}
