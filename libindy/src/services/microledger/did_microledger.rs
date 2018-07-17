use services::microledger::microledger::Microledger;

pub struct DidMicroledger {
    name: String
}

impl Microledger for DidMicroledger {
    fn new(name: &str) -> Self {
        DidMicroledger {
            name: name.to_string()
        }
    }
}