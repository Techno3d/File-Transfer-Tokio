use bytes::BytesMut;
use tokio::{net::TcpListener, fs::File};
use tokio::io::{AsyncReadExt, BufReader, AsyncWriteExt, AsyncBufReadExt};
use file_transfer_tokio::{Message, Status};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("192.168.0.34:8080").await.unwrap();
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let (read, mut writer) = socket.split();
        let mut reader = BufReader::new(read);

        // let mut data = BytesMut::new();
        let mut data = BytesMut::new();
        let mut message = String::new();


        match reader.read_line(&mut message).await {
            Ok(bytes) => println!("bytes read: {}", bytes),
            Err(e) => {
                eprintln!("Malformed Responce: {}\n Skipping Request", e);
                writer.write(&[2u8]).await.unwrap();
                continue;
            },
        };

        println!("{:?}", message);

        let message = serde_json::from_str::<Message>(&message.trim()).unwrap();

        let mut file = File::create(message.file_name).await.unwrap();

        
        writer.write(&[1u8]).await.unwrap();

        match message.status {
            Status::Single => {    //Files under 64 bytes 
                let bytes = reader.read_buf(&mut data).await.unwrap();
                println!("{:?}, {}", data, bytes);
            
                file.write_all(&mut data).await.unwrap();
            },
            Status::Multiple => { //Files over 64 bytes
                'outer: loop {
                    for _ in 0..message.packets {
                        let mut chunk = [0u8; 64];
                        match reader.read_exact(&mut chunk).await {
                            Ok(_) => {},
                            Err(err) => {
                                eprintln!("{}", err);
                                writer.write_all(&[3u8]).await.unwrap();
                                continue 'outer;
                            },
                        };
                        data.extend_from_slice(&chunk);
                    }
                    writer.write_all(&[1u8]).await.unwrap();
                    break;
                }

                file.write_all(&mut data).await.unwrap();
            },
            Status::Ready => {},
        }
    }
}
