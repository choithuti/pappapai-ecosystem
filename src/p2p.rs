// src/p2p.rs – FINAL
use libp2p::{gossipsub, identity, kad, mdns, noise, swarm::{SwarmEvent, SwarmBuilder}, tcp, yamux, PeerId, Multiaddr};
use futures::StreamExt;

pub async fn start_p2p(bus: GlobalBus, crypto: Arc<CryptoEngine>, snn: Arc<SNNCore>) {
    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(local_key.public());
    println!("Pappap Peer ID: {}", peer_id);

    let mut swarm = SwarmBuilder::with_tokio_executor(
        tcp::tokio::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1Lazy)
            .authenticate(noise::Config::new(&local_key).unwrap())
            .multiplex(yamux::Config::default())
            .boxed(),
        PappapBehaviour { gossipsub: ..., kad: ..., mdns: ... },
        peer_id,
    ).build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/9000".parse().unwrap()).unwrap();

    let mut rx = bus.subscribe();

    loop {
        tokio::select! {
            Some((target, data)) = rx.recv() => {
                if target == "gossip" {
                    let encrypted = crypto.encrypt(&data);
                    swarm.behaviour_mut().gossipsub.publish(TOPIC, encrypted)?;
                }
            }
            event = swarm.select_next_some() => {
                if let SwarmEvent::Behaviour(BehaviourEvent::Gossipsub(msg)) = event {
                    if let Ok(plain) = crypto.decrypt(&msg.data) {
                        if snn.threat_score(&plain).await < 0.9 {
                            bus.send("network", plain);
                        }
                    }
                }
            }
        }
    }
}