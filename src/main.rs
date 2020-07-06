use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;
use dns_lookup;
use std::{thread, time, str};
use gpio::{GpioIn, GpioOut};
fn main(){
    let mut button = gpio::sysfs::SysFsGpioInput::open(15).unwrap();
    let mut pressed = false;
    println!("GPIO23: {:?}", button.read_value().unwrap());
    gpio::sysfs::SysFsGpioOutput::open(21).unwrap().set_value(false).unwrap();
    loop{
    thread::sleep(time::Duration::from_millis(100));
    match button.read_value().unwrap(){
        gpio::GpioValue::High => {pressed = false;},
        gpio::GpioValue::Low => { //a button press actually pulls the pin low
            if !pressed{
            pressed = true;
            match alert(){
                Ok(_) => {
                    println!("success!");
                    success_flash();
                }
                Err(e) => {println!("ERROR: {}",e);
            on_failure();}
    }}}}}
}

fn alert() -> std::io::Result<()> {
    let hostname: String;
    match dns_lookup::get_hostname(){
        Ok(hostn) => {hostname = hostn;}
        Err(_) => {hostname = "unknown".to_string();}
    }
    println!("hostname: {:?}", hostname);
    let mut stream = TcpStream::connect("192.168.1.162:5432")?;
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
fn success_flash(){
    let mut value = false;
    let mut light = gpio::sysfs::SysFsGpioOutput::open(21).unwrap();
    let mut counter = 0;
    thread::spawn(move || loop {
        counter +=1;
        light.set_value(value).unwrap();
        thread::sleep(time::Duration::from_millis(250));
        value = !value;
        if counter > 480 {break;}
});
}
fn on_failure(){
    let mut counter = 0;
    loop{
        println!("in failure mode");
        counter +=1;
    match alert(){
        Ok(_) => {
            println!("finally got connection.");
            success_flash();
            break;
        }
        Err(_) => {
            if counter > 10{
                Command::new("reboot");
            }
            else{thread::sleep(time::Duration::from_secs(30));}
        }
}}}