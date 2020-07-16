use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;
use dns_lookup;
use std::{thread, time, str};
use gpio_cdev::{Chip, LineRequestFlags};
fn main(){
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let button = chip.get_line(15).unwrap().request(LineRequestFlags::INPUT, 1, "Button Switch pin").unwrap();
    let mut pressed = false;
    loop{
    thread::sleep(time::Duration::from_millis(100));
    match button.get_value().unwrap(){
        1 => {pressed = false;},
        0 => { //a button press actually pulls the pin low
            if !pressed{
            pressed = true;
            match alert(){
                Ok(_) => {
                    println!("success!");
                    success_flash();
                }
                Err(e) => {println!("ERROR: {}",e);
            on_failure();}
    }}}
    _ => {println!("Got other input");}}}}

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
               Ok(string_out) => {println!("Got data: {}", string_out);}
               Err(_) => {println!("fault");}}
            }
        Err(_) => {println!("Fault when reading data!");}}
    Ok(())
}
fn success_flash(){
    let mut value = false;
    let mut counter = 0;
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let light = chip.get_line(21).unwrap().request(LineRequestFlags::OUTPUT, 1, "Button Light pin").unwrap();
    thread::spawn(move || loop {
        counter +=1;
        match light.set_value(value as u8){ Ok(_)=>(), Err(_) => break}
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
        Ok(_) => {println!("finally got connection.");
            success_flash();
            break;}
        Err(_) => {
            if counter > 10{Command::new("reboot");}
            else{thread::sleep(time::Duration::from_secs(30));}
        }}}}