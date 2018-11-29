use settings;
use std::io::Read;
use std::sync::Mutex;
use reqwest;
use reqwest::header::{ContentType};
use std::env;
lazy_static!{
    static ref NEXT_U8_RESPONSE: Mutex<Vec<Vec<u8>>> = Mutex::new(vec![]);
}

//Todo: change this RC to a u32
pub fn post_u8(body_content: &Vec<u8>) -> Result<Vec<u8>,String> {
    let url = format!("{}/agency/msg", settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT).or(Err("Invalid Configuration".to_string()))?);

    if settings::test_agency_mode_enabled() {return Ok(NEXT_U8_RESPONSE.lock().unwrap().pop().unwrap_or(Vec::new()));}
    //Setting SSL Certs location. This is needed on android platform. Or openssl will fail to verify the certs
    if cfg!(target_os = "android") {
        info!("::Android code");
        set_ssl_cert_location();
    }
    let client = reqwest::ClientBuilder::new().build().or(Err("Preparing Post failed".to_string()))?;
    debug!("Posting encrypted bundle to: \"{}\"", url);
    let mut response = match  client.post(&url).body(body_content.to_owned()).header(ContentType::octet_stream()).send() {
        Ok(result) => {
            trace!("got the result");
            result
        },
        Err(err) => {
            error!("error: {}", err);
            return Err("could not connect".to_string())
        },
    };

    trace!("Response Header: {:?}", response);
    if !response.status().is_success() {
        let mut content = String::new();
        match response.read_to_string(&mut content) {
            Ok(x) => info!("Request failed: {}", content),
            Err(x) => info!("could not read response"),
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

fn set_ssl_cert_location(){

    let ssl_cert_file= "SSL_CERT_FILE";
    env::set_var(ssl_cert_file,env::var("EXTERNAL_STORAGE").unwrap() + "/cacert.pem"); //TODO: CHANGE ME, HARDCODING FOR TESTING ONLY
    match env::var(ssl_cert_file) {
        Ok(val) => info!("{}:: {:?}", ssl_cert_file, val),
        Err(e) => error!("couldn't find var in env {}:: {}. This needs to be set on Android to make https calls.\n See https://github.com/seanmonstar/reqwest/issues/70 for more info",
                         ssl_cert_file, e),
    }
    info!("::SSL_CERT_FILE has been set");
}
