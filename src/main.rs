use std::{io::stdin, process::exit};

use serialport::{SerialPortInfo, SerialPortType};

fn main() {
    println!("Hello, world!");

    let mut valid_ports = scan_ports();
    for p in &valid_ports {
        println!("name: {}", p.port_name)
    }
    println!("Choose a port: ");
    let mut chosen_port = String::new();
    stdin()
        .read_line(&mut chosen_port)
        .expect("Did not enter a string");
    let chosen_port = chosen_port.trim();

    valid_ports.retain(|x| x.port_name == chosen_port);

    if valid_ports.is_empty() {
        println!("Port not found!");
        exit(1);
    }
    let chosen_port = &valid_ports[0];

    println!("CHosen port = {:?}", chosen_port);
}

fn scan_ports() -> Vec<SerialPortInfo> {
    let mut ports = serialport::available_ports().expect("No ports found");

    // filter out those with an unknown type
    ports.retain(|x| !matches!(x.port_type, SerialPortType::Unknown));

    ports
}
