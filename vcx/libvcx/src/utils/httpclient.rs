use settings;
use std::io::Read;
use std::sync::Mutex;
use reqwest;
use reqwest::header::CONTENT_TYPE;
use std::env;
use error::prelude::*;

lazy_static! {
    static ref NEXT_U8_RESPONSE: Mutex<Vec<Vec<u8>>> = Mutex::new(vec![]);
}

//Todo: change this RC to a u32
pub fn post_u8(body_content: &Vec<u8>) -> VcxResult<Vec<u8>> {
    let endpoint = settings::get_config_value(settings::CONFIG_AGENCY_ENDPOINT)?;
    let url = format!("{}/agency/msg", endpoint);

    if settings::test_agency_mode_enabled() {
        return Ok(NEXT_U8_RESPONSE.lock().unwrap().pop().unwrap_or(Vec::new()));
    }

    //Setting SSL Certs location. This is needed on android platform. Or openssl will fail to verify the certs
    if cfg!(target_os = "android") {
        info!("::Android code");
        set_ssl_cert_location();
    }
    let client = reqwest::ClientBuilder::new().timeout(::utils::timeout::TimeoutUtils::long_timeout()).build()
        .or(Err(VcxError::from_msg(VcxErrorKind::PostMessageFailed, "Preparing Post failed")))?;
    debug!("Posting encrypted bundle to: \"{}\"", url);

    let mut response =
        client.post(&url)
            .body(body_content.to_owned())
            .header(CONTENT_TYPE, "octet_stream")
            .send()
            .map_err(|err| {
                error!("error: {}", err);
                VcxError::from_msg(VcxErrorKind::PostMessageFailed, "Could not connect")
            })?;

    trace!("Response Header: {:?}", response);
    if !response.status().is_success() {
        let mut content = String::new();
        match response.read_to_string(&mut content) {
            Ok(x) => info!("Request failed: {}", content),
            Err(x) => info!("could not read response"),
        };
        return Err(VcxError::from_msg(VcxErrorKind::PostMessageFailed, format!("POST failed with: {}", content)));
    }

    let mut content = Vec::new();
    response.read_to_end(&mut content)
        .or(Err(VcxError::from_msg(VcxErrorKind::PostMessageFailed, "could not read response")))?;

    Ok(content)
}

pub fn set_next_u8_response(body: Vec<u8>) {
    NEXT_U8_RESPONSE.lock().unwrap().push(body);
}

fn set_ssl_cert_location() {
    let ssl_cert_file = "SSL_CERT_FILE";
    env::set_var(ssl_cert_file, env::var("EXTERNAL_STORAGE").unwrap() + "/cacert.pem"); //TODO: CHANGE ME, HARDCODING FOR TESTING ONLY
    match env::var(ssl_cert_file) {
        Ok(val) => info!("{}:: {:?}", ssl_cert_file, val),
        Err(e) => error!("couldn't find var in env {}:: {}. This needs to be set on Android to make https calls.\n See https://github.com/seanmonstar/reqwest/issues/70 for more info",
                         ssl_cert_file, e),
    }
    info!("::SSL_CERT_FILE has been set");
}
