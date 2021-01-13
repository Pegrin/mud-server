use std::error::Error;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);
    loop {
        let (mut socket, address) = listener.accept().await?;
        println!("New address connected: {}", &address);
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            socket.write_all(format!(
                "Welcome visitor! \n\
                You connected from: {} \n",
                address.ip()
            ).as_bytes())
                .await
                .unwrap();

            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                socket.write_all("Echo says: ".as_bytes())
                    .await
                    .expect("failed to write data to socket");
                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }
}
