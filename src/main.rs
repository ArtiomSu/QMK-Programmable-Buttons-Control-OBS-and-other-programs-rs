use std::sync::{Mutex, Arc};
use std::{time};
use std::io::Error;

mod programmable_keys;
use crate::programmable_keys::programmable_keys::ProgrammableKeys;

const QUEUE_CHECKING_DELAY: time::Duration = time::Duration::from_millis(20);

#[cfg(target_os = "linux")]
mod linux_listner;

#[cfg(target_os = "windows")]
mod window_listener;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Error> {
    let programmable_keys_vec: Vec<ProgrammableKeys> = Vec::new();
    let programmable_keys_arc = Arc::new(Mutex::new(programmable_keys_vec));

    let client = match obws::Client::connect("localhost", 4455, Some("")).await {
        Ok(client) => {

            Some(client)
        },
        Err(e) => {
            eprintln!("Failed to connect to OBS: {}", e);
            None
        }
    };

    let queue = programmable_keys_arc.clone();
    tokio::spawn(async move{
        loop {
            std::thread::sleep(QUEUE_CHECKING_DELAY);

            let retrieved_key = match queue.lock() {
                Ok(mut borrowed_queue) => {
                    borrowed_queue.pop()
                },
                Err(e) => {
                    eprintln!("Error locking queue: {:?}", e);
                    None
                }
            };

            match retrieved_key {
                Some(key) => {
                    ProgrammableKeys::process_keys(key, &client).await;
                },
                None => {}
            }
        }
    });

    #[cfg(target_os = "linux")]
    linux_listner::linux_start(&programmable_keys_arc);

    #[cfg(target_os = "windows")]
    window_listener::windows_start(&programmable_keys_arc);

    Ok(())
}