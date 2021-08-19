#[cfg(test)]
mod test;

use std::{error::Error, os::unix::thread};

use libp2p::{
    floodsub::{self, Floodsub, FloodsubEvent, Topic},
    futures::StreamExt,
    identity,
    mdns::{Mdns, MdnsEvent},
    mplex, noise,
    swarm::{NetworkBehaviour, NetworkBehaviourEventProcess, SwarmEvent},
    tcp, NetworkBehaviour, PeerId, Swarm, Transport,
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

impl<MsgHandlerF> NetworkBehaviourEventProcess<FloodsubEvent> for Behaviour<MsgHandlerF>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
    fn inject_event(&mut self, event: FloodsubEvent) {
        println!("process event");
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
                    println!("Discovered peer: {:?}", peer);
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer, _) in list {
                    println!("Expired peer: {:?}", peer);
                    if !self.mdns.has_node(&peer) {
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

pub struct NetworkService<BehaviourT: NetworkBehaviour>
where
    BehaviourT: NetworkBehaviour,
{
    swarm: Swarm<BehaviourT>,
}

impl<MsgHandlerF> NetworkService<Behaviour<MsgHandlerF>>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
    pub fn new(topic_name: String, msg_handler: MsgHandlerF) -> Result<Self, Box<dyn Error>> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from_public_key(local_key.public());
        println!("Local peer id: {:?}", local_peer_id);

        // Create a keypair for authenticated encryption of the transport.
        let noise_key = noise::Keypair::<noise::X25519Spec>::new()
            .into_authentic(&local_key)
            .expect("Signing libp2p-noise static DH keypair failed.");

        let mut floodsub = Floodsub::new(local_peer_id);
        let floodsub_topic = floodsub::Topic::new(topic_name);
        floodsub.subscribe(floodsub_topic.clone());

        // Build transport
        let transport = {
            // create a simple TCP transport
            tcp::TcpConfig::new()
                .nodelay(true)
                .upgrade(libp2p::core::upgrade::Version::V1)
                .authenticate(noise::NoiseConfig::xx(noise_key).into_authenticated())
                .multiplex(mplex::MplexConfig::new())
                .boxed()
        };

        let mdns = async_std::task::block_on(Mdns::new(Default::default()))?;

        let behaviour = Behaviour {
            floodsub,
            mdns,
            floodsub_topic,
            msg_handler,
        };

        Ok(Self {
            swarm: Swarm::new(transport, behaviour, local_peer_id),
        })
    }

    pub fn run(service: &mut Self) -> Result<(), Box<dyn Error>> {
        service
            .swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        std::thread::spawn(
            || async {
                loop {
                    if let Some(SwarmEvent::NewListenAddr { address, .. }) = service.swarm.next().await {
                        println!("Listening on {:?}", address);
                    }
                }
            }
        );

        Ok(())
    }

    pub fn broadcast_msg(&mut self, msg: impl Into<Vec<u8>>) {
        let topic = self.swarm.behaviour().floodsub_topic.clone();
        self.swarm.behaviour_mut().floodsub.publish(topic, msg);
    }
}
