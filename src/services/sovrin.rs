use std::error::Error;

pub struct SovrinService {
    dummy: String
}

impl SovrinService {
    pub fn new() -> SovrinService {
        trace!(target: "SovrinService", "new");
        SovrinService { dummy: "sovrin_dummy".to_string() }
    }

    pub fn set_did(&self, did: String) -> Result<String, Box<Error>> {
        trace!(target: "SovrinService", "set_did {:?}", did);
        Ok(did)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_creation_is_possible() {
        let sovrin_service = SovrinService::new();
        assert_eq!("sovrin_dummy", sovrin_service.dummy, "Dummy field is filled by constructor");
    }
}