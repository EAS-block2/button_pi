use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.1.144:5432")?;

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
}
