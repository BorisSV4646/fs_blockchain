use log::{error, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{fs, io};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

const DEFAULT_PORT: u16 = 8808;

#[derive(Serialize, Deserialize, Debug)]
struct Peer {
    address: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    my_address: String,
    peers: Vec<Peer>,
}

async fn get_public_ip() -> Result<String, Box<dyn Error>> {
    let ip = reqwest::get("https://api.ipify.org").await?.text().await?;
    Ok(ip)
}

pub async fn write_local_ip_to_file<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let config_str = fs::read_to_string(&path)?;

    let mut config: Config = toml::from_str(&config_str).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Ошибка парсинга TOML: {}", e))
    })?;

    let ip = get_public_ip().await?;
    let ip_str = ip.to_string();

    // Обновляем поле my_address (например, можно добавить порт, если требуется)
    config.my_address = format!("{}:{}", ip_str, DEFAULT_PORT);

    // Сериализуем обновлённую конфигурацию обратно в TOML-строку
    let new_config_str = toml::to_string_pretty(&config).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Ошибка сериализации TOML: {}", e),
        )
    })?;

    // Записываем строку обратно в файл (перезаписываем файл)
    fs::write(path, new_config_str)?;
    Ok(())
}

/// Загружает конфигурацию из файла (например, "config.toml")
fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    info!("Запуск ноды блокчейна FS Spin");

    if let Err(e) = write_local_ip_to_file("src/bin/config.toml").await {
        eprintln!("Ошибка записи IP в файл: {}", e);
    } else {
        info!("Локальный IP записан в файл shared_ips.txt");
    }

    let config = load_config("src/bin/config.toml")?;
    info!("Конфигурация загружена: {:?}", config);

    // Запускаем TCP-сервер, который слушает на адресе, указанном в конфигурации
    let listener = TcpListener::bind(&config.my_address)
        .await
        .expect("Не удалось привязать адрес");
    info!("Нода запущена и слушает на {}", config.my_address);

    Ok(())
    // // Запускаем задачу для обработки входящих соединений
    // tokio::spawn(async move {
    //     loop {
    //         match listener.accept().await {
    //             Ok((mut socket, addr)) => {
    //                 info!("Принято соединение от {}", addr);
    //                 tokio::spawn(async move {
    //                     let mut buf = vec![0u8; 1024];
    //                     loop {
    //                         match socket.read(&mut buf).await {
    //                             Ok(0) => {
    //                                 info!("Соединение закрыто {}", addr);
    //                                 break;
    //                             }
    //                             Ok(n) => {
    //                                 let msg = String::from_utf8_lossy(&buf[..n]);
    //                                 info!("Получено сообщение от {}: {}", addr, msg);
    //                                 if let Err(e) =
    //                                     socket.write_all("Сообщение получено".as_bytes()).await
    //                                 {
    //                                     error!("Ошибка записи для {}: {:?}", addr, e);
    //                                     break;
    //                                 }
    //                             }
    //                             Err(e) => {
    //                                 error!("Ошибка чтения от {}: {:?}", addr, e);
    //                                 break;
    //                             }
    //                         }
    //                     }
    //                 });
    //             }
    //             Err(e) => {
    //                 error!("Ошибка принятия соединения: {:?}", e);
    //             }
    //         }
    //     }
    // });

    // // Подключаемся к пировым нодам, указанным в конфигурации, и отправляем тестовое сообщение
    // for peer in config.peers {
    //     let peer_addr = peer.address;
    //     tokio::spawn(async move {
    //         if let Err(e) = connect_and_send(peer_addr.clone()).await {
    //             error!("Ошибка подключения к {}: {:?}", peer_addr, e);
    //         }
    //     });
    // }

    // // Чтобы программа не завершалась сразу, засыпаем в бесконечном цикле
    // loop {
    //     sleep(Duration::from_secs(10)).await;
    // }
}

/// Подключается к указанному адресу, отправляет сообщение и выводит ответ
async fn connect_and_send(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(&addr).await?;
    info!("Подключились к пиру {}", addr);
    stream.write_all("Привет от ноды".as_bytes()).await?;
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    info!("Получен ответ от {}: {}", addr, response);
    Ok(())
}
