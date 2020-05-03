//! An UDP echo server that just sends back everything that it receives.
//!
//! If you're on Unix you can test this out by in one terminal executing:
//!
//!     cargo run --example echo-udp
//!
//! and in another terminal you can run:
//!
//!     cargo run --example connect -- --udp 127.0.0.1:8080
//!
//! Each line you type in to the `nc` terminal should be echo'd back to you!

#![warn(rust_2018_idioms)]

use tokio;
use tokio::net::UdpSocket;

use futures::future;
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};

struct Server {
    socket: UdpSocket,
    up_socket: UdpSocket,
    to_send: Option<(usize, SocketAddr)>,
    up_to_send: Option<(usize, SocketAddr)>,
    hm: HashMap<String, SocketAddr>,
    remote_addr: SocketAddr,
}

impl Server {
    async fn run(&mut self) -> Result<(), io::Error> {
        println!("in run");

        // let mut hm = HashMap::new();
        let mut buf: Vec<u8> = vec![0; 65_507];

        loop {
            if let Some((size, peer)) = self.to_send {
                //forward client request to up address
                println!("size {} peer {}", size, peer);
                let amt = self
                    .up_socket
                    .send_to(&buf[..size], &self.remote_addr)
                    .await?;
                // println!("send to up addr {}/{} bytes to {}", amt, size, peer);
                //store the local address in hashmap
                let s: String = String::from_utf8_lossy(&buf[..2]).to_string();
                self.hm.insert(s, peer);
            }

            self.to_send = Some(self.socket.recv_from(&mut buf).await?);
        }
    }

    async fn run_remote(&mut self) -> Result<(), io::Error> {
        println!("in run remote");

        let mut buf: Vec<u8> = vec![0; 65_507];
        loop {
            if let Some((size, peer)) = self.up_to_send {
                let s: String = String::from_utf8_lossy(&buf[..2]).to_string();
                if let Some(local_addr) = self.hm.get(&s) {
                    //send back response from up address to client
                    let amt = self.socket.send_to(&buf[..size], &local_addr).await?;
                    println!("send to client {}/{} bytes to {}", amt, size, peer);
                } else {
                    println!("ghost addr {}", s)
                }
            }

            self.up_to_send = Some(self.up_socket.recv_from(&mut buf).await?);
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

    let up_addr: SocketAddr = "104.238.131.160:60053".parse()?;

    let hm = HashMap::new();
    let mut server = Server {
        socket,
        to_send: None,
        up_socket,
        hm,
        up_to_send: None,
        remote_addr: up_addr,
    };

    // let _run1 = server.run();
    // let shared = Rc::new(RefCell::new(server));
    // let mut _run2 = shared.borrow_mut().run_remote();
    // let mut _run1 = shared.borrow_mut();
    // let mut _run2 = shared.borrow_mut();
    // _run1.run()
    match future::try_join(server.run(), server.run()).await {
        Err(e) => println!("an error occurred; error = {:?}", e),
        _ => println!("done!"),
    }

    Ok(())
}
