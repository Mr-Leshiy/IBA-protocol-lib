use crate::*;
use bytes::Bytes;
#[derive(Clone)]
pub struct TestMsg {
    pub data: &'static str,
}

impl Msg for TestMsg {
    fn data(&self) -> Bytes {
        self.data.into()
    }
}

pub struct TestPeer {
    pub received_msg: TestMsg,
    pub send_msg: TestMsg,
}

impl Peer<TestMsg> for TestPeer {
    fn send_msg(&mut self, msg: TestMsg) {
        self.send_msg = msg;
    }

    fn receive_msg(&mut self) -> TestMsg {
        self.received_msg.clone()
    }
}


#[test]
fn peers_simple_test() {
    let mut peer = TestPeer {
        send_msg: TestMsg { data: "" },
        received_msg: TestMsg { data: "" },
    };

    peer.send_msg(TestMsg {
        data: "Hello, how are you",
    });

    assert_eq!(peer.send_msg.data, "Hello, how are you");

    peer.received_msg = TestMsg { data: "I am fine" };

    assert_eq!(peer.receive_msg().data, "I am fine");
}
