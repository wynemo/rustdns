#![warn(rust_2018_idioms)]

use tokio;
use tokio::net::UdpSocket;
use tokio::time;

use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::time::Duration;
use std::{env, io};

struct Server {
    socket: UdpSocket,
    up_socket: UdpSocket,
    to_send: Option<(usize, SocketAddr)>,
    hm: HashMap<String, SocketAddr>,
    remote_addr: SocketAddr,
}

impl Server {
    async fn run(&mut self) -> Result<(), io::Error> {
        println!("in run");

        let mut buf: Vec<u8> = vec![0; 1024];
        let mut new_buf: Vec<u8> = vec![0; 65_507];

        loop {
            self.to_send = Some(self.socket.recv_from(&mut buf).await?);
            if let Some((size, peer)) = self.to_send {
                println!("size {} peer {}", size, peer);
                let s: String = String::from_utf8_lossy(&buf[..2]).to_string();
                self.hm.insert(s, peer);
                self.up_socket
                    .send_to(&buf[..size], &self.remote_addr)
                    .await?;
                if let Ok(Ok((up_size, source))) = time::timeout(
                    Duration::from_secs(2),
                    self.up_socket.recv_from(&mut new_buf),
                )
                .await
                {
                    let s: String = String::from_utf8_lossy(&new_buf[..2]).to_string();
                    if let Some(local_addr) = self.hm.get(&s) {
                        println!(
                            "up_size {} local_addr {} source {}",
                            up_size, local_addr, source
                        );
                        self.socket
                            .send_to(&new_buf[..up_size], &local_addr)
                            .await?;
                    } else {
                        println!("ghost addr {}", s)
                    }
                } else {
                    println!("timeout")
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    let local_addr: SocketAddr = "0.0.0.0:0".parse()?;
    let up_socket = UdpSocket::bind(local_addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let up_addr = env::args().nth(2).unwrap_or_else(|| "114.114.114.114:53".to_string());
    let up_addr: SocketAddr = up_addr.parse()?;

    let hm = HashMap::new();
    let mut server = Server {
        socket,
        to_send: None,
        up_socket,
        hm,
        remote_addr: up_addr,
    };

    server.run().await?;

    Ok(())
}
