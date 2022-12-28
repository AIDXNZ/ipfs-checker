use std::{thread, fs::File, io::BufReader, ops::Mul, borrow::BorrowMut, time::Duration};
use clap::{command, arg, value_parser};
use futures::{prelude, executor::block_on, future::select, StreamExt, select};
use libp2p::{Multiaddr, PeerId, Swarm, identity, identify, core::{upgrade::Version, transport::Boxed, muxing::StreamMuxerBox}, tcp::async_io, dns::DnsConfig, noise, yamux::YamuxConfig, swarm::SwarmEvent};
use serde_derive::{Serialize, Deserialize};
use yaml_rust::YamlLoader;
use libp2p::Transport;
use std::io::prelude::*;
use async_std::{prelude::*, path::PathBuf};

#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    addrs: Vec<String>,
}

async fn build_transport(
    key_pair: identity::Keypair,
    
) -> Boxed<(PeerId, StreamMuxerBox)>{
    let base_transport = DnsConfig::system(async_io::Transport::new(
        libp2p::tcp::Config::default().port_reuse(true).nodelay(true),
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
    let timeout = async_std::future::timeout(Duration::from_secs(30), async_main());
    let handler = thread::spawn(||{
        block_on(timeout);
    });
    handler.join().unwrap();
}

async fn async_main() -> Result<(), ::std::io::Error> {

    let matches = command!()
        .arg(
            arg!(
                -f --file <File> "Config Path"
            ).value_parser(value_parser!(PathBuf)))
        .get_matches();

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

    let path = matches.get_one::<PathBuf>("file").unwrap();


    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let cfg = YamlLoader::load_from_str(&contents).unwrap();

    let doc = &cfg[0];
    
    let len = &doc["addrs"].clone().into_iter().count();

    

    let mut failed_addrs = vec![];

    for i in 0..len.clone() {
        let item = doc["addrs"][i].as_str().unwrap();
        let check = item.parse::<Multiaddr>();

        match check {
            Ok(addr) => {
                swarm.dial(addr).unwrap()
            },
            Err(err) => {
                failed_addrs.push(item);
                println!("Error {:?}", err);
            }
        }

    }




    loop {
        select! {
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::ConnectionEstablished {
                        peer_id,
                        endpoint,
                        ..
                    } => {
                        println!("{:?},", endpoint.get_remote_address());
                    },
                    SwarmEvent::OutgoingConnectionError {peer_id, error} => {
                            println!("Connection Error: {:?}", error);
                        }
                    _ => (),
                }
            }
        }
    }
    
    Ok(())
} 