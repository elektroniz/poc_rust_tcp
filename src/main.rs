// @elektroniz -| implementing TCP in RUST
// Versioning: 0.1
// Starting development date: 10/04/2022

// Before reading the code, just consider what
// is TCP.
// TCP allows hosts to send data in such a way
// to garantee no losing data among partners.
// Our main purpose is implementing something
// that works in that way.

use std::io;

fn main() -> io::Result<()> {
    // nic = network interface card.
    // tun0 because is a virtual interface card. tun stay for "tunnel".
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer = [0u8; 1504];
     
    loop {
        let _nbytes = nic.recv(&mut  buffer[..])?;
        let _eth_flags  = u16::from_be_bytes([buffer[0], buffer[1]]);
        let eth_proto   = u16::from_be_bytes([buffer[2], buffer[3]]);

        let ipv4_proto = 0x0800;
        if eth_proto != ipv4_proto { continue; }

        println!("communicating throught ipv4_proto of {}", _nbytes);
        
    }
}
