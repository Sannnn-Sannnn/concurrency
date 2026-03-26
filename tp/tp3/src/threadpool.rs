use std::sync::{mpsc, Arc, Mutex};

use crate::worker::{ Worker, Job };

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    // 1. Creamos el canal (MPSC: Multiple Producer, Single Consumer)
    let (sender, receiver) = mpsc::channel();

    // 2. Envolvemos el receptor para que sea compartido y seguro entre hilos
    // Arc: Permite múltiples dueños (Referencia Atómica)
    // Mutex: Garantiza que solo un hilo lea del canal a la vez
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      // 3. Creamos cada worker pasándole su ID y una COPIA del Arc
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    ThreadPool { workers, sender }
  }
  pub fn execute<F>(&self, f: F)
  where
    F: FnOnce() + Send + 'static,
  {
    // 4. Empaquetamos la función en un Box para convertirla en un Job
    let job = Box::new(f);

    // 5. La enviamos por el canal. El primer Worker libre la agarrará.
    self.sender.send(job).unwrap();
  }
}