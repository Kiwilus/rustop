use std::net::UdpSocket;

pub fn get_local_ip() -> String {
    // Wir verbinden uns nicht wirklich, nur um die eigene IP zu ermitteln
    let socket = UdpSocket::bind("0.0.0.0:0").ok();
    
    if let Some(sock) = socket {
        sock.connect("8.8.8.8:80").ok();
        if let Ok(addr) = sock.local_addr() {
            return addr.ip().to_string();
        }
    }
    
    "Unknown".to_string()
}