use std::net::SocketAddr;

fn main() {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();

    println!("ip address: {:?}, port: {:?}", addr.ip(), addr.port());
}