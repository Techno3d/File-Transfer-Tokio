use std::fmt::Display;

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