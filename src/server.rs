use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;

pub struct Server {
	ip: String,
	listener: TcpListener,
}

impl Server {
	pub fn new(ip: &str) -> Result<Server, String>{
		let listener = TcpListener::bind(ip);

		match listener {
			Ok(l) => {
				return Ok(
					Server {
						ip: ip.to_string(),
						listener: l,
					}
				);
			},
			Err(e) => {
				return Err(format!("Unable to bind to ip {} because {}", ip, e));
			}
		}
	}

	pub fn listen(&self) {
		println!("listening");

		for stream in self.listener.incoming() {
			match stream {
				Ok(s) => handle_stream(s),
				Err(e) => println!("Connection error: {}", e),
			}
		}

		println!("done listening");
	}
}

fn handle_stream(mut stream: TcpStream) {
	let mut buffer = [0; 1024];
	stream.read(&mut buffer).unwrap();

	let response = "HTTP/1.1 200 OK\r\n\r\n";
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}
