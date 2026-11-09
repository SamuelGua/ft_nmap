use std::fs::File;
use std::io;
use std::io::BufRead;
use std::net::IpAddr;

pub enum TypeScan { SYN, NULL, ACK, XMAS, UDP }


pub struct Parameters {
    pub ports: Vec<u16>,
    pub ip: Vec<IpAddr>,
    pub speedup: u16,
    pub scan: TypeScan,
}
// [--help] [--ports [Numbers/ranged]] --ip IP_ADDRESS | --file FILE [--speedup] [--scan [type]]
impl Parameters {
    pub fn new_argv<I>(mut args: I) -> Result<Self, String>
    where I: Iterator<Item = String> {
        let mut ports: Vec<u16> = Vec::new();
        let mut ip: Vec<IpAddr> = Vec::new();
        let mut speedup: u16 = 1;
        let mut scan: TypeScan = TypeScan::SYN;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" => {
                    Self::print_helper();
                    std::process::exit(0);
                }
                "--ports" => {
                        let value = args.next().ok_or_else(|| "Option --ports attend une liste de ports en paramètre.".to_string())?;
                        let parsed_ports = Self::setup_ports(&value)?;
                        ports.extend(parsed_ports);
                }
                "--ip" => {
                    if ip.is_empty() == false {
                       return Err("--ip is already set".to_string())
                    }
                    let value = args.next().ok_or_else(|| "Option --ip attend une IP".to_string())?;
                    let parsed_ip = Self::setup_ip(&value)?;
                    ip.push(parsed_ip);
                    println!("{}", parsed_ip);
                }
                "--file" => {
                    if ip.is_empty() == false {
                        return Err("--ip is already set".to_string())
                    }
                    let value = args.next().ok_or_else(|| "Option --scan attend une de ces valeurs".to_string())?;
                    let parsed_file = Self::setup_ip_in_file(&value)?;
                    ip.extend(parsed_file);
                }
                "--speedup" => {
                    let value = args.next().ok_or_else(|| "Option --speedup attend un argument".to_string())?;
                    let parsed_speedup = Self::setup_speedup(&value)?;
                    speedup = parsed_speedup;
                }
                "--scan" => {
                    let value = args.next().ok_or_else(|| "Option --scan attend une de ces valeurs".to_string())?;
                    let parsed_scan = Self::setup_scan(&value)?;
                    scan = parsed_scan;
                }
                _ => {
                    return Err(format!("Incorrect argument: {}", arg))
                }
            }
        }

        if ip.is_empty() {
            return Err("No ip specified".to_string());
        }
        if ports.is_empty() {
            ports.extend(1..=1024)
        } else {
        ports.sort();
        ports.dedup();
        }

        Ok(Parameters { ports, ip, speedup, scan })
    }

    fn setup_scan(scan_arg: &str) -> Result<TypeScan, String> {
         match scan_arg {
            "SYN" => Ok(TypeScan::SYN),
            "NULL" => Ok(TypeScan::NULL),
            "ACK" => Ok(TypeScan::ACK),
            "XMAS" => Ok(TypeScan::XMAS),
            "UDP" => Ok(TypeScan::UDP),
            _ => Err("Wrong parameter format.".to_string()),
        }
    }

    fn setup_ports(args: &str) -> Result<Vec<u16>, String> {
        let mut ports: Vec<u16> = Vec::new();

        let mut parts: Vec<&str> = args.split(',').collect();
        for part in parts.iter_mut() {
            if let Some((start, end)) = part.split_once("-") {

                let min = start.parse::<u16>().map_err(|_| format!("Invalid port number start: {}", start))?;
                let max = end.parse::<u16>().map_err(|_| format!("Invalid port number end: {}", end))?;
                if min > max {
                    return Err(format!("Invalid port number start: {}", start));
                }
                for p in min..=max {
                    ports.push(p);
                }
            } else {
                let port = part.parse::<u16>().map_err(|_| format!("Invalid port: {}", part))?;
                ports.push(port);
            }
        }
        Ok(ports)
    }

    fn setup_ip(args: &str) -> Result<IpAddr, String>  {
        args.parse::<IpAddr>().map_err(|e| e.to_string())
    }

    fn setup_ip_in_file(path: &str) -> Result<Vec<IpAddr>, String> {
        let mut ip: Vec<IpAddr> = Vec::new();
        let file = File::open(path).map_err(|e| format!("error file {}", e))?;
        let lecteur = io::BufReader::new(file);

        for ligne in lecteur.lines() {
            let ligne_text = ligne.unwrap(); // a gerer aussi
            match ligne_text.trim().parse::<IpAddr>() {
                Ok(valid_ip) => ip.push(valid_ip),
                Err(_) =>  println!("Wrong ip: {}", ligne_text),
            }
        }
        Ok(ip)
    }
    
    fn setup_speedup(args: &str) -> Result<u16, String>  {
        args.parse::<u16>().map_err(|e| e.to_string())
    }
    
    fn print_helper() {
        println!("ft_nmap [OPTIONS]\n
            --help Print this help screen\n
            --ports ports to scan (eg: 1-10 or 1,2,3 or 1,5-15)\n
            --ip ip addresses to scan in dot format\n
            --file File name containing IP addresses to scan,\n
            --speedup [250 max] number of parallel threads to use\n
            --scan SYN/NULL/FIN/XMAS/ACK/UDP");
    }
}