mod packet;

use packet::RawPacket;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::{Buf, BufMut};

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

        let mut buf = bytes::Bytes::from(buf);
        let id = buf.get_varint().unwrap();
        match state {
            0 => {
                let protocol = buf.get_varint().unwrap();
                let server = buf.get_string().unwrap();
                let port = buf.get_u16();
                let next_state = buf.get_u8();
                println!("New handshake, protocol: {}, server: {}, port: {}", protocol, server, port);
                state = next_state;
            },
            1 => {
                match id {
                    0 => {
                        let res = "{\"version\":{\"name\":\"1.8.9\",\"protocol\":47},\"players\":{\"max\":100,\"online\":0,\"sample\":[]},\"description\":{\"text\":\"Hello\"}}";
                        let len = res.len() as u8;
                        socket.write_u8(len + 2).await?;
                        socket.write_u8(0x00).await?;
                        socket.write_u8(len).await?;
                        socket.write_all(res.as_bytes()).await?;
                    },
                    1 => {
                        let mut res = bytes::BytesMut::with_capacity(10);
                        res.put_u8(9);
                        res.put_u8(0x01);
                        res.put_u64(buf.get_u64());
                        socket.write_all(&res).await?;
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }
}

#[tokio::main]
pub async fn main() -> tokio::io::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move { handle_socket(socket).await });
    }
}
