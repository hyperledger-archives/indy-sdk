use services::signus::SignusService;

pub mod types;
pub mod constants;

use std::rc::Rc;
use std::collections::HashMap;


pub struct SSSService {
    crypto_service: Rc<SignusService>
}

// Should it be singleton
impl SSSService {
    pub fn new(crypto_service: Rc<SignusService>) -> SSSService { SSSService { crypto_service } }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_new_sss_service() -> SSSService {
        let crypto_service: Rc<SignusService> = Rc::new(SignusService::new());
        SSSService::new(crypto_service.clone())
    }
}