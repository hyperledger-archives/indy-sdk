extern crate libc;

use errors::indy::IndyError;
use services::payments::{PaymentsMethodCBs, PaymentsService};

use std::rc::Rc;

pub enum PaymentsCommand {
    RegisterMethod(
        String, //type
        PaymentsMethodCBs, //method callbacks
        Box<Fn(Result<(), IndyError>) + Send>),
    #[allow(dead_code)]
    CreateAddress(
        String, //type
        String, //config
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateAddressAck(
        i32, //handle
        Result<String /* address */, IndyError>),
}

pub struct PaymentsCommandExecutor {
    payments_service: Rc<PaymentsService>
}

impl PaymentsCommandExecutor {
    pub fn new(payments_service: Rc<PaymentsService>) -> PaymentsCommandExecutor {
        PaymentsCommandExecutor {
            payments_service
        }
    }

    pub fn execute(&self, command: PaymentsCommand) {
        match command {
            PaymentsCommand::RegisterMethod(type_, method_cbs, cb) => {
                cb(self.register_method(&type_, method_cbs));
            }
            PaymentsCommand::CreateAddress(type_, config, cb) => {
                self.create_address(&type_, &config, cb);
            }
            PaymentsCommand::CreateAddressAck(_handle, _result) => {
                unimplemented!()
            }
        }
    }

    fn register_method(&self, type_: &str, methods: PaymentsMethodCBs) -> Result<(), IndyError> {
        trace!("register_method >>> type_: {:?}, methods: {:?}", type_, methods);

        self.payments_service.register_payment_method(type_, methods);
        let res = Ok(());

        trace!("register_method << res: {:?}", res);

        res
    }

    fn create_address(&self, type_: &str, config: &str, cb: Box<Fn(Result<String, IndyError>) + Send>) {
        let cmd_handle = ::utils::sequence::SequenceUtils::get_next_id();
        match self.payments_service.create_address(cmd_handle, type_, config) {
            Ok(()) => unimplemented!(),
            Err(err) => cb(Err(IndyError::from(err))),
        }
    }
}
