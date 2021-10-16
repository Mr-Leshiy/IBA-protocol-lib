use crate::*;

// mock of NetworkHandlerTrait
struct NetworkHandlerTest<'a> {
    pub received_msg: &'a mut Vec<Vec<u8>>,
}

impl<'a> Unpin for NetworkHandlerTest<'a> {}

impl<'a> NetworkHandlerTrait for NetworkHandlerTest<'a> {
    fn broadcast_msg(&mut self, msg: Vec<u8>) {
        self.received_msg.push(msg);
    }
}

impl<'a> futures::Stream for NetworkHandlerTest<'a> {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(()))
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn basic_test() {
    static mut RECEIVED_MSG: Vec<Vec<u8>> = Vec::new();
    unsafe {
        let handler = NetworkHandlerTest {
            received_msg: &mut RECEIVED_MSG,
        };

        let (mut service, worker) = build_network(handler);
        tokio::spawn(worker);

        let msg = vec![1, 2, 3, 4, 5];
        service.broadcast_msg(msg.clone()).await.unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(RECEIVED_MSG.pop(), Some(msg));
    }
}
