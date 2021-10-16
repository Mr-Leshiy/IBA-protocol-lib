use libp2p::{
    floodsub::{Floodsub, FloodsubEvent, Topic},
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};

#[derive(NetworkBehaviour)]
pub struct Behaviour<MsgHandlerF>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
    floodsub: Floodsub,
    mdns: Mdns,

    #[behaviour(ignore)]
    floodsub_topic: Topic,
    #[behaviour(ignore)]
    msg_handler: MsgHandlerF,
}

impl<MsgHandlerF> Behaviour<MsgHandlerF>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
    pub fn new(
        floodsub: Floodsub,
        mdns: Mdns,
        floodsub_topic: Topic,
        msg_handler: MsgHandlerF,
    ) -> Self {
        Self {
            floodsub,
            mdns,
            floodsub_topic,
            msg_handler,
        }
    }

    pub fn broadcast_msg(&mut self, msg: Vec<u8>) {
        let topic = self.floodsub_topic.clone();
        self.floodsub.publish(topic, msg);
    }
}

impl<MsgHandlerF> NetworkBehaviourEventProcess<FloodsubEvent> for Behaviour<MsgHandlerF>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
    fn inject_event(&mut self, event: FloodsubEvent) {
        if let FloodsubEvent::Message(message) = event {
            (self.msg_handler)(message.data);
        }
    }
}

impl<MsgHandlerF> NetworkBehaviourEventProcess<MdnsEvent> for Behaviour<MsgHandlerF>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
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
