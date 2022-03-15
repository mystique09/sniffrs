use std::{net::IpAddr, str::FromStr};

#[derive(Debug)]
pub struct Argument {
    pub flag: String,
    pub ip: IpAddr,
    pub threads: u16,
}

impl Argument {
    pub fn new(args: Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.");
        }

        if args.len() > 4 {
            return Err("Too many arguments.");
        }
        let flag = args.get(1).unwrap();

        if let Ok(ipaddr) = IpAddr::from_str(flag) {
            let arg = Self {
                flag: "".to_string(),
                ip: ipaddr,
                threads: 4,
            };
            Ok(arg)
        } else {
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!(
                    "Usage: -h --help to display this message. \r\n -t to select how many threads."
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments.");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(args.get(3).unwrap()) {
                    Ok(ip) => ip,
                    Err(_) => return Err("Invalid IP_ADDRESS, not an IPV6 or IPV4."),
                };

                let threads = match args.get(2).unwrap().parse::<u16>() {
                    Ok(n) => n,
                    Err(_) => return Err("failed to parse number of threads."),
                };

                let arg = Self {
                    flag: flag.clone().to_string(),
                    ip: ipaddr,
                    threads,
                };
                return Ok(arg);
            } else {
                return Err("invalid input.");
            }
        }
    }
}
