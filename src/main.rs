use futures::executor::block_on;
use log::info;
use std::{
    env,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};
use uuid::Uuid;

// Using rustfs with ACID semantics
#[tokio::main]
async fn main() {
    env_logger::init(); // For env_logger

    let args: Vec<String> = env::args().collect();
    let lena = args.len() - 1;
    println!("Number of arguments is {}", lena);

    if lena != 0 && lena != 2 {
        println!("Need 2 args: <upload|download> <filename>");
        return;
    }

    if lena == 2 {
        if args[1] != "upload" && args[1] != "download" {
            println!("First arg is neither upload nor download");
            return;
        }
    }

    // Create a channel for "safe concurrency"
    // mpsc: MPSC in Rust stands for "Multi-Producer, Single-Consumer,"
    // which is a type of channel used for communication between threads,
    // allowing multiple threads to send messages to a single receiving thread.
    // This mechanism helps ensure "safe concurrency" by enabling message passing
    // instead of shared memory access.
    let (tx, rx) = mpsc::channel();

    send(tx); // send data

    receive(rx); // receive data
}

fn send(tx: Sender<String>) {
    // Spawn a thread for blocking operation
    thread::spawn(move || {
        // Simulate a blocking operation (e.g., writing to RustFS)
        // Send data through the channel to comply with and adhere to ACID semantics
        let data = format!("{:?}", Uuid::new_v4());
        info!("->  Sending: {:?}", data);
        tx.send(data).unwrap();
    });
}

fn receive(rx: Receiver<String>) {
    // Async operation
    let async_operation = async {
        // Wait for data from the channel
        let received_data = rx.recv().unwrap();
        // Simulate an async operation (e.g., reading from RustFS)
        info!("<- Received: {:?}", received_data); // info is appropriate
    };
    // Execute the async operation
    block_on(async_operation);
}
