use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Instant;

use crate::http;
use crate::math;

pub fn handle(mut stream: TcpStream) {
  let mut buffer = [0; 1024];

  if let Ok(_) = stream.read(&mut buffer) {
    let req_processed = http::get_i_value(&buffer);
    match req_processed {
      Ok(i) => {
        println!("Received: {i}");
        let start_time = Instant::now();
        let result = math::leibniz(i);
        let total_time = start_time.elapsed();

        let body = format!(
          "Valor de Pi para el termino {}: {} (Tiempo: {}s)",
          i, result, total_time.as_secs_f64()
        );

        let response = format!(
          "HTTP/1.1 200 OK\r\n\
        Content-Type: text/plain; charset=utf-8\r\n\
        Content-Length: {}\r\n\
        \r\n\
        {}",
          body.len(),
          body
        );
        if let Err(e) = stream.write_all(response.as_bytes()) {
          eprintln!("Error while sending response: {}", e);
        }
      }
      Err(e) => {
        println!("Error: {e}");
        let respuesta = format!("HTTP/1.1 400 Bad Request\r\n\r\nError: {}", e);
        let _ = stream.write_all(respuesta.as_bytes());
      }
    }
  }
}