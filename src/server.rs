use std::io::Read;
use std::net::{TcpListener, TcpStream};
pub mod header;
use header::NetworkProtocolHeader;

fn receive_header_over_tcp(mut stream: &TcpStream) -> Result<NetworkProtocolHeader, Box<dyn std::error::Error>> {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let mut header = NetworkProtocolHeader::default();
    header.unpack(&buffer[0..bytes_read])?;
    Ok(header)
}

fn main() -> std::io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:8080")?;

    loop {
        for stream in server.incoming() {
            let mut stream = stream.unwrap();
            let received_header = receive_header_over_tcp(&mut stream).unwrap();
            println!("Received header: {:?}", received_header);
            break;
        }
    }
    //Ok(())
}

