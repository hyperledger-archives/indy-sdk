use settings;
use std::io::Read;
use std::sync::Mutex;
use reqwest;
use reqwest::header::{ContentType};

lazy_static!{
    static ref NEXT_U8_RESPONSE: Mutex<Vec<Vec<u8>>> = Mutex::new(vec![]);
}

lazy_static!{
    static ref NEXT_STR_RESPONSE: Mutex<Vec<String>> = Mutex::new(vec![]);
}

pub fn post(body_content: &str, url: &str) -> Result<String,String> {
    let client = reqwest::Client::new();
    debug!("Posting \"{}\" to: \"{}\"", body_content, url);
    if settings::test_agency_mode_enabled() {return Ok(NEXT_STR_RESPONSE.lock().unwrap().pop().unwrap_or("test_mode_response".to_owned()));}
    let mut response = match  client.post(url).body(body_content.to_owned()).send() {
        Ok(result) => result,
        Err(err) => return Err("could not connect".to_string()),
    };

    info!("Response Header: {:?}", response);
    if !response.status().is_success() {return Err("POST failed".to_string());}

    let mut content = String::new();
    match response.read_to_string(&mut content) {
        Ok(_) => Ok(content.to_owned()),
        Err(_) => Err("could not read response".to_string()),
    }
}

/* TODO: remove and use generics */
pub fn post_u8(body_content: &Vec<u8>, url: &str) -> Result<Vec<u8>,String> {
    let client = reqwest::Client::new();
    info!("Posting encrypted bundle to: \"{}\"", url);
    if settings::test_agency_mode_enabled() {return Ok(NEXT_U8_RESPONSE.lock().unwrap().pop().unwrap_or(Vec::new()));}
    let mut response = match  client.post(url).body(body_content.to_owned()).header(ContentType::octet_stream()).send() {
        Ok(result) => result,
        Err(err) => {
            error!("error: {}", err);
            return Err("could not connect".to_string())
        },
    };

    info!("Response Header: {:?}", response);
    if !response.status().is_success() {
        let mut content = String::new();
        match response.read_to_string(&mut content) {
            Ok(x) => error!("Request failed: {}", content),
            Err(x) => error!("could not read response"),
        };
        return Err("POST failed".to_string());
    }

    let mut content = Vec::new();
    match response.read_to_end(&mut content) {
        Ok(x) => Ok(content.to_owned()),
        Err(_) => Err("could not read response".to_string()),
    }
}

pub fn set_next_u8_response(body: Vec<u8>) {
    NEXT_U8_RESPONSE.lock().unwrap().push(body);
}

pub fn set_next_str_response(body: String) {
    NEXT_STR_RESPONSE.lock().unwrap().push(body);
}
