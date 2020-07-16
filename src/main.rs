use std::io::prelude::*;
use std::net::TcpStream;
use std::process::Command;
use dns_lookup;
use std::{thread, time, str};
use gpio_cdev::{Chip, LineRequestFlags, EventRequestFlags, EventType};
fn main(){
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let button = chip.get_line(15).unwrap();
    let mut pressed = false;

spawn_button().unwrap();
loop{
    thread::sleep(time::Duration::from_secs(3));
}
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
               Ok(string_out) => {println!("Got data: {}", string_out);}
               Err(_) => {println!("fault");}}
            }
        Err(_) => {println!("Fault when reading data!");}}
    Ok(())
}
/*fn success_flash(){
    let mut value = false;
    let mut light: gpio::sysfs::SysFsGpioOutput;
    match gpio::sysfs::SysFsGpioOutput::open(21){
        Ok(e)=> {light = e;}
        Err(_)=> return,
    }
    let mut counter = 0;
    thread::spawn(move || loop {
        counter +=1;
        match light.set_value(value){ Ok(_)=>(), Err(_) => break}
        thread::sleep(time::Duration::from_millis(250));
        value = !value;
        if counter > 480 {break;}
});
}*/
fn on_failure(){
    let mut counter = 0;
    loop{
        println!("in failure mode");
        counter +=1;
    match alert(){
        Ok(_) => {println!("finally got connection.");
            //success_flash();
            break;}
        Err(_) => {
            if counter > 10{Command::new("reboot");}
            else{thread::sleep(time::Duration::from_secs(30));}
        }}}}
fn spawn_button()-> gpio_cdev::errors::Result<()> {
    let mut chip = Chip::new("/dev/gpiochip0")?;
    let input = chip.get_line(15)?;
    thread::spawn(move || loop {
        for event in input.events(LineRequestFlags::INPUT,EventRequestFlags::BOTH_EDGES,"Button Listen Thread").unwrap(){
            let evt = event.unwrap();
            match evt.event_type() {
                EventType::RisingEdge => {
                    println!("rising edge");
                }
                EventType::FallingEdge => {
                    println!("rising edge");
                }
        }
}});
Ok(())
}