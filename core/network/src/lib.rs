mod libp2p_impl;

pub use libp2p_impl::handler::build_handler;

use async_std::channel::{self, Receiver, SendError, Sender};
use futures::{Stream, StreamExt};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

#[cfg(test)]
mod tests;

pub trait NetworkHandlerTrait: Stream + Unpin {
    fn broadcast_msg(&mut self, msg: Vec<u8>);

    fn receive_msg(&mut self, msg: Vec<u8>);
}

pub struct NetworkWorker<NetworkHandler: NetworkHandlerTrait> {
    handler: NetworkHandler,

    from_service: Receiver<Vec<u8>>,
}

impl<NetworkHandler: NetworkHandlerTrait> Future for NetworkWorker<NetworkHandler> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // process msgs
        let mut num_iterations = 0;
        loop {
            num_iterations += 1;
            if num_iterations >= 100 {
                cx.waker().wake_by_ref();
                break;
            }

            let msg = match self.from_service.poll_next_unpin(cx) {
                Poll::Ready(Some(msg)) => msg,
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Pending => break,
            };

            self.handler.broadcast_msg(msg);
        }

        // process events
        let mut num_iterations = 0;
        loop {
            num_iterations += 1;
            if num_iterations >= 100 {
                cx.waker().wake_by_ref();
                break;
            }

            let _ = match self.handler.poll_next_unpin(cx) {
                Poll::Ready(Some(event)) => event,
                Poll::Ready(None) => return Poll::Ready(()),
                Poll::Pending => break,
            };
        }

        Poll::Pending
    }
}

pub struct NetworkService {
    to_worker: Sender<Vec<u8>>,
}

impl NetworkService {
    pub async fn broadcast_msg(&mut self, msg: Vec<u8>) -> Result<(), SendError<Vec<u8>>> {
        self.to_worker.send(msg).await
    }
}

pub fn build_network<'a, NetworkHandler: 'a + NetworkHandlerTrait>(
    handler: NetworkHandler,
) -> (NetworkService, NetworkWorker<NetworkHandler>) {
    let (to_worker, from_service) = channel::unbounded();

    let worker = NetworkWorker {
        handler,
        from_service,
    };
    let service = NetworkService { to_worker };

    (service, worker)
}
