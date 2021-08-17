use crate::*;

pub struct TestPeer {
    pub received_msg: Vec<u8>,
    pub send_msg: Vec<u8>,
}

impl PeerTrait for TestPeer {
    fn send_msg(&mut self, msg: Vec<u8>) {
        self.send_msg = msg;
    }

    fn receive_msg(&self) -> Vec<u8> {
        self.received_msg.clone()
    }
}

#[test]
fn peers_simple_test() {
    let mut peer = TestPeer {
        send_msg:  "".into(),
        received_msg: "".into(),
    };

    let msg1 = Vec::from("Hello, how are you");
    let msg2 = Vec::from("I am fine, thank you");

    peer.send_msg(msg1.clone());

    assert_eq!(peer.send_msg, msg1);

    peer.received_msg = msg2.clone();

    assert_eq!(peer.receive_msg(), msg2)
}
