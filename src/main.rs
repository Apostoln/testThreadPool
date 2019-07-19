use std::env;
use std::net::{TcpListener};

mod server_handler;
use server_handler::Handler;

mod thread_pool;
use thread_pool::ThreadPool;

fn main() {
    let args : Vec<_> = env::args().collect();
    let ip = format!("{}:{}", args[1], args[2]);
    let cores = 4;
    let pool = ThreadPool::new(cores);

    let listener = TcpListener::bind(ip).unwrap();
    for stream in listener.incoming() {
        println!("Accept new connection!");
        pool.execute(|| {
            println!("Start handle");
            Handler::new(stream.unwrap()).handle();
        });
    }
}
