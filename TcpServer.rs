use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};

static GLOBAL_COUNTER: AtomicU64 = AtomicU64::new(0);

fn handle_client(_stream: TcpStream) {
	println!("ID prod: {:?}", GLOBAL_COUNTER);
	GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn main() -> std::io::Result<()> {
	let listener = TcpListener::bind("127.0.0.1 8080")?;
	
	//accepting connections
	for stream in listener.incoming(){
		handle_client(stream?);
	}
	Ok(())
}
