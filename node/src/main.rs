use igd::aio::search_gateway;
use igd::PortMappingProtocol;
use local_ip_address::local_ip;
use log::{error, info};
use node::core::errors::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio::time::sleep;

const DEFAULT_PORT: u16 = 8808;

#[derive(Serialize, Deserialize, Debug)]
struct Peer {
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    peers: Vec<Peer>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    info!("Запуск ноды блокчейна FS Spin");

    let local_ipv4 = get_local_ipv4();
    if let Err(e) = setup_port_mapping(local_ipv4).await {
        error!("Ошибка проброса порта: {}", e);
    }

    let my_ip = write_local_ip_to_file("node/config/config.toml").await?;
    let config = load_config("node/config/config.toml")?;

    let mut peer_addrs = vec![];
    for peer in config.peers {
        if peer.address == my_ip {
            continue;
        }
        let peer_addr = peer.address;
        peer_addrs.push(peer_addr);
    }

    let socket_addr = format!("{}:{}", local_ipv4, DEFAULT_PORT);
    run_node(socket_addr.as_str(), peer_addrs).await?;
    Ok(())
}

async fn get_public_ip() -> Result<String> {
    let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(ip)
}

fn get_local_ipv4() -> Ipv4Addr {
    let ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());
    match ip {
        IpAddr::V4(ipv4) => ipv4,
        IpAddr::V6(_) => {
            info!("Получен IPv6, ожидается IPv4. Использую 0.0.0.0.");
            "0.0.0.0".parse().unwrap()
        }
    }
}

async fn write_local_ip_to_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let config_str = fs::read_to_string(&path)?;
    let mut config: Config = toml::from_str(&config_str)?;
    let ip = get_public_ip().await?;
    let ip_str = format!("{}:{}", ip.to_string(), DEFAULT_PORT);
    if !config.peers.iter().any(|peer| peer.address == ip_str) {
        config.peers.push(Peer {
            address: ip_str.clone(),
        });
    }
    let new_config_str = toml::to_string_pretty(&config)?;
    fs::write(path, new_config_str)?;
    Ok(ip_str)
}

fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

async fn setup_port_mapping(local_ip: Ipv4Addr) -> Result<()> {
    let local_addr = SocketAddrV4::new(local_ip, DEFAULT_PORT);

    let gateway = search_gateway(Default::default()).await?;

    let mut found = false;
    let mut index = 0;
    while let Ok(mapping) = gateway.get_generic_port_mapping_entry(index).await {
        if mapping.external_port == DEFAULT_PORT && mapping.protocol == PortMappingProtocol::TCP {
            info!("Проброс порта уже существует");
            found = true;
            break;
        }
        index += 1;
    }
    if !found {
        info!("Проброс порта не найден, добавляем его...");
        gateway
            .add_port(
                PortMappingProtocol::TCP,
                DEFAULT_PORT,
                local_addr,
                0,
                "FS Spin Node",
            )
            .await?;
        info!("Порт успешно проброшен!");
    }

    Ok(())
}

async fn run_node(listen_addr: &str, peer_addrs: Vec<String>) -> Result<()> {
    let listener = TcpListener::bind(listen_addr).await?;
    println!("Node listening on {}", listen_addr);

    spawn(async move {
        loop {
            match listener.accept().await {
                Ok((mut socket, addr)) => {
                    println!("Incoming connection from {}", addr);
                    spawn(async move {
                        let mut buf = vec![0u8; 1024];
                        loop {
                            match socket.read(&mut buf).await {
                                Ok(0) => {
                                    println!("Connection closed by {}", addr);
                                    break;
                                }
                                Ok(n) => {
                                    let msg = String::from_utf8_lossy(&buf[..n]);
                                    println!("Received from {}: {}", addr, msg);
                                    if let Err(e) = socket.write_all(b"Message received").await {
                                        eprintln!("Error writing to {}: {:?}", addr, e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error reading from {}: {:?}", addr, e);
                                    break;
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {:?}", e);
                }
            }
        }
    });

    for peer_addr in peer_addrs {
        let addr = peer_addr.clone();
        spawn(async move {
            match TcpStream::connect(&addr).await {
                Ok(mut stream) => {
                    println!("Connected to peer {}", addr);
                    if let Err(e) = stream.write_all(b"Hello from node!").await {
                        eprintln!("Error sending to {}: {:?}", addr, e);
                    }
                    let mut buf = vec![0u8; 1024];
                    match stream.read(&mut buf).await {
                        Ok(n) => {
                            let response = String::from_utf8_lossy(&buf[..n]);
                            println!("Received from {}: {}", addr, response);
                        }
                        Err(e) => eprintln!("Error reading from {}: {:?}", addr, e),
                    }
                }
                Err(e) => {
                    eprintln!("Error connecting to {}: {:?}", addr, e);
                }
            }
        });
    }

    loop {
        sleep(Duration::from_secs(10)).await;
    }
}
