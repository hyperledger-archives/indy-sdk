extern crate regex;
extern crate chrono;

use command_executor::{Command, CommandContext, CommandMetadata, CommandParams, CommandGroup, CommandGroupMetadata};
use commands::*;

use indy::{ErrorCode, IndyError};
use libindy::payment::Payment;

use serde_json::Value as JSONValue;
use serde_json::Map as JSONMap;

use utils::table::print_list_table;


pub mod group {
    use super::*;

    command_group!(CommandGroupMetadata::new("payment-address", "Payment address management commands"));
}

pub mod create_command {
    use super::*;

    command!(CommandMetadata::build("create", "Create the payment address for specified payment method.")
                .add_required_param("payment_method", "Payment method to use")
                .add_optional_param("seed", "Seed for creating payment address")
                .add_example("payment-address create payment_method=sov")
                .add_example("payment-address create payment_method=sov seed=000000000000000000000000000Seed1")
                .finalize()
    );

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let payment_method = get_str_param("payment_method", params).map_err(error_err!())?;
        let seed = get_opt_str_param("seed", params).map_err(error_err!())?;

        let config = {
            let mut json = JSONMap::new();
            update_json_map_opt_key!(json, "seed", seed);
            JSONValue::from(json).to_string()
        };

        let res = match Payment::create_payment_address(wallet_handle, payment_method, &config) {
            Ok(payment_address) => Ok(println_succ!("Payment Address has been created \"{}\"", payment_address)),
            Err(err) => Err(handle_payment_error(err, Some(payment_method))),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod list_command {
    use super::*;

    command!(CommandMetadata::build("list", "Lists all payment addresses that are stored in the wallet.")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let res = match Payment::list_payment_addresses(wallet_handle) {
            Ok(payment_addresses_json) => {
                let mut payment_addresses: Vec<String> = serde_json::from_str(&payment_addresses_json)
                    .map_err(|_| println_err!("Wrong data has been received"))?;

                let list_addresses =
                    payment_addresses.iter()
                        .map(|payment_address| {
                            let parts = payment_address.split(":").collect::<Vec<&str>>();
                            json!({
                                "address": payment_address,
                                "method": parts.get(1).unwrap_or(&"Unknown payment method")
                            })
                        })
                        .collect::<Vec<serde_json::Value>>();

                print_list_table(&list_addresses,
                                 &vec![("address", "Payment Address"),
                                       ("method", "Payment Method")],
                                 "There are no payment addresses");
                Ok(())
            }
            Err(err) => Err(handle_indy_error(err, None, None, None)),
        };

        trace!("execute << {:?}", res);
        res
    }
}

pub mod sign_command {
    use super::*;

    command!(CommandMetadata::build("sign", "Create a proof of payment address control by signing an input and producing a signature.")
                .add_required_param("address", "Payment address to use")
                .add_required_param("input", "The input data to be signed")
                .add_example("sign address=pav:sov:1a2b3c4d5e6f7g8h9ioplkjhgfdsfgbv input=910274529363984")
                .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let wallet_handle = ensure_opened_wallet_handle(&ctx)?;

        let address = get_str_param("address", params).map_err(error_err!())?;
        let input = get_str_param("input", params).map_err(error_err!())?;

        let res = match Payment::sign_with_address(wallet_handle, address, input) {
            Ok(signature) => Ok(println_succ!("Signature \"0x{}\"", bin2hex(signature.as_slice()))),
            Err(err) => Err(handle_indy_error(err, None, None, None))
        };

        trace!("execute << {:?}", res);
        res
    }

    fn bin2hex(b: &[u8]) -> String {
        b.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
}

pub mod verify_command {
    use super::*;

    command!(CommandMetadata::build("verify", "Verify a proof of payment address control by verifying a signature.")
             .add_required_param("address", "Payment address to use")
             .add_required_param("input", "The input data that was signed")
             .add_required_param("signature", "The signature generated from sign-with-address")
             .add_example("verify address=pav:sov:1a2b3c4d5e6f7g8h9ioplkjhgfdsfgbv input=910274529363984 signature=0x1c542a32bd39cf1fd343fd4f211ea2f7fb5c4bca0ab7c7d8c7dc192511593484b86cb9b919040981addf81ee4c0feae080ef592efdb7c6ea6e4ae1c007a60178")
             .finalize());

    fn execute(ctx: &CommandContext, params: &CommandParams) -> Result<(), ()> {
        trace!("execute >> ctx {:?} params {:?}", ctx, params);

        let address = get_str_param("address", params).map_err(error_err!())?;
        let input = get_str_param("input", params).map_err(error_err!())?;
        let signature = get_str_param("signature", params).map_err(error_err!())?;

        if &signature[0..2] != "0x" {
            println_err!("Wrong data has been received. Expected signature to start with '0x'");
            return Err(());
        }

        let sig = hex2bin(&signature[2..]).map_err(|e| println_err!("{}", e))?;

        let res = match Payment::verify_with_address(address, input, sig.as_slice()) {
            Ok(valid) => {
                if valid {
                    Ok(println_succ!("Valid signature"))
                } else {
                    Ok(println_err!("Invalid signature"))
                }
            }
            Err(err) => Err(handle_indy_error(err, None, None, None))
        };

        trace!("execute << {:?}", res);
        res
    }

    fn hex2bin(s: &str) -> Result<Vec<u8>, String> {
        if s.len() % 2 != 0 {
            return Err("Bad input".to_string());
        }
        for (i, ch) in s.chars().enumerate() {
            if !ch.is_digit(16) {
                return Err(format!(
                    "Bad character position {}",
                    i
                ));
            }
        }

        let input: Vec<_> = s.chars().collect();

        let decoded: Vec<u8> = input
            .chunks(2)
            .map(|chunk| {
                ((chunk[0].to_digit(16).unwrap() << 4) | (chunk[1].to_digit(16).unwrap())) as u8
            })
            .collect();

        Ok(decoded)
    }
}

pub fn handle_payment_error(err: IndyError, payment_method: Option<&str>) {
    match err.error_code {
        ErrorCode::UnknownPaymentMethod => println_err!("Unknown payment method {}", payment_method.unwrap_or("")),
        ErrorCode::IncompatiblePaymentError => println_err!("No methods were scraped or more than one was scraped"),
        ErrorCode::PaymentInsufficientFundsError => println_err!("Insufficient funds on inputs"),
        ErrorCode::PaymentExtraFundsError => println_err!("Extra funds on inputs"),
        ErrorCode::PaymentSourceDoesNotExistError => println_err!("Payment source not found"),
        ErrorCode::PaymentOperationNotSupportedError => println_err!("Payment operation not supported"),
        ErrorCode::WalletItemAlreadyExists => println_err!("Payment address already exists"),
        _ => println_err!("{}", err.message)
    }
}

#[cfg(test)]
#[cfg(feature = "nullpay_plugin")]
pub mod tests {
    use super::*;
    use commands::common::tests::{load_null_payment_plugin, NULL_PAYMENT_METHOD};
    use commands::did::tests::SEED_MY1;

    mod create {
        use super::*;

        #[test]
        pub fn create_works() {
            let ctx = setup_with_wallet();
            load_null_payment_plugin(&ctx);
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let addresses = list_payment_addresses(&ctx);
            assert_eq!(1, addresses.len());
            assert!(addresses[0].starts_with("pay:null:"));

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn create_works_for_seed() {
            let ctx = setup_with_wallet();
            load_null_payment_plugin(&ctx);
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                params.insert("seed", SEED_MY1.to_string());
                cmd.execute(&ctx, &params).unwrap();
            }
            let addresses = list_payment_addresses(&ctx);
            assert_eq!(1, addresses.len());
            //            assert_eq!("pay:null:AkQr7K6CP1tslXd", addresses[0]);  TODO: Exactly check

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn create_works_for_unknown_payment_method() {
            let ctx = setup_with_wallet();
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", "unknown_payment_method".to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn create_works_for_no_opened_wallet() {
            let ctx = setup();
            load_null_payment_plugin(&ctx);
            {
                let cmd = create_command::new();
                let mut params = CommandParams::new();
                params.insert("payment_method", NULL_PAYMENT_METHOD.to_string());
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }
    }

    mod list {
        use super::*;

        #[test]
        pub fn list_works() {
            let ctx = setup_with_wallet();
            load_null_payment_plugin(&ctx);
            create_payment_address(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            let addresses = list_payment_addresses(&ctx);
            assert_eq!(1, addresses.len());

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_empty_list() {
            let ctx = setup_with_wallet();
            load_null_payment_plugin(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap();
            }
            let addresses = list_payment_addresses(&ctx);
            assert_eq!(0, addresses.len());

            tear_down_with_wallet(&ctx);
        }

        #[test]
        pub fn list_works_for_no_opened_wallet() {
            let ctx = setup();
            load_null_payment_plugin(&ctx);
            {
                let cmd = list_command::new();
                let params = CommandParams::new();
                cmd.execute(&ctx, &params).unwrap_err();
            }
            tear_down();
        }
    }

    fn list_payment_addresses(ctx: &CommandContext) -> Vec<String> {
        let wallet_handle = ensure_opened_wallet_handle(ctx).unwrap();
        let payment_addresses = Payment::list_payment_addresses(wallet_handle).unwrap();
        serde_json::from_str(&payment_addresses).unwrap()
    }

    pub fn create_payment_address(ctx: &CommandContext) -> String {
        let wallet_handle = ensure_opened_wallet_handle(ctx).unwrap();
        Payment::create_payment_address(wallet_handle, NULL_PAYMENT_METHOD, "{}").unwrap()
    }
}
