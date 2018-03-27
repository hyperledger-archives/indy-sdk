pub mod connection;
pub mod base;
pub mod schema;
pub mod cred_def;
pub mod issuer_cred;
pub mod proof;


pub trait ToErrorCode {
    fn to_error_code(&self) -> u32;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_error_code(){

    }
}