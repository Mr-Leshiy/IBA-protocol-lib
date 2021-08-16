#[cfg(test)]
mod mock;
#[cfg(test)]
mod test;

use bytes::Bytes;

pub trait Msg {
    fn data(&self) -> Bytes;
}

pub trait Peer<T: Msg> {
    fn send_msg(&mut self, msg: T);

    fn receive_msg(&mut self) -> T;
}
