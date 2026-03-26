use std::thread;
use std::sync::{mpsc, Arc, Mutex};

// Definimos el alias para el trabajo (el paquete que viaja por el canal)
pub type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
  pub id: usize,
  pub thread: thread::JoinHandle<()>,
}

impl Worker {
  // El constructor del Worker recibe su ID y una copia compartida del Receptor
  pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

    // Aquí es donde sucede la magia: spawneamos el hilo que VIVIRÁ siempre
    let thread = thread::spawn(move || {
      loop {
        // 1. Pedimos el candado del Mutex para escuchar el canal
        // 2. Intentamos recibir (recv) un trabajo
        let job = receiver.lock().unwrap().recv();

        match job {
          Ok(f) => {
            println!("Worker {} received a task; executing.", id);
            // 3. ¡Ejecutamos el closure!
            f();
          }
          Err(_) => {
            // Si el canal se cierra, salimos del loop para que el hilo muera dignamente
            println!("Worker {} disconnecting.", id);
            break;
          }
        }
      }
    });

    Worker { id, thread }
  }
}