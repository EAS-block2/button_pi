use std::io::prelude::*;
use std::net::TcpStream;

fn main(){
    match alert(){
        Ok(_) => {
            println!("success! exiting.");
        }
        Err(_) => {println!("failed.");}
    }
}

fn alert() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("192.168.1.144:5432")?;
    stream.write(b"Hello World")?;
    /*let a = stream.read(&mut [0; 128])?;
    println!("{:?}", a);*/
    Ok(())
}
