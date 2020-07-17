use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;
use dns_lookup;
use serde_yaml;
use serde::Deserialize;
use std::{thread, time, str};
use gpio_cdev::{Chip, LineRequestFlags};
fn main(){
    let conf_f = std::fs::File::open("/home/pi/config.yaml").expect("can't find config");
    let config: Config = serde_yaml::from_reader(conf_f).expect("Bad YAML config file!");
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let button = chip.get_line(15).unwrap().request(LineRequestFlags::INPUT, 1, "Button Switch pin").unwrap();
    let mut pressed = false;
    let mut server_address: String = config.server_addr;
    match &config.alarm_mode{
        1 => server_address.push_str(":5432"),
        2 => server_address.push_str(":5433"),
        _ => panic!("Unrecognized Alarm Mode!"),
    }
    loop{
    thread::sleep(time::Duration::from_millis(100));
    match button.get_value().unwrap(){
        1 => {pressed = false;},
        0 => { //a button press actually pulls the pin low
            if !pressed{
            pressed = true;
            match alert(&server_address){
                Ok(_) => {
                    println!("success!");
                    match success_flash(){Ok(_)=>(),Err(_)=>{println!("already flashing");}}
                }
                Err(e) => {println!("ERROR: {}",e);
            on_failure(&server_address);}
    }}}
    _ => {println!("Got other input");}}}}

fn alert(server_address: &String) -> std::io::Result<()> {
    let hostname: String;
    match dns_lookup::get_hostname(){
        Ok(hostn) => {hostname = hostn;}
        Err(_) => {hostname = "unknown".to_string();}
    }
    let mut stream = TcpStream::connect(server_address)?;
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
fn success_flash() -> gpio_cdev::errors::Result<()>{
    let mut value = false;
    let mut counter = 0;
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let light = chip.get_line(21)?.request(LineRequestFlags::OUTPUT, 1, "Button Light pin")?;
    thread::spawn(move || loop {
        counter +=1;
        match light.set_value(value as u8){ Ok(_)=>(), Err(_) => break}
        thread::sleep(time::Duration::from_millis(250));
        value = !value;
        if counter > 480 {break;}
});
Ok(())
}
fn on_failure(server_address: &String){
    let mut counter = 0;
    loop{
        println!("in failure mode");
        counter +=1;
    match alert(server_address){
        Ok(_) => {println!("finally got connection.");
        match success_flash(){Ok(_)=>(),Err(_)=>{println!("already flashing");}}
            break;}
        Err(_) => {
            if counter > 10{Command::new("reboot");}
            else{thread::sleep(time::Duration::from_secs(30));}
        }}}}
#[derive(Deserialize)]
struct Config{
    server_addr: String,
    alarm_mode: u8,
}