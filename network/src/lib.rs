#[cfg(test)]
mod test;

pub trait PeerTrait {
    fn send_msg(&mut self, msg: Vec<u8>);

    fn receive_msg(&self) -> Vec<u8>;
}

pub trait NetworkServiceTrait<PeerT>
where
    PeerT: PeerTrait,
{
    fn get_peers(&self) -> Vec<PeerT>;

    fn broadcast_msg(&mut self, msg: Vec<u8>);

    fn receive_msg(&self) -> (PeerT, Vec<u8>);

    fn disconnect_peer(&mut self, peer: PeerT);

    fn add_peer(&mut self, peer: PeerT);
}

pub struct NetworkService<PeerT>
where
    PeerT: PeerTrait,
{
    peers: Vec<PeerT>,
}

impl<PeerT> NetworkService<PeerT>
where
    PeerT: PeerTrait,
{
    pub fn broadcast_msg(&mut self, msg: Vec<u8>) {
        self.peers
            .iter_mut()
            .for_each(|peer| peer.send_msg(msg.clone()));
    }
}
