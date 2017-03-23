use std::error::Error;

pub struct SovrinService {}

impl SovrinService {
    pub fn new() -> SovrinService {
        SovrinService {}
    }

    pub fn set_did(&self, did: String) -> Result<(), Box<Error>> {
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