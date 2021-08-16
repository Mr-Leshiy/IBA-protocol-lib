use crate::*;
use mock::*;

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
