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
    AuthPackMessage(
        String, // plaintext message
        String, //list of receiving keys
        String, //my verkey
        i32, //wallet_handle
        Box<Fn(Result<String/*JWM serialized as string*/>) + Send>),
    AnonPackMessage(
        String, // plaintext message
        String, // list of receiving keys
        i32, // wallet_handle
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
            RouteCommand::AuthPackMessage(message, recv_keys_json, my_vk, wallet_handle, cb) => {
                info!("PackMessage command received");
                cb(self.auth_pack_msg(&message, &recv_keys_json, my_vk, wallet_handle));
            }
            RouteCommand::AnonPackMessage(plaintext, recv_keys_json, wallet_handle, cb) => {
                info!("PackMessage command received");
                cb(self.anon_pack_msg(&message, &recv_keys_json, wallet_handle));
            }
            RouteCommand::UnpackMessage(ames, my_vk, wallet_handle, cb) => {
                info!("UnpackMessage command received");
                cb(self.unpack_msg(&ames, &my_vk, wallet_handle));
            }
        };
    }

    pub fn auth_pack_msg(&self, message: &str, recv_keys_json: &str, my_vk: String, wallet_handle: i32) -> Result<String> {

        //convert type from json array to Vec<String>
        let recv_keys : Vec<&str> = serde_json::from_str(recv_keys_json)
            .map_err(|err| RouteError::SerializationError(format!("Failed to serialize recv_keys {:?}", err)))?;

        self.route_service.pack_msg(message, recv_keys,
                                    my_vk.as_deref(),
                                    auth, wallet_handle,
                                    self.wallet_service.clone(),
                                    self.crypto_service.clone())
    }

    pub fn anon_pack_msg(&self, message: &str, recv_keys_json: &str, wallet_handle: i32) -> Result<String> {

        //convert type from json array to Vec<&str>
        let recv_keys : Vec<&str> = serde_json::from_str(recv_keys_json)
            .map_err(|err| RouteError::SerializationError(format!("Failed to serialize recv_keys {:?}", err)))?;

        self.route_service.anon_pack_msg(message,
                                         recv_keys,
                                         wallet_handle,
                                         self.wallet_service.clone(),
                                         self.crypto_service.clone())
    }

    pub fn unpack_msg(&self, ames: &str, my_vk: &str, wallet_handle: i32) -> Result<(String, Option<String>)> {
        self.route_service.unpack_msg(ames_as_string,
                                      my_vk,
                                      wallet_handle,
                                      self.wallet_service.clone(),
                                      self.crypto_service.clone())
    }
}