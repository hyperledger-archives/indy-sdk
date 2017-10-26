extern crate mockito;

use settings;
use std::io::Read;
use reqwest;

pub fn post(body_content: &str, url: &str) -> Result<String,String> {
    let client = reqwest::Client::new();
    info!("Posting \"{}\" to: \"{}\"", body_content, url);
    if settings::test_mode_enabled() {return Ok("test_mode_response".to_owned());}
    let mut response = match  client.post(url).body(body_content.to_owned()).send() {
        Ok(result) => result,
        Err(err) => return Err("could not connect".to_string()),
    };

    info!("Response: {:?}", response);
    if !response.status().is_success() {return Err("POST failed".to_string());}

    let mut content = String::new();
    match response.read_to_string(&mut content) {
        Ok(_) => {info!("Response: {}", content); Ok(content.to_owned())},
        Err(_) => Err("could not read response".to_string()),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use utils::httpclient;

    const URL: &'static str = mockito::SERVER_URL;

    #[test]
    fn test_httpclient_fails_with_no_response() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let mut my_url = String::from("http://127.0.0.1:3333");
        my_url.push_str("/nothing");

        match httpclient::post("anything", &my_url) {
            Ok(x) => assert_eq!(x, "nothing"), //should fail if we get here
            Err(x) => assert_eq!(x, "could not connect"),
        };
    }

    #[test]
    fn test_httpclient_success() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let _m = mockito::mock("POST", "/agent/core")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("world")
            .create();

        let mut my_url = String::from(URL);
        my_url.push_str("/agent/core");

        match httpclient::post("anything", &my_url) {
            Err(x) => assert_eq!(1,0), //should fail if we get here
            Ok(x) => assert_eq!(x,"world"),
        };
    }

    #[test]
    fn test_httpclient_fails_with_bad_url() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let _m = mockito::mock("POST", "/agent/core")
            .with_status(202)
            .with_header("content-type", "text/plain")
            .with_body("world")
            .create();

        let mut my_url = String::from(URL);
        my_url.push_str("/garbage");

        match httpclient::post("anything", &my_url) {
            Ok(x) => assert_eq!(x, "garbage"), //should fail if we get here
            Err(x) => assert_eq!(x, "POST failed"),
        };
    }

    #[test]
    fn test_httpclient_fails_with_404() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        let _m = mockito::mock("POST", "/agent/core")
            .with_status(404)
            .with_header("content-type", "text/plain")
            .with_body("world")
            .create();

        let mut my_url = String::from(URL);
        my_url.push_str("/agent/core");

        match httpclient::post("anything", &my_url) {
            Err(x) => assert_eq!(x,"POST failed"),
            Ok(x) => assert_eq!(x,"world"), //should fail if we get here
        };
    }

    #[test]
    fn test_httpclient_in_test_mode() {
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"true");

        let mut my_url = String::from(URL);
        my_url.push_str("/agent/core");

        match httpclient::post("anything", &my_url) {
            Err(x) => assert_eq!(1,0), //should fail if we get here
            Ok(x) => assert_eq!(x,"test_mode_response"),
        };
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
    }
}
