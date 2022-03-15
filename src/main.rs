use std::io::Write;
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::Sender;
use std::{env, process, sync::mpsc::channel};
use std::{io, thread};

use sniff_rs::argument;

const MAX_PORTS: u16 = 65535;

fn main() {
    let args: Vec<String> = env::args().collect();

    let new_arg = argument::Argument::new(args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("problem parsing arguments: {}", err);
            process::exit(0);
        }
    });

    let num_threads = new_arg.threads;
    let (sndr, rcvr) = channel();

    for i in 0..num_threads {
        let sndr = sndr.clone();

        thread::spawn(move || {
            sniff(sndr, i, new_arg.ip, num_threads);
        });
    }

    let mut results: Vec<u16> = vec![];
    drop(sndr);

    for p in rcvr {
        results.push(p);
    }
    results.sort();

    println!("");
    for v in results {
        println!("port {} is open.", v);
    }
}

fn sniff(tx: Sender<u16>, init_port: u16, ip: IpAddr, nt: u16) {
    let mut port = init_port + 1;

    loop {
        match TcpStream::connect((ip, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_PORTS - port) <= nt {
            break;
        }
        port += nt;
    }
}
