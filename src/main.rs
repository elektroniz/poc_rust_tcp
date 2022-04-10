use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buffer = [0u8; 1504];

    loop {
        let _nbytes = nic.recv(&mut  buffer[..])?;
        let _eth_flags  = u16::from_be_bytes([buffer[0], buffer[1]]);
        let eth_proto   = u16::from_be_bytes([buffer[2], buffer[3]]);

        let ipv4_proto = 0x0800;
        if eth_proto != ipv4_proto { continue; }

        match etherparse::Ipv4HeaderSlice::from_slice(&buffer[4.._nbytes]) {
            Ok(packet) =>  {
 
                println!("{:?} -> {:?}: - Read {} bytes (proto: {:x})",
                    packet.source_addr();
                    packet.destination_addr();
                    packet.payload_len(),
                    packet.protocol()
                );
                let source_address      = packet.source_addr();
                let destination_address = packet.destination_addr();

            }
            Err(e) => {
                println!("Warning: weird packet {:?}", e)
            }
        }
    }
}
