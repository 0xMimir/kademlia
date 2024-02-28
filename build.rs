use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Error binding socket");
    socket.connect("8.8.8.8:80").expect("Error connecting");
    let ip = socket
        .local_addr()
        .expect("Error getting local address")
        .ip()
        .to_string();
    println!("cargo:rustc-env=IP_ADDR={}", ip);
}
