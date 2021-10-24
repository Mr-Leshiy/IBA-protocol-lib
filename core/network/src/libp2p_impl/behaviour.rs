use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    floodsub: Floodsub,
    mdns: Mdns,

    #[behaviour(ignore)]
    floodsub_topic: Topic,
}

impl Behaviour {
    pub fn new(floodsub: Floodsub, mdns: Mdns, floodsub_topic: Topic) -> Self {
        Self {
            floodsub,
            mdns,
            floodsub_topic,
        }
    }

    pub fn broadcast_msg(&mut self, msg: Vec<u8>) {
        let topic = self.floodsub_topic.clone();
        self.floodsub.publish(topic, msg);
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for Behaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(_) = event {}
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for Behaviour {
    // Called when `mdns` produces an event.
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer, _) in list {
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}
