use std::io::prelude::*;
use std::net::TcpStream;
use dns_lookup;
use std::str;
use std::{thread, time};
use gpio::{GpioIn, GpioOut};
fn main(){
    let mut button = gpio::sysfs::SysFsGpioInput::open(15).unwrap();
    loop{
    match button.read_value().unwrap(){
        gpio::GpioValue::Low => {println!("button off")}
        gpio::GpioValue::High => {
            match alert(){
                Ok(_) => {
                    println!("success! exiting.");
                }
                Err(_) => {start_err_flash();}
    }}}}
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

fn start_err_flash(){
    let mut value = false;
    let mut light = gpio::sysfs::SysFsGpioOutput::open(21).unwrap();
    thread::spawn(move || loop {
        light.set_value(value).unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
});
}