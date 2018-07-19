use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;

use serde_json;
use serde_json::Value as JValue;

use errors::common::CommonError;


// NOTE: The following use of lifetimes seems incorrect, both network and peers should have
// independent lifetimes. But the following code makes their lifetimes same
pub struct Peer<'a> {
    pub name: String,
    // An peer can be on any number of networks but for simplicity keeping it one.
    // Else HashMap of networks is needed
    pub network: Option<Rc<RefCell<Network<'a>>>>,
    // Incase of multiple networks, needs to be an HashMap of inboxes
    // TODO: Add mutex
    pub inbox: VecDeque<String>
}

pub struct Network<'a> {
    pub name: String,
    // TODO: Add mutex
    pub peers: HashMap<String, Rc<RefCell<Peer<'a>>>>
}

impl<'a> Network<'a> {
    pub fn new(name: &str) -> Self {
        Network {
            name: name.to_string(),
            peers: HashMap::new()
        }
    }

    pub fn register_peer(&mut self, peer: Rc<RefCell<Peer<'a>>>) {
        self.peers.insert(peer.borrow().name.to_owned(), Rc::clone(&peer));
    }

    pub fn send_message(&self, msg: &str, dest_id: &str) -> Result<(), CommonError> {
        match self.peers.get(dest_id) {
            Some(peer) => {
                Ok(peer.borrow_mut().inbox.push_back(msg.to_string()))
            },
            None => Err(CommonError::InvalidState(format!("Cannot find peer id {}", dest_id)))
        }
    }
}

impl<'a> Peer<'a> {
    pub fn new(name: &str) -> Self {
        Peer {
            name: name.to_string(),
            network: None,
            inbox: VecDeque::new()
        }
    }

    /*pub fn set_network(&mut self, network: Rc<RefCell<Network<'a>>>) {
        self.network = Some(network);
    }*/

    /*pub fn register_to_network(mut self, network: Rc<RefCell<Network<'a>>>) -> Self {
        self.network = Some(network);
        let self_ = Rc::new(RefCell::new(self));
        network.borrow_mut().register_peer(self_);
        self
    }*/

    pub fn process(&mut self) -> Vec<String> {
        self.inbox.drain(..).collect()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_new_network(name: &str) -> Rc<RefCell<Network>> {
        Rc::new(RefCell::new(Network::new(name)))
    }

    #[test]
    fn test_creation() {
        let network = get_new_network("n");
        let peer1 = Peer::new("a1");
        let peer2 = Peer::new("a2");

        assert!(network.borrow().peers.is_empty());
        assert!(peer1.network.is_none());
        assert!(peer1.inbox.is_empty());
        assert!(peer2.network.is_none());
        assert!(peer2.inbox.is_empty());
    }

    /*#[test]
    fn test_registration() {
        let mut network = Rc::new(RefCell::new(Network::new("n")));
        let mut peer1 = Rc::new(RefCell::new(Peer::new("a1")));
        let mut peer2 = Rc::new(RefCell::new(Peer::new("a2")));

        {
            peer1.borrow_mut().register_to_network(Rc::clone(&network));
            peer2.borrow_mut().register_to_network(Rc::clone(&network));
        }
        assert!(peer1.borrow().network.is_some());
        assert!(peer2.borrow().network.is_some());

        *//*let a1 = Rc::clone(&peer1);
        let a2 = Rc::clone(&peer2);*//*

        *//*let n1 = a1.borrow().network.unwrap();
        let n2 = a2.borrow().network.unwrap();*//*

        let n1 = peer1.borrow().network.clone().unwrap();
        let n2 = peer2.borrow().network.clone().unwrap();

        assert_eq!(&n1.borrow().name, &n2.borrow().name);

        assert!(network.borrow().peers.get(&peer1.borrow().name).is_some());
        assert!(network.borrow().peers.get(&peer2.borrow().name).is_some());
    }*/

    #[test]
    fn test_registration() {
        let network = get_new_network("n");
        let peer1 = Rc::new(RefCell::new(Peer::new("a1")));
        let peer2 = Rc::new(RefCell::new(Peer::new("a2")));

        assert!(network.borrow().peers.get(&peer1.borrow().name).is_none());
        assert!(network.borrow().peers.get(&peer2.borrow().name).is_none());

        network.borrow_mut().register_peer(Rc::clone(&peer1));
        network.borrow_mut().register_peer(Rc::clone(&peer2));

        assert!(network.borrow().peers.get(&peer1.borrow().name).is_some());
        assert!(network.borrow().peers.get(&peer2.borrow().name).is_some());
    }

    #[test]
    fn test_messaging() {
        let msg1 = json!({
            "k1": "v1"
        }).to_string();
        let msg2 = json!({
            "k2": "v2"
        }).to_string();

        let peer1_id = "a1";
        let peer2_id = "a2";

        let network = get_new_network("n");
        let peer1 = Rc::new(RefCell::new(Peer::new(peer1_id)));
        let peer2 = Rc::new(RefCell::new(Peer::new(peer2_id)));

        network.borrow_mut().register_peer(Rc::clone(&peer1));
        network.borrow_mut().register_peer(Rc::clone(&peer2));

        // sending msg1 to peer2
        assert_eq!(peer2.borrow().inbox.front(), None);
        network.borrow().send_message(&msg1, peer2_id).unwrap();
        assert_eq!(peer2.borrow().inbox.front().unwrap(), &msg1);

        // sending msg2 to peer1
        assert_eq!(peer1.borrow().inbox.front(), None);
        network.borrow().send_message(&msg2, peer1_id).unwrap();
        assert_eq!(peer1.borrow().inbox.front().unwrap(), &msg2);

        let m1 = peer1.borrow_mut().process();
        assert_eq!(m1[0], msg2);
        assert_eq!(peer1.borrow().inbox.front(), None);

        let m2 = peer2.borrow_mut().process();
        assert_eq!(m2[0], msg1);
        assert_eq!(peer2.borrow().inbox.front(), None);

        let msgs: Vec<String> = vec![
            json!({
                "k3": "v3"
            }).to_string(),
            json!({
                "k4": "v4"
            }).to_string(),
            json!({
                "k5": "v5"
            }).to_string(),
        ];

        for msg in msgs.clone() {
            network.borrow().send_message(&msg, peer1_id).unwrap();
        }
        assert_eq!(peer1.borrow_mut().process(), msgs);
    }
}
