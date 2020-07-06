use std::io::prelude::*;
use std::net::TcpStream;
use dns_lookup;
use std::{thread, time, str};
use gpio::{GpioIn, GpioOut};
use crossbeam_channel::bounded;
fn main(){
    let mut button = gpio::sysfs::SysFsGpioInput::open(15).unwrap();
    let mut flashing = false;
    let mut pressed = false;
    let (tx, rx) = bounded(0);
    end_err_flash(tx.clone(), &flashing);
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
                    end_err_flash(tx.clone(), &flashing);
                    flashing = false;
                }
                Err(e) => {
                    if !flashing {start_err_flash(rx.clone());
                    flashing = true;}
                println!("flashing due to {}",e);}
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
fn end_err_flash(comms: crossbeam_channel::Sender<u8>, idk:& bool){
    let mut light = gpio::sysfs::SysFsGpioOutput::open(21).unwrap();
    light.set_value(false).unwrap();
    if *idk{
        comms.send(1).unwrap();
    }
}
fn start_err_flash(comms: crossbeam_channel::Receiver<u8>){
    let mut value = false;
    let mut light = gpio::sysfs::SysFsGpioOutput::open(21).unwrap();
    thread::spawn(move || loop {
        light.set_value(value).unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        value = !value;
        match comms.try_recv() {
            Ok(got) => {if got == 1{break;}}
            Err(_) => ()
        }
});
}