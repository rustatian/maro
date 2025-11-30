use tokio::net::{TcpListener, ToSocketAddrs};

async fn run_server<T: ToSocketAddrs>(addr: T) -> std::io::Result<()> {
	let listener = TcpListener::bind(addr).await?;
	println!("Listening on {}", listener.local_addr()?);

	loop {
		let (stream, peer) = listener.accept().await?;
		println!("Accepted connection from {}", peer);
		tokio::spawn(async move {
			// Handle the connection
		});
	}
	Ok(())
}