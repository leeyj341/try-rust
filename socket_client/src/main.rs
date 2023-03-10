use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use std::error::Error;
use std::io;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:30000").await?;
    let mut stream2 = TcpStream::connect("127.0.0.1:30000").await?;

    println!("connect stream");

    loop {
        tokio::spawn(async move {

        });

        let mut read_line = String::new();
        let _result = match io::stdin().read_line(&mut read_line){
            Ok(n) => {
                println!("{} bytes read from input", n);
                let stream_result = stream.write(read_line.as_bytes()).await;
                let stream_result2 = stream2.write(read_line.as_bytes()).await;
                println!("stream success={:?}", stream_result.is_ok());
                println!("stream2 success={:?}", stream_result2.is_ok());
            },
            Err(error) => eprintln!("{}", error)
        };
        let mut line = [];
        let _result = stream.read(&mut line).await;
        println!("from server: {:?}", std::str::from_utf8(&line));
    }
    // let result = stream.write(b"hello world\n").await;
    // Ok(())
}
