use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io;
use tokio::net::TcpStream;

enum Request {}

enum Response {}

pub async fn send<T: Serialize>(stream: &mut TcpStream, msg: &T) -> io::Result<()> {
    Ok(())
}

pub async fn recv<T: DeserializeOwned>(stream: &mut TcpStream) -> io::Result<T> {
    let mut buf = vec![0u8, 10];
    serde_json::from_slice(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}