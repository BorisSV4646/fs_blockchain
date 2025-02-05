use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <port> [peer_ports...]", args[0]);
        return;
    }
    let my_port: u16 = args[1].parse().expect("Неверный номер порта");
    let peer_ports: Vec<u16> = args[2..]
        .iter()
        .map(|s| s.parse().expect("Неверный номер порта для пира"))
        .collect();

    let my_addr = format!("127.0.0.1:{}", my_port);
    let listener = TcpListener::bind(&my_addr)
        .await
        .expect("Не удалось привязать порт");
    println!("Нода запущена и слушает на {}", my_addr);

    // Запускаем задачу для обработки входящих соединений
    tokio::spawn(async move {
        loop {
            let (mut socket, addr) = listener
                .accept()
                .await
                .expect("Не удалось принять соединение");
            println!("Принято соединение от {}", addr);

            tokio::spawn(async move {
                let mut buf = vec![0u8; 1024];
                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(n) if n == 0 => break, // соединение закрыто
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("Ошибка чтения: {:?}", e);
                            break;
                        }
                    };

                    let msg = String::from_utf8_lossy(&buf[..n]);
                    println!("Получено сообщение от {}: {}", addr, msg);

                    // Отправляем ответ (echo)
                    if let Err(e) = socket.write_all("Сообщение получено".as_bytes()).await
                    {
                        eprintln!("Ошибка записи: {:?}", e);
                        break;
                    }
                }
            });
        }
    });

    // Подключаемся к пирам и отправляем им тестовое сообщение
    for peer_port in peer_ports {
        let peer_addr = format!("127.0.0.1:{}", peer_port);
        // Вспомогательная функция для подключения к пиру и отправки сообщения
        if let Err(e) = connect_and_send(peer_addr.clone()).await {
            eprintln!("Ошибка подключения к {}: {}", peer_addr, e);
        }
    }

    // Чтобы программа не завершалась сразу, засыпаем в бесконечном цикле
    loop {
        sleep(Duration::from_secs(10)).await;
    }
}

async fn connect_and_send(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(&addr).await?;
    println!("Подключились к пиру {}", addr);
    stream.write_all("Привет от ноды".as_bytes()).await?;
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);
    println!("Получен ответ от {}: {}", addr, response);
    Ok(())
}
