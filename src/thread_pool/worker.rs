use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

pub struct Worker {
    pub id : usize,
    pub handler : Option<thread::JoinHandle<()>>,
}

pub type Job = Box<dyn FnBox + Send + 'static>;

pub enum Message {
    NewJob(Job),
    Terminate,
}

impl Worker {
    pub fn new(id : usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let handler = thread::spawn(move ||{
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap();
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} got a terminate message", id);
                        break;
                    }
                }
            }
            /*
            while let Ok(msg) = receiver.lock().unwrap().recv() {
                match msg {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job", id);
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} got a terminate message", id);
                        break;
                    }
                }
            }*/
        });

        Worker {
            id,
            handler : Some(handler),
        }
    }
}