mod server;
use server::Server;

fn main() {
	let server = Server::new("127.0.0.1:7878");
	match server {
		Ok(s) => {
			println!("Server started.");
			s.listen();
		},
		Err(e) => println!("Error starting server."),
	}
}
