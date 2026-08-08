use dotenvy::dotenv;
use futures::executor::block_on;
use log::info;
use std::{
    env, fs,
    io::Write,
    process::{Command, Stdio},
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

    match lena {
        0 => {
            println!(
                "\n{:>12}Number of arguments is 0, so
            just sending sample data to a channel
            and receiving the data from the channel",
                " "
            );
        }
        1 => {
            println!("\n{:>12}Need 0 or 2 arguments\n", " ");
            return;
        }
        2 => {
            if args[1] != "upload" && args[1] != "download" {
                println!("\n{:>12}First arg is neither upload nor download\n", ' ');
                return;
            }
        }
        _ => println!(),
    }

    // Create a channel for "safe concurrency"
    // mpsc: MPSC in Rust stands for "Multi-Producer, Single-Consumer,"
    // which is a type of channel used for communication between threads,
    // allowing multiple threads to send messages to a single receiving thread.
    // This mechanism helps ensure "safe concurrency" by enabling message passing
    // instead of shared memory access.
    let (tx, rx) = mpsc::channel();

    let mut data = String::from("");
    println!("{data}"); // to prevent the unused warning

    if lena == 0 {
        data = format!("{:?}", Uuid::new_v4());
    } else {
        data = fs::read_to_string(args[2].clone())
            .unwrap()
            .trim()
            .to_string();
    }

    send_data(tx, data); // send data

    if lena == 0 {
        receive_data(rx); // receive data
        println!();
    } else {
        upload_data(rx, args[2].clone());
    }
}

// send data
fn send_data(tx: Sender<String>, data: String) {
    // Spawn a thread for blocking operation
    thread::spawn(move || {
        // Simulate a blocking operation (e.g., writing to RustFS)
        // Send data through the channel to comply with and adhere to ACID semantics
        info!("->  Sending: {:?}", data);
        tx.send(data).unwrap();
    });
}

// receive data
fn receive_data(rx: Receiver<String>) {
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

// upload data to a bucket
fn upload_data(rx: Receiver<String>, filename: String) {
    // Async operation
    let async_operation = async {
        // Wait for data from the channel
        let received_data = rx.recv().unwrap();
        // Simulate an async operation (e.g., reading from RustFS)
        info!("<- Received: {:?}", received_data);
        // upload the received data to the
        // distributed-storage-sytem:bucket
        // mentioned in .env
        dotenv().ok(); // // Load environment variables from .env file
        // Access the variables
        let bucket = env::var("BUCKET").expect("BUCKET must be set");
        let ds_url =
            env::var("DISTRIBUTED_STORAGE_URL").expect("DISTRIBUTED_STORAGE_URL must be set");
        // Yes, the `aws` cli can be used against `RustFS` 🙂
        print!("\nUploading data to {ds_url}/{bucket}/ ... ");
        let mut child = Command::new("aws")
            .arg("s3")
            .arg("cp")
            .arg("-") // stdio
            .arg("s3://".to_owned() + &bucket + "/" + &filename)
            .arg("--endpoint-url")
            .arg(ds_url)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start command");
        // .wait(); // complains about stdin
        // let ecode = child.wait().expect("failed to wait on child");
        // assert!(ecode.success());
        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(&received_data.into_bytes())
                .expect("Failed to write to stdin");
            println!("Done\n");
        };
    };
    // Execute the async operation
    block_on(async_operation);
}
