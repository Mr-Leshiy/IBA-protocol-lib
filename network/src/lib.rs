#[cfg(test)]
mod test;

use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, self},
    identity,
    swarm::NetworkBehaviourEventProcess,
    tcp, NetworkBehaviour, PeerId,
};

#[derive(NetworkBehaviour)]
pub struct NetworkService {
    floodsub: Floodsub,
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for NetworkService {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(_) = event {
            // process from the peer
        }
    }
}

impl NetworkService {
    fn new(topic_name: String) -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from_public_key(local_key.public());
        println!("Local peer id: {:?}", peer_id);

        let transport = {
            // create a simple TCP transport
            let tcp = tcp::TcpConfig::new();
            tcp
        };

        let floodsub_topic = floodsub::Topic::new(topic_name);

        Self {
            floodsub: Floodsub::new(peer_id),
        }
    }
}
