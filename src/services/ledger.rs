use errors::ledger::LedgerError;

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> LedgerService {
        LedgerService {}
    }

    pub fn send_nym_tx(&self, issuer: &str, dest: &str, verkey: Option<&str>,
                       xref: Option<&str>, data: Option<&str>,
                       role: Option<&str>) -> Result<(), LedgerError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ledger_service_can_be_created() {
        let ledger_service = LedgerService::new();
        assert!(true, "No crashes on LedgerService::new");
    }

    #[test]
    fn ledger_service_can_be_dropped() {
        fn drop_test() {
            let ledger_service = LedgerService::new();
        }

        drop_test();
        assert!(true, "No crashes on LedgerService::drop");
    }
}