use errors::route::RouteError;
use services::wallet::WalletService;
use services::crypto::CryptoService;
use services::route::RouteService;
use utils::option::OptionDeref;
use std::result;
use std::rc::Rc;
use serde_json;


type Result<T> = result::Result<T, RouteError>;


pub enum RouteCommand {
    PackMessage(
        String, // plaintext message
        String, //list of receiving keys
        Option<String>, //my verkey, provide only if authcrypt else provide None
        bool, // authcrypt -> true, anoncrypt -> false
        i32, //wallet handle
        Box<Fn(Result<String/*JWM serialized as string*/>) + Send>),
    UnpackMessage(
        String, // AMES either JSON or Compact Serialization
        String, // my verkey
        i32, // wallet handle
        Box<Fn(Result<(String /*plaintext*/)>) + Send>),
    AddRoute(
        String, //DID#key
        String, //endpoint
        i32, //wallet_handle
        Box<Fn(Result<()>) + Send>),
    LookupRoute(
        String, //DID#key
        i32, //wallet_handle
        Box<Fn(Result<(String /*endpoint*/)>) + Send>),
    RemoveRoute(
        String, //DID#key
        i32, //wallet_handle
        Box<Fn(Result<()>) + Send>),
    UpdateRoute(
        String, //DID#key
        String, //new endpoint
        i32, //wallet_handle
        Box<Fn(Result<()>) + Send>),
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
                cb(self.pack_msg(&plaintext, &recv_keys, my_vk, auth, wallet_handle));
            }
            RouteCommand::UnpackMessage(ames, my_vk, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(&ames, &my_vk, wallet_handle));
            }
        };
    }

    pub fn pack_msg(&self, message: &str, recv_keys_json: &str, my_vk: Option<String>, auth: bool, wallet_handle: i32) -> Result<String> {

        //convert type from json array to Vec<String>
        let recv_keys : Vec<String> = serde_json::from_str(recv_keys_json)
            .map_err(|err| RouteError::SerializationError(format!("Failed to serialize recv_keys {:?}", err)))?;

        self.route_service.pack_msg(message, &recv_keys,
                                    my_vk.as_deref(),
                                    auth, wallet_handle,
                                    self.wallet_service.clone(),
                                    self.crypto_service.clone())
    }

    pub fn unpack_msg(&self, ames_as_string: &str, my_vk: &str, wallet_handle: i32) -> Result<String> {
        self.route_service.unpack_msg(ames_as_string,
                                      my_vk,
                                      wallet_handle,
                                      self.wallet_service.clone(),
                                      self.crypto_service.clone())
    }
}