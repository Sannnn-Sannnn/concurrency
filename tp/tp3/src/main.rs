use std::net::{SocketAddr, TcpListener};
use std::thread;

mod handler;
mod math;
mod http;

pub fn main() {
  let address = SocketAddr::from(([127, 0, 0, 1], 3030));
  let listener = TcpListener::bind(address).expect("Error while trying to bind port");

  println!("Server started in {}", address);

  for stream in listener.incoming() {
    match stream {
      Ok(s) => {
        thread::spawn(move || {
          handler::handle(s);
        });
      }
      // Catch error
      Err(e) => {
        println!("Error: {}", e);
      }
    }
  }
}