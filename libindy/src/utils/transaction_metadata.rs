use errors::*;
use domain::ledger::response::{
    Message,
    Reply,
    ResponseMetadata
};
use serde::de::DeserializeOwned;

pub fn parse_response_metadata(response: &str) -> IndyResult<String> {
    let response_metadata = _parse_response_metadata(response)?;

    let res = serde_json::to_string(&response_metadata)
        .to_indy(IndyErrorKind::InvalidState, "Cannot serialize ResponseMetadata")?;

    Ok(res)
}

fn _parse_response_metadata(response: &str) -> IndyResult<ResponseMetadata> {
    let message: Message<serde_json::Value> = serde_json::from_str(response)
        .to_indy(IndyErrorKind::InvalidTransaction, "Cannot deserialize transaction Response")?;

    let response_object: Reply<serde_json::Value> = handle_response_message_type(message)?;
    let response_result = response_object.result();

    let response_metadata = match response_result["ver"].as_str() {
        None => parse_transaction_metadata_v0(&response_result),
        Some("1") => parse_transaction_metadata_v1(&response_result),
        ver @ _ => return Err(err_msg(IndyErrorKind::InvalidTransaction, format!("Unsupported transaction response version: {:?}", ver)))
    };

    Ok(response_metadata)
}

pub fn get_last_signed_time(response: &str) -> u64 {
    let c = _parse_response_metadata(response);
    println!("{:?}", c);
    c.map(|resp| resp.last_txn_time.unwrap_or(0)).unwrap_or(0)
}

pub fn handle_response_message_type<T>(message: Message<T>) -> IndyResult<Reply<T>> where T: DeserializeOwned + ::std::fmt::Debug {
    trace!("handle_response_message_type >>> message {:?}", message);

    match message {
        Message::Reject(response) | Message::ReqNACK(response) =>
            Err(err_msg(IndyErrorKind::InvalidTransaction, format!("Transaction has been failed: {:?}", response.reason))),
        Message::Reply(reply) =>
            Ok(reply)
    }
}

fn parse_transaction_metadata_v0(message: &serde_json::Value) -> ResponseMetadata {
    ResponseMetadata {
        seq_no: message["seqNo"].as_u64(),
        txn_time: message["txnTime"].as_u64(),
        last_txn_time: message["state_proof"]["multi_signature"]["value"]["timestamp"].as_u64(),
        last_seq_no: None,
    }
}

fn parse_transaction_metadata_v1(message: &serde_json::Value) -> ResponseMetadata {
    ResponseMetadata {
        seq_no: message["txnMetadata"]["seqNo"].as_u64(),
        txn_time: message["txnMetadata"]["txnTime"].as_u64(),
        last_txn_time: message["multiSignature"]["signedState"]["stateMetadata"]["timestamp"].as_u64(),
        last_seq_no: None,
    }
}