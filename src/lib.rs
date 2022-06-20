use std::fmt::Display;
use std::{env, process};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Status {
    Single, Multiple, Ready
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub status: Status,
    pub packets: u64,
    pub file_name: String
}


#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub place: u16,
    pub content: Vec<u8>
}

#[derive(Debug)]
pub struct Arguments {
    pub file_name: String,
    pub output_name: String,
    pub server_addr: String
}

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File Name: {}\nServer Address: {}\nOutput File's Name: {}", self.file_name, self.server_addr, self.output_name)
    }
}

impl Arguments {
    pub fn new() -> Arguments {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            eprintln!("Specify a file to transfer, and server");
            process::exit(1);
        } else if args.len() == 3 {
            if args.get(1).unwrap() == "--help" || args.get(1).unwrap() == "-h" { 
                println!("Must have a server open at 192.168.0.34:8080\n\n\n1st option: the file to transfer\n2nd option: server address\n3nd option (optional): name of the file (do not include file extention)");
                process::exit(0);
            } else {
                let file_name = args.get(1).unwrap();
                let (_, ext) = file_name.split_at(file_name.find(".").unwrap());
                Arguments {
                    file_name: args.get(1).unwrap().to_string(),
                    output_name: format!("output{}", ext.trim()).to_string(),
                    server_addr: args.get(2).unwrap().to_owned()
            }
            }

        } else {
            let file_name = args.get(1).unwrap();
            let (_, ext) = file_name.split_at(file_name.find(".").unwrap());
            Arguments {
                file_name: args.get(1).unwrap().to_string(),
                output_name: format!("{}{}", args.get(3).unwrap().to_string(), ext.trim()),
                server_addr: args.get(2).unwrap().to_owned()
            }
        }
    }
}