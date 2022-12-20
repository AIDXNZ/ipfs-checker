use std::{thread, fs::File, io::BufReader, ops::Mul, borrow::BorrowMut};
use futures::{prelude, executor::block_on, future::select, StreamExt, select};
use libp2p::{Multiaddr, PeerId, Swarm, identity, identify, core::{upgrade::Version, transport::Boxed, muxing::StreamMuxerBox}, tcp::async_io, dns::DnsConfig, noise, yamux::YamuxConfig, swarm::SwarmEvent};
use serde_derive::{Serialize, Deserialize};
use yaml_rust::YamlLoader;
use libp2p::Transport;
use std::io::prelude::*;
use async_std::prelude::*;

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    addrs: Vec<String>,
}

async fn build_transport(
    key_pair: identity::Keypair,
    
) -> Boxed<(PeerId, StreamMuxerBox)>{
    let base_transport = DnsConfig::system(async_io::Transport::new(
        libp2p::tcp::Config::default().nodelay(true),
    )).await.unwrap();
    let noise_config = noise::NoiseAuthenticated::xx(&key_pair).unwrap();
    let yamux_config = YamuxConfig::default();

    base_transport
    .upgrade(Version::V1)
    .authenticate(noise_config)
    .multiplex(yamux_config)
    .boxed()
}

fn main() {
    let handler = thread::spawn(||{
        block_on(async_main());
    });
    handler.join().unwrap();
}

async fn async_main() -> Result<(), ::std::io::Error> {

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.clone().public());
    let transport = build_transport(local_key.clone()).await;
    let mut swarm = {
        Swarm::with_threadpool_executor(
            transport,
            identify::Behaviour::new(identify::Config::new("/ipfs/id/1.0.0".to_string(), local_key.clone().public())),
            local_peer_id
        )
    };

    let _ = swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap());
    for lis in swarm.listeners() {
        println!("Listening on {:?}", lis);
    }


    let mut file = File::open("config.yaml").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let cfg = YamlLoader::load_from_str(&contents).unwrap();

    let doc = &cfg[0];
    
    let len = &doc["addrs"].clone().into_iter().count();

    for i in 0..len.clone() {
        let item = doc["addrs"][i].as_str().unwrap();
        let check = item.parse::<Multiaddr>();

        match check {
            Ok(addr) => {
                swarm.dial(addr).unwrap()
            },
            Err(err) => println!("Invalid Multiaddress {:?}", item)
        }

    }

    loop {
        select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::Behaviour(identify::Event::Received {info, ..}) => {
                        println!("{:?}", info);
                    }
                    _ => (),
                }
            }
        }
    }
    
    Ok(())
} 