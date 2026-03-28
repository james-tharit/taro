use std::{
    net::TcpListener,
    thread,
};

use threadpool::ThreadPool;
mod connection;

#[cfg(test)]
mod tests;

// IP and PORT can be set by environment variables, but for simplicity
// we will hardcode them here.
const IP: &str = "127.0.0.1";
const PORT: i32 = 7878;


fn main() {
    let server_address = format!("{IP}:{PORT}");
    let listener = TcpListener::bind(&server_address).unwrap();
    println!("Server start at: {server_address}");

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let current_thread = thread::current().id();
        println!("{:?}", &current_thread);

        pool.execute(|| {
            crate::connection::handle_connection(stream);
        });
    }
}

