use tokio::{net::{TcpListener}, sync::broadcast, io::{AsyncBufReadExt, AsyncWriteExt}};
use tokio::io::{BufReader};

async fn open_tcp(server_addr: &str) {
    // 소켓 리스너 생성
    let listener = TcpListener::bind(server_addr).await.unwrap();
    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        // 유저가 접속할 때마다 받기
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let msg = result.unwrap();
                        writer.write_all(msg.as_bytes()).await.unwrap();
                        println!("{}: {}", addr.to_string(), msg);
                    }
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    open_tcp("127.0.0.1:30000").await
}
