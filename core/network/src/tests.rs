use crate::*;

// mock of NetworkHandlerTrait
struct NetworkHandlerTest<'a> {
    pub broadcasted_msg: &'a mut Vec<Vec<u8>>,
}

impl<'a> Unpin for NetworkHandlerTest<'a> {}

impl<'a> NetworkHandlerTrait for NetworkHandlerTest<'a> {
    fn broadcast_msg(&mut self, msg: Vec<u8>) {
        self.broadcasted_msg.push(msg);
    }

    fn receive_msg(&mut self, _: Vec<u8>) {}
}

impl<'a> futures::Stream for NetworkHandlerTest<'a> {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(()))
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn basic_test() {
    static mut BROADCASTED_MSG: Vec<Vec<u8>> = Vec::new();
    unsafe {
        let handler = NetworkHandlerTest {
            broadcasted_msg: &mut BROADCASTED_MSG,
        };

        let (mut service, worker) = build_network(handler);
        tokio::spawn(worker);

        let msg = vec![1, 2, 3, 4, 5];
        service.broadcast_msg(msg.clone()).await.unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));

        assert_eq!(BROADCASTED_MSG.pop(), Some(msg));
    }
}
