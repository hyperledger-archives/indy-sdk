use indy::api::ErrorCode as indyError;
use api::Errorcode as cxsError;

pub fn indy_to_cxs_error_code(err:indyError) -> cxsError {
    match err {
        indyError::Success => cxsError::Success,
        _ => cxsError::Failure,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_error(){
        let indy_error = indyError::Success;
        let cxs_error = cxsError::Success;
        assert_eq!(indy_to_cxs_error_code(indy_error),  cxs_error );
    }


}