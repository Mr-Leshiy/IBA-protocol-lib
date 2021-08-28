use crate::behaviour::Behaviour;
use std::error::Error;
use libp2p::{Swarm, floodsub::{self,Floodsub}, identity, mdns::Mdns,
mplex, noise, tcp, PeerId, Transport,};

pub type NetworkHandler<MsgHandlerF> = Swarm<Behaviour<MsgHandlerF>>;

impl<MsgHandlerF> crate::NetworkHandlerTrait for NetworkHandler<MsgHandlerF> where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send
{
}

pub fn build_handler<MsgHandlerF>(
    topic_name: String,
    msg_handler: MsgHandlerF,
) -> Result<NetworkHandler<MsgHandlerF>, Box<dyn Error>>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
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

    let behaviour = Behaviour::new(floodsub, mdns, floodsub_topic, msg_handler);

    let mut handler = Swarm::new(transport, behaviour, local_peer_id);
    handler.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    Ok(handler)
}
