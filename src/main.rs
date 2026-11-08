use std::io::BufRead;
// [--help] [--ports [Numbers/ranged]] --ip IP_ADDRESS | --file FILE [--speedup] [--scan [type]]
use std::{env, io};
use std::net::IpAddr;
use std::fs::{File};

enum TypeScan { SYN, NULL, ACK, XMAS, UDP }

struct Ports {min: u16, max: u16}

struct Parameters {
    ports: Ports,
    ip: Vec<IpAddr>,
    speedup: u16,
    scan: TypeScan,
}

impl Parameters {

    fn new(ip: Vec<IpAddr>, ports: Ports, speedup: u16, scan :TypeScan) -> Self {
        Parameters { ports, ip, speedup, scan }
    }

    fn setup_scan(scan_arg :&str) -> Result<TypeScan, i32> {
        match scan_arg {
            "SYN" => return Ok(TypeScan::SYN),
            "NULL" => return Ok(TypeScan::NULL),
            "ACK" => return Ok(TypeScan::ACK),
            "XMAS" => return  Ok(TypeScan::XMAS),
            "UDP" => return Ok(TypeScan::UDP),
            _ => return Err(1)
        }
    }
}

fn main() -> Result<(), String> {

    let mut args = env::args().skip(1);
    let mut ports = Ports { min: 0, max: 0};
    let mut ip: Vec<IpAddr> = Vec::new();
    let mut speedup: u16 = 0;
    let mut scan: TypeScan = TypeScan::NULL;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" => {
            println!("voici le hlep");
            return Ok(())
            }
            "--ports" => {
                if let Some(value) = args.next() {
                    let ip_ranged: Vec<&str> = value.split("-").collect();
                    if ip_ranged.len() > 2 {
                        return Err(format!("Wrong ports value {}", value));
                    }

                    ports.min = ip_ranged.get(0).and_then(|s| s.parse().ok()).unwrap_or(0);
                    ports.max = ip_ranged.get(1).and_then(|s| s.parse().ok()).unwrap_or(ports.min);
                    if ports.min == 0 {
                        return Err(format!("Wrong ports value {}", value))
                    }
                }
            }
            "--ip" => {
                if let Some(value) = args.next() {
                    match value.parse::<IpAddr>() {
                        Ok(valide_ip) => ip.push(valide_ip),
                        Err(_) => return Err(format!("Error"))
                    }
                } else {
                    return Err(format!("No IpAddr"))
                }
            }
            "--file" => {
                if let Some(value) = args.next() {
                    // let file = File::open(value).unwrap(); // a gerer io::result
                    let file = File::open(value).map_err(|e| format!("erro file {}", e))?;
                    let lecteur = io::BufReader::new(file);

                    for ligne in lecteur.lines() {
                        let ligne_text = ligne.unwrap(); // a gerer aussi
                        match ligne_text.trim().parse::<IpAddr>() {
                            Ok(valide_ip) => ip.push(valide_ip),
                            Err(_) =>  println!("Wrong ip: {}", ligne_text),
                        }
                    }
                }
            }
            "--speedup" => {
                if let Some(value) = args.next() {
                    match value.parse::<u16>() {
                        Ok(n) => speedup = n,
                        Err(_) => return Err(format!("Incorrect scan: {}", value))
                    }
                } else {
                    return Err(format!("No speedup numbers"))
                }
            }
            "--scan" => {
                if let Some(value) = args.next() {
                    match Parameters::setup_scan(&value) {
                        Ok(v) => scan = v,
                        Err(_) => return Err(format!("Incorrect scan: {}", value))
                    }
                } else {
                    return Err(format!("No scan type"))
                }
            }
            _ => {
                return Err(format!("Incorrect argument: {}", arg))
            }
        }
    }
    let param = Parameters::new(ip, ports, speedup, scan);
    Ok(())
}


