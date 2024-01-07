mod packet;

use packet::Status;

use crust_protocol::{Deserialize,ser::Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::packet::PingResponse;

#[tokio::main]
pub async fn main() -> tokio::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { handle_socket(socket).await });
    }
}

pub async fn handle_socket(mut socket: tokio::net::TcpStream) -> tokio::io::Result<()> {
    let mut state = 0;
    loop {
        let len = {
            let mut v = 0;
            let mut i = 0;
            loop {
                let byte = socket.read_u8().await?;
                v |= ((byte & 0x7F) << i) as i32;
                if (byte & 0x80) == 0 { break Some(v) }

                i += 1;
                if i > 4 { break None }
            }
        }.unwrap();

        let mut buf = vec![0u8; len as usize];
        socket.read_exact(&mut buf).await?;
        println!("{:?}", buf);

        let mut buf = buf.into_iter();
        let id = buf.read_varint().unwrap();
        match state {
            0 => {
                let protocol = buf.read_varint().unwrap();
                let server = buf.read_string().unwrap();
                let port = buf.read_u16().unwrap();
                let next_state = buf.read_u8().unwrap();
                println!("New handshake, protocol: {}, server: {}, port: {}", protocol, server, port);
                state = next_state;
            },
            1 => {
                match id {
                    0 => {
                        let res = Status {
                            json_response: "{\"version\":{\"name\":\"1.8.9\",\"protocol\":47},\"players\":{\"max\":100,\"online\":0,\"sample\":[]},\"description\":{\"text\":\"Hello\"}}".to_string(),
                        };
                        socket.write_all(&res.serialize()).await?;
                    },
                    1 => {
                        let res = PingResponse { payload: buf.read_i64().unwrap() }.serialize();
                        socket.write_all(&res).await?;
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}
