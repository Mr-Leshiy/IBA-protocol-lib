use bytes::Bytes;

use crate::*;

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
