use errors::route::RouteError;
use services::wallet::WalletService;
use services::crypto::CryptoService;
use services::route::RouteService;

use std::result;
use std::rc::Rc;
use serde_json;

type Result<T> = result::Result<T, RouteError>;


pub enum RouteCommand {
    PackMessage(
        String, // plaintext message
        String, //list of receiving keys
        String, //my verkey, provide only if authcrypt else provide None
        bool, // authcrypt -> true, anoncrypt -> false
        i32, //wallet handle
        Box<Fn(Result<String/*JWM serialized as string*/>) + Send>),
    UnpackMessage(
        String, // AMES either JSON or Compact Serialization
        String, // my verkey
        i32, // wallet handle
        Box<Fn(Result<(String /*plaintext*/)>) + Send>),
}

pub struct RouteCommandExecutor {
    wallet_service: Rc<WalletService>,
    crypto_service: Rc<CryptoService>,
    route_service: Rc<RouteService>
}

impl RouteCommandExecutor {
    pub fn new(wallet_service: Rc<WalletService>,
               crypto_service: Rc<CryptoService>,
               route_service: Rc<RouteService>,
    ) -> RouteCommandExecutor {
        RouteCommandExecutor {
            wallet_service,
            crypto_service,
            route_service,
        }
    }

    pub fn execute(&self, command: RouteCommand) {
        match command {
            RouteCommand::PackMessage(plaintext, recv_keys, my_vk, auth, wallet_handle, cb) => {
                info!("PackMessage command received");
                cb(self.pack_msg(&plaintext, &recv_keys, &my_vk, auth, wallet_handle));
            }
            RouteCommand::UnpackMessage(jwm, my_vk, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(&jwm, &my_vk, wallet_handle));
            }
        };
    }

    pub fn pack_msg(&self, message: &str, recv_keys_json: &str, my_vk: &str, auth: bool, wallet_handle: i32) -> Result<String> {

        //convert type from json array to Vec<String>
        let recv_keys : Vec<String> = serde_json::from_str(recv_keys_json)
            .map_err(|err| RouteError::SerializationError(format!("Failed to serialize recv_keys {:?}", err)))?;

        //convert type from string to Option
        let my_vk_opt= match my_vk {
            _ => Some(my_vk),
            "" => None
        };

        self.route_service.pack_msg(message, &recv_keys, my_vk_opt, auth, wallet_handle, self.wallet_service.clone(), self.crypto_service.clone())
    }

    pub fn unpack_msg(&self, ames_as_string: &str, my_vk: &str, wallet_handle: i32) -> Result<String> {
        self.route_service.unpack_msg(ames_as_string, my_vk, wallet_handle, self.wallet_service.clone(), self.crypto_service.clone())
    }

    pub fn add_route(&self, did_with_key_frag : &str, endpoint : &str, wallet_handle:i32) -> Result<()> {
        self.route_service.add_route(did_with_key_frag, endpoint, wallet_handle, self.wallet_service.clone())
    }

    pub fn lookup_route(&self, did_with_key_frag : &str, wallet_handle : i32) -> Result<String> {
        self.route_service.lookup_route(did_with_key_frag, wallet_handle, self.wallet_service.clone())
    }

    pub fn remove_route(&self, did_with_key_frag : &str, wallet_handle : i32) -> Result<()> {
        self.route_service.remove_route(did_with_key_frag, wallet_handle, self.wallet_service.clone())
    }

    pub fn update_route(&self, did_with_key_frag : &str, new_endpoint : &str, wallet_handle : i32) -> Result<()> {
        self.route_service.update_route(did_with_key_frag, new_endpoint, wallet_handle, self.wallet_service.clone())
    }
}