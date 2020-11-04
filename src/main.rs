use std::io::prelude::*;
use std::net::TcpStream;
use dns_lookup;
use serde_yaml;
use serde::Deserialize;
use std::{thread, time, str};
use gpio_cdev::{Chip, LineRequestFlags};
fn main(){
    println!("starting");
    init();
    loop{
        println!("heartbeat");
        thread::sleep(time::Duration::from_secs(25));
    }
}
fn init() {
    let conf_f = std::fs::File::open("/home/pi/config.yaml").expect("can't find config");
    let config: Config = serde_yaml::from_reader(conf_f).expect("Bad YAML config file!");
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    if config.alm_general{
    match chip.get_line(15).unwrap().request(LineRequestFlags::INPUT, 1, "Button Switch pin"){
        Ok(g_in) => {
            let mut gen_alm = Alarm{address: (&config.server_addr).to_string(), ports: config.general_ports, input: g_in, output: 21, pressed: false};
            thread::spawn(move || loop{
                gen_alm.run();
                thread::sleep(time::Duration::from_millis(100))});
        }
        Err(e) => {println!("General Alm pin registration failure: {}",e)}
    }}
    if config.alm_silent{
    match chip.get_line(23).unwrap().request(LineRequestFlags::INPUT, 1, "Button Switch pin"){
    Ok(s_in) => {
        let mut sil_alm = Alarm{address: (&config.server_addr).to_string(), ports: config.silent_ports, input: s_in, output: 12, pressed: false};
        thread::spawn(move || loop{
            sil_alm.run();
            thread::sleep(time::Duration::from_millis(100))});
    }
    Err(e) => {println!("Silent Alm pin registration failure: {}",e)}
}
}}

#[derive(Deserialize)]
struct Config{
    server_addr: String,
    general_ports: Vec<u32>,
    silent_ports: Vec<u32>,
    alm_general: bool,
    alm_silent: bool,
}
struct Alarm{
    input: gpio_cdev::LineHandle,
    output: u32,
    address: String,
    ports: Vec<u32>,
    pressed: bool,
}
impl Alarm{
    fn run(&mut self){
        match self.input.get_value().unwrap(){
            1 => {self.pressed = false;},
            0 => { //a button press pulls the pin low
                if !self.pressed{
                println!("Button Pressed");
                self.pressed = true;
                match self.alert(&self.ports[0]){
                    Ok(_) => {
                        println!("success!");
                        match self.success_flash(){Ok(_)=>(),Err(_)=>{println!("already flashing");}}
                    }
                    Err(e) => {println!("ERROR: {}",e);
                self.on_failure();}
        }}}
        _ => {println!("Got improper input");}}
    }
    fn alert(&self, port: &u32) -> std::io::Result<()> {
        let hostname: String;
        match dns_lookup::get_hostname(){
            Ok(hostn) => {hostname = hostn;}
            Err(_) => {hostname = "unknown".to_string();}
        }
        let mut addr = self.address.clone();
        addr.push_str(&port.to_string());
        let mut stream = TcpStream::connect(addr)?;
        let to_send = hostname.into_bytes();
        stream.write(to_send.as_slice())?;
        let mut data = [0 as u8; 50];
        let size = stream.read(&mut data)?;
        match str::from_utf8(&data[0..size]){
        Ok(string_out) => {match string_out{
            "ok" => {return Ok(());}
            _ => {self.on_failure();}
        }}
        Err(_)=>{self.on_failure();}}

        Ok(())
    }
    
    fn on_failure(&self){
        let mut counter = 0;
        'outer: loop{
            println!("in failure mode");
            counter +=1;
            for i in &self.ports{
        match self.alert(i){
            Ok(_) => {println!("finally got connection.");
            match self.success_flash(){Ok(_)=>(),Err(_)=>{println!("already flashing");}}
                break 'outer;}
            Err(_) => {
                if counter > 10{panic!("cannot connect to server");}
                else{thread::sleep(time::Duration::from_secs(30));}
            }}}}}

    fn success_flash(&self) -> gpio_cdev::errors::Result<()>{
        let mut value = false;
        let mut counter = 0;
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let output = chip.get_line(self.output)?.request(LineRequestFlags::OUTPUT, 1, "Button Light pin")?;
        thread::spawn(move || 
            loop {
            println!("flashing");
            counter +=1;
            match output.set_value(value as u8){ Ok(_)=>(), Err(_) => break}
            thread::sleep(time::Duration::from_millis(250));
            value = !value;
            if counter > 180 {break;}
    });
    Ok(())
    }
}