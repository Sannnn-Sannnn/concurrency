use std::net::{ SocketAddr, TcpListener };

mod handler;
mod math;
mod http;
mod threadpool;
mod worker;

use crate::threadpool::ThreadPool;

const THREADPOOL_SIZE: usize = 4;

pub fn main() {
  let address = SocketAddr::from(([127, 0, 0, 1], 3030));
  let listener = TcpListener::bind(address).expect("Error while trying to bind port");
  let pool = ThreadPool::new(THREADPOOL_SIZE);

  println!("Server started in address {}", address);

  for stream in listener.incoming() {
    match stream {
      Ok(s) => {
        pool.execute(move || {
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