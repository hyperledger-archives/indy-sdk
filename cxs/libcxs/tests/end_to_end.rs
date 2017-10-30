extern crate cxs;
extern crate tempfile;

use tempfile::NamedTempFileOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::ffi::CString;
use cxs::api;

static CONFIG: &'static str = r#"
{
    "agent_endpoint": "https://agency-ea-sandbox.evernym.com",
    "agency_pairwise_did":"72x8p4HubxzUK1dwxcc5FU",
    "agent_pairwise_did":"UJGjM6Cea2YVixjWwHN9wq",
    "enterprise_did_agency":"RF3JM851T4EQmhh8CdagSP",
    "enterprise_did_agent":"JmvnKLYj7b7e5ywLxkRMjM",
    "enterprise_name":"enterprise",
    "logo_url":"https://s19.postimg.org/ykyz4x8jn/evernym.png",
    "agency_pairwise_verkey":"7118p4HubxzUK1dwxcc5FU",
    "agent_pairwise_verkey":"U22jM6Cea2YVixjWwHN9wq"
}
"#;

#[ignore]
#[test]
fn connection_ete() {
    let mut file = NamedTempFileOptions::new()
        .suffix(".json")
        .create()
        .unwrap();
    file.write_all(CONFIG.as_bytes()).unwrap();
    //    thread::sleep(Duration::from_secs(100));
    let path = CString::new(file.path().to_str().unwrap()).unwrap();
    let mut r = api::cxs::cxs_init(path.as_ptr());
    assert!(r == 0);
    thread::sleep(Duration::from_secs(1));

    let mut handle: u32 = 0;
    let id = CString::new("{\"id\":\"ckmMPiEDcH4R5URY\"}").unwrap();
    let options = CString::new("{\"phone\":\"\"}").unwrap(); //ADD PHONE NUMBER
    r = api::connection::cxs_connection_create(id.as_ptr(), std::ptr::null(), std::ptr::null(), &mut handle);
    assert!(r == 0);
    thread::sleep(Duration::from_secs(1));
    r = api::connection::cxs_connection_connect(handle, options.as_ptr());
    assert!(r == 0);
    thread::sleep(Duration::from_secs(1));
    unsafe {
        print!("{}", CString::from_raw(api::connection::cxs_connection_get_data(handle)).into_string().unwrap());
    }

//    while true {
//        let mut status: u32 = 0;
//        print!("{}", api::connection::cxs_connection_get_state(handle, &mut status));
//        thread::sleep(Duration::from_secs(5))
//    }
}
