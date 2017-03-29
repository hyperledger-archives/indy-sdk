use errors::sovrin::SovrinError;

pub struct SovrinService {}

impl SovrinService {
    pub fn new() -> SovrinService {
        SovrinService {}
    }

    pub fn send_nym_tx(&self, issuer: &str, dest: &str, verkey: Option<&str>,
                       xref: Option<&str>, data: Option<&str>,
                       role: Option<&str>) -> Result<(), SovrinError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sovrin_service_can_be_created() {
        let sovrin_service = SovrinService::new();
        assert!(true, "No crashes on SovrinService::new");
    }

    #[test]
    fn sovrin_service_can_be_dropped() {
        fn drop_test() {
            let sovrin_service = SovrinService::new();
        }

        drop_test();
        assert!(true, "No crashes on SovrinService::drop");
    }
}