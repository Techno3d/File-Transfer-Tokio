use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpSocket;
use tokio_files::{Message, Arguments};

use std::{env, process};

#[tokio::main]
async fn main() {
    let args = args();
    println!("{}", args);

    let socket = TcpSocket::new_v4().unwrap(); 
    let stream = match socket.connect(args.server_addr.parse().unwrap()).await {
        Ok(stream) => {
            stream
        },
        Err(_) => panic!("No server open at {}", args.server_addr),
    };

    let (mut reader, mut writer) = stream.into_split();

    let path = Path::new(&args.file_name);
    let mut file = File::open(path).await.unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.unwrap();

    if contents.len() > 64 {  //If The size is greater than 1000 bytes we break it into smaller chunks
        let content_len: u64 = contents.len().try_into().unwrap();
        let message = Message {
            status: tokio_files::Status::Multiple,
            packets: content_len/64,
            file_name: args.output_name
        };

        writer.write_all((serde_json::to_string(&message).unwrap() + "\n").as_bytes()).await.unwrap();

        let mut responce = [0u8; 1];
        reader.read_exact(&mut responce).await.unwrap();
        if responce[0] == 2 {
            eprintln!("server couldn't accept responce");
            process::exit(1);
        }
        
        let writerr = Rc::new(RefCell::new(writer));

        loop {
            let iter = contents.chunks(contents.len()/64);
            for content in iter {
                let writerr = Rc::clone(&writerr);
                writerr.borrow_mut().write_all(&content).await.unwrap();
            }     
    
            reader.read_exact(&mut responce).await.unwrap();
            if responce[0] == 3u8 {
                eprintln!("Couldn't Send File Properly");
            } else {
                break;
            }
        }


    } else {  //Else we just send in one packet
        let message = Message {
            status: tokio_files::Status::Single,
            packets: 1,
            file_name: args.file_name
        };
        writer.write_all((serde_json::to_string(&message).unwrap() + "\n").as_bytes()).await.unwrap();
        serde_json::to_string(&message).unwrap().as_bytes().len();
        

        let mut responce = [0u8; 1];
        reader.read_exact(&mut responce).await.unwrap();

        if responce[0] == 2 {
            eprintln!("server couldn't accept responce");
            process::exit(1);
        }

        reader.read_exact(&mut responce).await.unwrap();
        if responce[0] == 3u8 {
            eprintln!("Couldn't Send File Properly");
        }
    }
}

fn args() -> Arguments {
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