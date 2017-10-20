extern crate reqwest;

mod mock;

use std::io::Read;


#[test]
fn mock_agent_srv() {
    let server = mock::agent_srv::MockAgentSrv::new(Some(3000), None);
    server.start().unwrap();

    let client = reqwest::Client::new();
    let mut resp = client.get("http://127.0.0.1:3000")
        .send()
        .unwrap();

    let mut content = String::new();
    resp.read_to_string(&mut content).unwrap();

    assert_eq!(content, "OK");
    server.stop().unwrap();
}

#[test]
fn mock_create_key() {
    let server = mock::agent_srv::MockAgentSrv::new(None, None);
    server.start().unwrap();

    let body = r#"{
      "to": "JmvnKLYj7b7e5ywLxkRMjM",
      "agentPayload": "{\"type\":\"CREATE_KEY\",\"forDID\":\"29gMVowi6hkzWsHSy8hcch\",\
      \"forDIDVerKey\":\"dLqfZF8FL5LmyRADdcKUcVvNfsdt6UA6RKbzNkZnrSX\",\"nonce\":\"sdf\"}"
    }"#;

    let url =  format!("http://127.0.0.1:{}", server.port);
    let client = reqwest::Client::new();

    let mut response = client.post(&url)
        .body(body)
        .send()
        .unwrap();

    server.stop().unwrap();

    let mut content = String::new();
    response.read_to_string(&mut content).unwrap();
    assert_eq!(content, "OK");
}