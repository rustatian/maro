use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

enum Request {
    Add { a: i32, b: i32 },
    Echo { msg: String },
}

enum Response {
    Value(i64),
    Text(String),
}

pub async fn send<T: Serialize>(stream: &mut TcpStream, msg: &T) -> io::Result<()> {
    let data =
        serde_json::to_vec(msg).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    stream.write_all(&data.len().to_be_bytes()).await?;
    stream.write_all(&data).await?;
    stream.flush().await
}

pub async fn recv<T: DeserializeOwned>(stream: &mut TcpStream) -> io::Result<T> {
    let mut len_buf = [0u8; 8];
    stream.read_exact(&mut len_buf).await?;
    let len: usize = usize::from_be_bytes(len_buf);

    let mut buf: Vec<u8> = vec![0u8; len];
    stream.read_exact(&mut buf).await?;
    serde_json::from_slice(&buf).map_err(|e: serde_json::Error| -> std::io::Error { io::Error::new(io::ErrorKind::InvalidData, e) })
}
