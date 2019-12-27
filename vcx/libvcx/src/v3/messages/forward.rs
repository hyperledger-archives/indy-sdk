use error::prelude::*;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Forward {
    pub to: String,
    #[serde(rename = "msg")]
    pub msg: ::serde_json::Value,
}

impl Forward {
    pub fn new(to: String, msg: Vec<u8>) -> VcxResult<Forward> {
        let msg = ::serde_json::from_slice(msg.as_slice())
            .map_err(|err| VcxError::from_msg(VcxErrorKind::InvalidState, err))?;

        Ok(Forward {
            to,
            msg,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use v3::messages::ack::tests::*;

    fn _to() -> String {
        String::from("GJ1SzoWzavQYfNL9XkaJdrQejfztN4XqdsiV4ct3LXKL")
    }

    fn _msg() -> ::serde_json::Value {
       json!(_ack())
    }

    fn _forward() -> Forward {
        Forward {
            to: _to(),
            msg: _msg(),
        }
    }

    #[test]
    fn test_forward_build_works() {
        let message = ::serde_json::to_vec(&_ack()).unwrap();
        let forward: Forward = Forward::new(_to(), message).unwrap();
        assert_eq!(_forward(), forward);
    }
}