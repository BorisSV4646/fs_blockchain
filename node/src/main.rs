use log::{error, info};
use node::core::errors::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
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

    write_local_ip_to_file("node/config/config.toml").await?;
    let config = load_config("node/config/config.toml")?;

    let socket_addr = format!("0.0.0.0:{}", DEFAULT_PORT);
    let listener = TcpListener::bind(&socket_addr).await?;
    info!("Нода запущена и слушает на {}", socket_addr);

    tokio::spawn(async move {
        loop {
            match listener.accept().await {
                Ok((mut socket, addr)) => {
                    info!("Принято соединение от {}", addr);
                    tokio::spawn(async move {
                        let mut buf = vec![0u8; 1024];
                        loop {
                            match socket.read(&mut buf).await {
                                Ok(0) => {
                                    info!("Соединение закрыто {}", addr);
                                    break;
                                }
                                Ok(n) => {
                                    let msg = String::from_utf8_lossy(&buf[..n]);
                                    info!("Получено сообщение от {}: {}", addr, msg);
                                    if let Err(e) =
                                        socket.write_all("Сообщение получено".as_bytes()).await
                                    {
                                        error!("Ошибка записи для {}: {:?}", addr, e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    error!("Ошибка чтения от {}: {:?}", addr, e);
                                    break;
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    error!("Ошибка принятия соединения: {:?}", e);
                }
            }
        }
    });

    for peer in config.peers {
        let peer_addr = peer.address;
        tokio::spawn(async move {
            if let Err(e) = connect_and_send(peer_addr.clone()).await {
                error!("Ошибка подключения к {}: {:?}", peer_addr, e);
            }
        });
    }

    loop {
        sleep(Duration::from_secs(10)).await;
    }
}

async fn get_public_ip() -> Result<String> {
    let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(ip)
}

async fn write_local_ip_to_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let config_str = fs::read_to_string(&path)?;
    let mut config: Config = toml::from_str(&config_str)?;
    let ip = get_public_ip().await?;
    let ip_str = ip.to_string();

    config.peers.push(Peer {
        address: format!("{}:{}", ip_str, DEFAULT_PORT),
    });
    let new_config_str = toml::to_string_pretty(&config)?;
    fs::write(path, new_config_str)?;
    Ok(())
}

fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

async fn connect_and_send(addr: String) -> Result<()> {
    let mut stream = TcpStream::connect(&addr).await?;
    info!("Подключились к пиру {}", addr);
    stream.write_all("Привет от ноды".as_bytes()).await?;
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    info!("Получен ответ от {}: {}", addr, response);
    Ok(())
}
