use std::fs::{File, OpenOptions};
use std::io::Error;
use std::os::unix::{fs::OpenOptionsExt, io::{RawFd, FromRawFd, IntoRawFd}};
use std::path::Path;
use std::sync::{Mutex, Arc};
use std::{thread, time};

use input::event::KeyboardEvent;
use input::event::keyboard::{KeyboardEventTrait};
use input::{Libinput, LibinputInterface, Event};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

mod programmable_keys;
use crate::programmable_keys::programmable_keys::ProgrammableKeys;

const LIBINPUT_FETCH_DELAY: time::Duration = time::Duration::from_millis(20);
const QUEUE_CHECKING_DELAY: time::Duration = time::Duration::from_millis(20);


struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}


fn watch_events(input: Libinput, queue: &Arc<Mutex<Vec<ProgrammableKeys>>> ) {
    loop {
        let mut borrowed_input:Libinput = input.clone();
        match borrowed_input.dispatch(){
            Ok(_) => {
                for event in borrowed_input {
                    match event {
                        Event::Keyboard(event) => {
                            match event {
                                KeyboardEvent::Key(event) => {
                                    match event.key_state() {
                                        input::event::keyboard::KeyState::Pressed => {
                                            let prog_key = ProgrammableKeys::from_u32(event.key());
                                            match prog_key {
                                                ProgrammableKeys::MACROUNKOWN => {
                                                    eprintln!("MACROUNKOWN PRESSED");
                                                },
                                                _ => {
                                                    match queue.lock() {
                                                        Ok(mut borrowed_queue) => {
                                                            //println!("Pushing {:?} to queue", prog_key);
                                                            borrowed_queue.push(prog_key);
                                                        },
                                                        Err(e) => {
                                                            eprintln!("Error locking queue: {:?}", e);
                                                        }
                                                    }
                                                }
                                            }
                                        },
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            },
            Err(err) => {
                eprintln!("Failed to dispatch libinput: {}", err);
            }
        }
        // sleep here so it doesn't eat up all the CPU
        thread::sleep(LIBINPUT_FETCH_DELAY);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let programmable_keys_vec: Vec<ProgrammableKeys> = Vec::new();
    let programmable_keys_arc = Arc::new(Mutex::new(programmable_keys_vec));
    let mut input = Libinput::new_with_udev(Interface);


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

    match input.udev_assign_seat("seat0") {
        Ok(_) => {
            watch_events(input, &programmable_keys_arc);
        },
        Err(_) => eprintln!("Failed to assign seat"),
    }

    Ok(())
}