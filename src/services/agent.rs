use errors::common::CommonError;

pub struct AgentService;

impl AgentService {
    pub fn new() -> AgentService {
        AgentService {}
    }

    pub fn connect(&self) -> Result<(), CommonError> {
        unimplemented!();
    }
}