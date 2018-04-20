extern crate libc;

use errors::indy::IndyError;
use errors::payments::PaymentsError;
use services::payments::{PaymentsMethodCBs, PaymentsService};

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub enum PaymentsCommand {
    RegisterMethod(
        String, //type
        PaymentsMethodCBs, //method callbacks
        Box<Fn(Result<(), IndyError>) + Send>),
    CreateAddress(
        String, //type
        String, //config
        Box<Fn(Result<String, IndyError>) + Send>),
    CreateAddressAck(
        i32, //handle
        Result<String /* address */, PaymentsError>),
}

pub struct PaymentsCommandExecutor {
    payments_service: Rc<PaymentsService>,
    pending_callbacks: RefCell<HashMap<i32, Box<Fn(Result<String, IndyError>) + Send>>>,
}

impl PaymentsCommandExecutor {
    pub fn new(payments_service: Rc<PaymentsService>) -> PaymentsCommandExecutor {
        PaymentsCommandExecutor {
            payments_service,
            pending_callbacks: RefCell::new(HashMap::new()),
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
            PaymentsCommand::CreateAddressAck(handle, result) => {
                self.create_address_ack(handle, result);
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
            Ok(()) => {
                self.pending_callbacks.borrow_mut().insert(cmd_handle, cb);
            }
            Err(err) => cb(Err(IndyError::from(err))),
        }
    }

    fn create_address_ack(&self, handle: i32, result: Result<String, PaymentsError>) {
        //TODO store address in Wallet
        match self.pending_callbacks.borrow_mut().remove(&handle) {
            Some(cb) => cb(result.map_err(IndyError::from)),
            None => error!("Can't process PaymentsCommand::CreateAddressAck for handle {} with result {:?} - appropriate callback not found!",
                           handle, result)
        }
    }
}
