use macaddr::MacAddr;
use std::net::{IpAddr, SocketAddr, UdpSocket};

pub fn send_magic_packet_to(mac_addr: MacAddr) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let connect_to = SocketAddr::new(IpAddr::V4([255, 255, 255, 255].into()), 9);
    let mut magic_packet = vec![0xFFu8; 6];

    for _ in 0..16 {
        magic_packet.extend_from_slice(&mac_addr.as_bytes());
    }

    println!("{:?}", magic_packet);

    socket.send_to(magic_packet.as_slice(), connect_to).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use macaddr::MacAddr;

    #[test]
    fn send_packet() {
        send_magic_packet_to(MacAddr::from([0x00, 0x00, 0x5E, 0x00, 0x53, 0x00]));
    }
}
