extern crate serde;
extern crate serde_json;

use serde_json::Value;
use serde_json::Map;
use std::string::String;
use error::prelude::*;

pub trait KeyMatch {
    fn matches(&self, key: &String, context: &Vec<String>) -> bool;
}

impl KeyMatch for String {
    fn matches(&self, key: &String, context: &Vec<String>) -> bool {
        key.eq(self)
    }
}

/*
Rewrites keys in a serde value structor to new mapped values. Returns the remapped value. Leaves
unmapped keys as they are.
*/
pub fn mapped_key_rewrite<T: KeyMatch>(val: Value, remap: &Vec<(T, String)>) -> VcxResult<Value> {
    let mut context: Vec<String> = Default::default();
    _mapped_key_rewrite(val, &mut context, remap)
}


fn _mapped_key_rewrite<T: KeyMatch>(val: Value, context: &mut Vec<String>, remap: &Vec<(T, String)>) -> VcxResult<Value> {
    if let Value::Object(mut map) = val {
        let mut keys:Vec<String> = _collect_keys(&map);

        while let Some(k) = keys.pop() {
            let mut value = map.remove(&k).ok_or_else(||{
                warn!("Unexpected key value mutation");
                VcxError::from_msg(VcxErrorKind::InvalidJson, "Unexpected key value mutation")
            })?;


            let mut new_k = k;
            for matcher in remap {
                if matcher.0.matches(&new_k, context) {
                    new_k = matcher.1.clone();
                    break;
                }
            }

            context.push(new_k.clone()); // TODO not efficient, should work with references
            value = _mapped_key_rewrite(value, context, remap)?;
            context.pop();

            map.insert(new_k, value);
        }
        Ok(Value::Object(map))
    }
    else {
        Ok(val)
    }
}

fn _collect_keys(map:&Map<String, Value>) -> Vec<String>{
    let mut rtn:Vec<String> = Default::default();
    for key in map.keys() {
        rtn.push(key.clone());
    }
    rtn
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn simple() {
        let simple_map = vec!(("d".to_string(), "devin".to_string()));
        let simple = json!({"d":"d"});
        let expected = json!({"devin":"d"});
        let transformed = mapped_key_rewrite(simple, &simple_map).unwrap();
        assert_eq!(expected, transformed);

        let simple = json!(null);
        let transformed = mapped_key_rewrite(simple.clone(), &simple_map).unwrap();
        assert_eq!(simple, transformed);

        let simple = json!("null");
        let transformed = mapped_key_rewrite(simple.clone(), &simple_map).unwrap();
        assert_eq!(simple, transformed);
    }

    #[test]
    fn abbr_test() {
        let un_abbr = json!({
  "statusCode":"MS-102",
  "connReqId":"yta2odh",
  "senderDetail":{
    "name":"ent-name",
    "agentKeyDlgProof":{
      "agentDID":"N2Uyi6SVsHZq1VWXuA3EMg",
      "agentDelegatedKey":"CTfF2sZ5q4oPcBvTP75pgx3WGzYiLSTwHGg9zUsJJegi",
      "signature":"/FxHMzX8JaH461k1SI5PfyxF5KwBAe6VlaYBNLI2aSZU3APsiWBfvSC+mxBYJ/zAhX9IUeTEX67fj+FCXZZ2Cg=="
    },
    "DID":"F2axeahCaZfbUYUcKefc3j",
    "logoUrl":"ent-logo-url",
    "verKey":"74xeXSEac5QTWzQmh84JqzjuXc8yvXLzWKeiqyUnYokx"
  },
  "senderAgencyDetail":{
    "DID":"BDSmVkzxRYGE4HKyMKxd1H",
    "verKey":"6yUatReYWNSUfEtC2ABgRXmmLaxCyQqsjLwv2BomxsxD",
    "endpoint":"52.38.32.107:80/agency/msg"
  },
  "targetName":"there",
  "statusMsg":"message sent"
});

        let abbr = json!({
  "sc":"MS-102",
  "id": "yta2odh",
  "s": {
    "n": "ent-name",
    "dp": {
      "d": "N2Uyi6SVsHZq1VWXuA3EMg",
      "k": "CTfF2sZ5q4oPcBvTP75pgx3WGzYiLSTwHGg9zUsJJegi",
      "s":
        "/FxHMzX8JaH461k1SI5PfyxF5KwBAe6VlaYBNLI2aSZU3APsiWBfvSC+mxBYJ/zAhX9IUeTEX67fj+FCXZZ2Cg==",
    },
    "d": "F2axeahCaZfbUYUcKefc3j",
    "l": "ent-logo-url",
    "v": "74xeXSEac5QTWzQmh84JqzjuXc8yvXLzWKeiqyUnYokx",
  },
  "sa": {
    "d": "BDSmVkzxRYGE4HKyMKxd1H",
    "v": "6yUatReYWNSUfEtC2ABgRXmmLaxCyQqsjLwv2BomxsxD",
    "e": "52.38.32.107:80/agency/msg",
  },
  "t": "there",
  "sm":"message sent"
});
        let map = vec![
            ("statusCode".to_string(), "sc".to_string()),
            ("connReqId".to_string(), "id".to_string()),
            ("senderDetail".to_string(), "s".to_string()),
            ("name".to_string(), "n".to_string()),
            ("agentKeyDlgProof".to_string(), "dp".to_string()),
            ("agentDID".to_string(), "d".to_string()),
            ("agentDelegatedKey".to_string(), "k".to_string()),
            ("signature".to_string(), "s".to_string()),
            ("DID".to_string(), "d".to_string()),
            ("logoUrl".to_string(), "l".to_string()),
            ("verKey".to_string(), "v".to_string()),
            ("senderAgencyDetail".to_string(), "sa".to_string()),
            ("endpoint".to_string(), "e".to_string()),
            ("targetName".to_string(), "t".to_string()),
            ("statusMsg".to_string(), "sm".to_string()),
        ];
        let transformed = mapped_key_rewrite(un_abbr, &map).unwrap();
        assert_eq!(abbr, transformed);
    }
}