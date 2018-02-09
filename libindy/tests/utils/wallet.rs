use indy::api::ErrorCode;
use indy::api::wallet::{
    indy_register_wallet_type,
    indy_create_wallet,
    indy_open_wallet,
    indy_delete_wallet,
    indy_close_wallet
};

use utils::callback::CallbackUtils;
use utils::inmem_wallet::InmemWallet;
use utils::timeout::TimeoutUtils;
use utils::sequence::SequenceUtils;

use std::collections::HashSet;
use std::ffi::CString;
use std::ptr::null;
use std::sync::mpsc::channel;
use std::sync::Mutex;

pub struct WalletUtils {}


impl WalletUtils {
    pub fn register_wallet_type(xtype: &str, force_create: bool) -> Result<(), ErrorCode> {
        lazy_static! {
            static ref REGISERED_WALLETS: Mutex<HashSet<String>> = Default::default();
        }

        let mut wallets = REGISERED_WALLETS.lock().unwrap();

        if wallets.contains(xtype) & !force_create {
            // as registering of plugged wallet with
            return Ok(())
        }

        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_register_wallet_type_cb(cb);

        let xxtype = CString::new(xtype).unwrap();

        let err = indy_register_wallet_type(
            command_handle,
            xxtype.as_ptr(),
            Some(InmemWallet::create),
            Some(InmemWallet::open),
            Some(InmemWallet::set),
            Some(InmemWallet::get),
            Some(InmemWallet::get_not_expired),
            Some(InmemWallet::list),
            Some(InmemWallet::close),
            Some(InmemWallet::delete),
            Some(InmemWallet::free),
            cb
        );

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        wallets.insert(xtype.to_string());
        Ok(())
    }

    pub fn create_wallet(pool_name: &str, wallet_name: &str, xtype: Option<&str>, config: Option<&str>, credentials: Option<&str>) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_create_wallet_cb(cb);

        let pool_name = CString::new(pool_name).unwrap();
        let wallet_name = CString::new(wallet_name).unwrap();
        let xtype_str = xtype.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let credentials_str = credentials.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               if xtype.is_some() { xtype_str.as_ptr() } else { null() },
                               if config.is_some() { config_str.as_ptr() } else { null() },
                               if credentials.is_some() { credentials_str.as_ptr() } else { null() },
                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn open_wallet(wallet_name: &str, config: Option<&str>, credentials: Option<&str>) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err, handle| {
            sender.send((err, handle)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_open_wallet_cb(cb);

        let wallet_name = CString::new(wallet_name).unwrap();
        let config_str = config.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());
        let credentials_str = credentials.map(|s| CString::new(s).unwrap()).unwrap_or(CString::new("").unwrap());

        let err =
            indy_open_wallet(command_handle,
                             wallet_name.as_ptr(),
                             if config.is_some() { config_str.as_ptr() } else { null() },
                             if credentials.is_some() { credentials_str.as_ptr() } else { null() },
                             cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, wallet_handle) = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(wallet_handle)
    }

    pub fn create_and_open_wallet(pool_name: &str, xtype: Option<&str>) -> Result<i32, ErrorCode> {
        let (sender, receiver) = channel();
        let (open_sender, open_receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });
        let open_cb = Box::new(move |err, handle| {
            open_sender.send((err, handle)).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_create_wallet_cb(cb);
        let (open_command_handle, open_cb) = CallbackUtils::closure_to_open_wallet_cb(open_cb);

        let pool_name = CString::new(pool_name).unwrap();
        let wallet_name = CString::new(format!("default-wallet-name-{}", SequenceUtils::get_next_id())).unwrap();
        let xtype = match xtype {
            Some(xtype) => CString::new(xtype).unwrap(),
            None => CString::new("default").unwrap()
        };

        let err =
            indy_create_wallet(command_handle,
                               pool_name.as_ptr(),
                               wallet_name.as_ptr(),
                               xtype.as_ptr(),
                               null(),
                               null(),
                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::medium_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err =
            indy_open_wallet(open_command_handle,
                             wallet_name.as_ptr(),
                             null(),
                             null(),
                             open_cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let (err, wallet_handle) = open_receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(wallet_handle)
    }

    pub fn delete_wallet(wallet_name: &str) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_delete_wallet_cb(cb);

        let wallet_name = CString::new(wallet_name).unwrap();

        let err =
            indy_delete_wallet(command_handle,
                               wallet_name.as_ptr(),
                               null(),
                               cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }

    pub fn close_wallet(wallet_handle: i32) -> Result<(), ErrorCode> {
        let (sender, receiver) = channel();

        let cb = Box::new(move |err| {
            sender.send(err).unwrap();
        });

        let (command_handle, cb) = CallbackUtils::closure_to_delete_wallet_cb(cb);


        let err =
            indy_close_wallet(command_handle,
                              wallet_handle,
                              cb);

        if err != ErrorCode::Success {
            return Err(err);
        }

        let err = receiver.recv_timeout(TimeoutUtils::short_timeout()).unwrap();

        if err != ErrorCode::Success {
            return Err(err);
        }

        Ok(())
    }
}