use libindy::IndyHandle;

use std::cell::RefCell;

#[derive(Debug)]
pub struct IndyContext {
    opened_wallet: RefCell<Option<(String, IndyHandle)>>,
}

impl IndyContext {
    pub fn new() -> IndyContext {
        IndyContext {
            opened_wallet: RefCell::new(None),
        }
    }

    pub fn set_opened_wallet(&self, wallet_name: &str, wallet_handle: IndyHandle) {
        *self.opened_wallet.borrow_mut() = Some((wallet_name.to_string(), wallet_handle));
    }

    pub fn unset_opened_wallet(&self) {
        *self.opened_wallet.borrow_mut() = None;
    }

    pub fn get_opened_wallet_name(&self) -> Option<String> {
        self.opened_wallet.borrow().as_ref().map(|&(ref name, _)| name.to_owned())
    }

    pub fn get_current_wallet_handle(&self) -> Option<IndyHandle> {
        self.opened_wallet.borrow().as_ref().map(|&(_, handle)| handle)
    }
}
