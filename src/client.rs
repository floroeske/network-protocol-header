use std::io::Write;
use std::net::TcpStream;
use std::thread;

pub mod header;
// use header::NetworkProtocolHeader;

fn send_header_over_tcp(header: &header::NetworkProtocolHeader, mut stream: &TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let packed_header = header.pack();
    stream.write_all(&packed_header)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let handle = thread::spawn(move || {
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        let header = header::NetworkProtocolHeader {
            version: 1,
            message_id: 42,
            payload_size: 12,
            reserved: [0, 0],
            extra_header_size: 0,
            extra_header: vec![0x11, 0x22, 0x33],
        };
        send_header_over_tcp(&header, &mut stream).unwrap();
    });

    handle.join().unwrap();
    Ok(())
}

