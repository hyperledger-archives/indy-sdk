use errors::route::RouteError;
use services::wallet::WalletService;
use services::crypto::CryptoService;
use services::route::RouteService;

use std::result;
use std::rc::Rc;

type Result<T> = result::Result<T, RouteError>;


pub enum RouteCommand {
    PackMessage(
        String, // plaintext message
        bool, // authcrypt -> true, anoncrypt -> false
        Vec<String>, //list of receiving keys
        i32, //wallet handle
        Option<String>, //my verkey, provide only if authcrypt else provide None
        Box<Fn(Result<String/*JWM serialized as string*/>) + Send>),
    UnpackMessage(
        String, // JWM either JSON or Compact Serialization
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
            RouteCommand::PackMessage(plaintext, auth, recv_keys, wallet_handle, my_vk, cb) => {
                info!("PackMessage command received");
                cb(self.pack_msg(&plaintext, auth, &recv_keys, wallet_handle, my_vk.as_ref()));
            }
            RouteCommand::UnpackMessage(jwm, my_vk, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(&jwm, &my_vk, wallet_handle));
            }
        };
    }

    pub fn pack_msg(&self, plaintext: &str, auth: bool, recv_keys: &Vec<String>, wallet_handle: i32, my_verkey: Option<&String>) -> Result<String> {
        let my_vk : Option<&str> = match my_verkey {
            Some(vk) => Some(vk.as_ref()),
            None => None
        };

        self.route_service.pack_msg(plaintext, auth, recv_keys, my_vk, wallet_handle, self.wallet_service, self.crypto_service)
    }

    pub fn unpack_msg(&self, json_jwm: &str, my_vk: &str, wallet_handle: i32) -> Result<String> {
        self.route_service.unpack_msg(json_jwm, my_vk, wallet_handle, self.wallet_service, self.crypto_service)
    }

    pub fn add_route(&self, did_with_key_frag : &str, endpoint : &str, wallet_handle:i32) -> Result<()> {
        self.route_service.add_route(did_with_key_frag, endpoint, wallet_handle, self.wallet_service)
    }

    pub fn lookup_route(&self, did_with_key_frag : &str, wallet_handle : i32) -> Result<String> {
        self.route_service.lookup_route(did_with_key_frag, wallet_handle, self.wallet_service)
    }

    pub fn remove_route(&self, did_with_key_frag : &str, wallet_handle : i32) -> Result<()> {
        self.route_service.remove_route(did_with_key_frag, wallet_handle, self.wallet_service)
    }

    pub fn update_route(&self, did_with_key_frag : &str, new_endpoint : &str, wallet_handle : i32) -> Result<()> {
        self.route_service.update_route(did_with_key_frag, new_endpoint, wallet_handle, self.wallet_service)
    }
}