mod behaviour;
use async_std::channel::{self, Receiver, SendError, Sender};
use behaviour::Behaviour;
use libp2p::{
    floodsub::{self, Floodsub},
    futures::StreamExt,
    identity,
    mdns::Mdns,
    mplex, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, PeerId, Swarm, Transport,
};
use std::error::Error;

pub struct NetworkWorker<BehaviourT>
where
    BehaviourT: NetworkBehaviour,
{
    swarm: Swarm<BehaviourT>,

    from_service: Receiver<Vec<u8>>,
}

pub struct NetworkService {
    to_worker: Sender<Vec<u8>>,
}

pub fn build_network<MsgHandlerF>(
    topic_name: String,
    msg_handler: MsgHandlerF,
) -> Result<(NetworkService, NetworkWorker<Behaviour<MsgHandlerF>>), Box<dyn Error>>
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

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let (to_worker, from_service) = channel::unbounded();

    let worker = NetworkWorker {
        swarm,
        from_service,
    };
    let service = NetworkService { to_worker };

    Ok((service, worker))
}

impl NetworkService {
    pub async fn broadcast_msg(&mut self, msg: Vec<u8>) -> Result<(), SendError<Vec<u8>>> {
        self.to_worker.send(msg).await
    }
}

impl<MsgHandlerF> NetworkWorker<Behaviour<MsgHandlerF>>
where
    MsgHandlerF: 'static + FnMut(Vec<u8>) + Send,
{
    pub async fn run(&mut self) {
        loop {
            futures::select! {
                msg = self.from_service.select_next_some() => {
                    self.swarm.behaviour_mut().broadcast_msg(msg);
                }
                event = self.swarm.select_next_some() => {
                    if let SwarmEvent::NewListenAddr { address, .. } = event {
                        println!("Listening on: {:?}", address);
                    }
                }
            }
        }
    }
}
