extern crate rand;
extern crate serde_json;

use std::collections::HashMap;
use api::CxsStateType;
use api::Errorcode;
use rand::Rng;
use std::sync::Mutex;

lazy_static! {
    static ref CONNECTION_MAP: Mutex<HashMap<u32, Box<Connection>>> = Default::default();
}

#[derive(Serialize, Deserialize)]
struct Connection {
    info: String,
    handle: u32,
    did: String,
    did_endpoint: String,
    wallet: String,
    state: CxsStateType,
}

pub fn build_connection (info_string: String) -> u32 {
    let new_handle = rand::thread_rng().gen::<u32>();

    let c = Box::new(Connection {
            info: info_string,
            handle: new_handle,
            did: String::new(),
            did_endpoint: String::new(),
            wallet: String::new(),
            state: CxsStateType::CxsStateInitialized,
        });

    let mut m = CONNECTION_MAP.lock().unwrap();
    m.insert(new_handle, c);

    new_handle
}

impl Connection {
    fn connect(&mut self) -> Errorcode { self.state = CxsStateType::CxsStateOfferSent; Errorcode::Success }
    fn get_state(&self) -> u32 { let state = self.state as u32; state }
}

impl Drop for Connection {
    fn drop(&mut self) {}
}

pub fn get_state(handle: u32) -> u32 {
    let m = CONNECTION_MAP.lock().unwrap();
    let result = m.get(&handle);

    let rc = match result {
        Some(t) => t.get_state(),
        None => CxsStateType::CxsStateNone as u32,
    };

    rc
}

pub fn connect(handle: u32) -> Errorcode {
    let mut m = CONNECTION_MAP.lock().unwrap();
    let result = m.get_mut(&handle);

    let rc = match result {
       Some(t) => t.connect(),
       None => Errorcode::Failure,
    };

    rc
}

pub fn to_string(handle:u32) -> String {
    let m = CONNECTION_MAP.lock().unwrap();
    let result = m.get(&handle);

    let connection_json = match result {
        Some(t) => serde_json::to_string(&t).unwrap(),
        None => String::new(),
    };

    connection_json.to_owned()
}

#[allow(unused_variables)]
pub fn release(handle:u32) -> Errorcode {
    let mut m = CONNECTION_MAP.lock().unwrap();
    let result = m.remove(&handle);

    let rc = match result {
        Some(t) => Errorcode::Success,
        None => Errorcode::Failure,
    };

    rc
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_connection() {
        let handle = build_connection("Whatever".to_owned());

        assert!(handle > 0);

        release(handle);
    }

    #[test]
    fn test_connection_release() {
        let handle = build_connection("Whatever".to_owned());

        assert!(handle > 0);

        let rc = release(handle);

        assert_eq!(rc, Errorcode::Success);
    }

    #[test]
    fn test_connect() {
        let handle = build_connection("Whatever".to_owned());

        assert!(handle > 0);

        let rc = connect(handle);

        assert_eq!(rc, Errorcode::Success);

        let state = get_state(handle);

        assert_eq!(state, CxsStateType::CxsStateOfferSent as u32);
        release(handle);
    }

    #[test]
    fn test_connect_fails() {
        // Need to add content here once we've implemented connected
        assert_eq!(0,0);
    }

    #[test]
    fn test_connection_release_fails() {
        let rc = release(1);
        
        assert_eq!(rc, Errorcode::Failure);    
    }

    #[test]
    fn test_get_state() {
        let handle = build_connection("Whatever".to_owned());

        let state = get_state(handle);

        assert_eq!(state, CxsStateType::CxsStateInitialized as u32);
        release(handle);
    }

    #[test]
    fn test_get_state_fails() {
        let state = get_state(1);

        assert_eq!(state, CxsStateType::CxsStateNone as u32);
    }

    #[test]
    fn test_get_string_fails() {
        let string = to_string(1);

        assert_eq!(string.len(), 0);
    }

    #[test]
    fn test_get_string() {
        let handle = build_connection("".to_owned());

        let string = to_string(handle);

        println!("string: {}", string);
        assert!(string.len() > 10);
        release(handle);
    }

    #[test]
    fn test_many_handles() {

        let handle1 = build_connection("handle1".to_owned());
        let handle2 = build_connection("handle2".to_owned());
        let handle3 = build_connection("handle3".to_owned());
        let handle4 = build_connection("handle4".to_owned());
        let handle5 = build_connection("handle5".to_owned());

        connect(handle1);
        connect(handle2);
        connect(handle3);
        connect(handle4);
        connect(handle5);

        let data1 = to_string(handle1);
        let data2 = to_string(handle2);
        let data3 = to_string(handle3);
        let data4 = to_string(handle4);
        let data5 = to_string(handle5);

        println!("handle1: {}", data1);
        println!("handle2: {}", data2);
        println!("handle3: {}", data3);
        println!("handle4: {}", data4);
        println!("handle5: {}", data5);

        release(handle1);
        release(handle2);
        release(handle3);
        release(handle4);
        release(handle5);

        /* This only works when you run "cargo test -- --test-threads=1 */
        //let m = CONNECTION_MAP.lock().unwrap();
        //assert_eq!(0,m.len());
    }
}
