use std::io::prelude::*;
use std::net::TcpStream;
use dns_lookup;
use std::str;
fn main(){
    match alert(){
        Ok(_) => {
            println!("success! exiting.");
        }
        Err(e) => {println!("failed with error {}", e);}
    }
}

fn alert() -> std::io::Result<()> {
    let hostname: String;
    match dns_lookup::get_hostname(){
        Ok(hostn) => {hostname = hostn;}
        Err(_) => {hostname = "unknown".to_string();}
    }
    println!("hostname: {:?}", hostname);
    let mut stream = TcpStream::connect("192.168.1.144:5432")?;
    let to_send = hostname.into_bytes();
    stream.write(to_send.as_slice())?;
    let mut data = [0 as u8; 50];
    match stream.read(&mut data){
        Ok(size) => {
           match str::from_utf8(&data[0..size]){
               Ok(string_out) => {
                   println!("Got data: {}", string_out);}
               Err(_) => {println!("fault");}
           }
        }
        Err(_) => {println!("Fault when reading data!");}
    }
    Ok(())
}
