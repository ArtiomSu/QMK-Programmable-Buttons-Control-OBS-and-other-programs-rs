use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::{RawFd, FromRawFd, IntoRawFd}};
use std::path::Path;
use std::sync::{Mutex, Arc};
use std::{thread, time};

use input::event::KeyboardEvent;
use input::event::keyboard::{KeyboardEventTrait, KeyboardKeyEvent};
use input::{Libinput, LibinputInterface, Event};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

const LIBINPUT_FETCH_DELAY: time::Duration = time::Duration::from_millis(20);
const QUEUE_CHECKING_DELAY: time::Duration = time::Duration::from_millis(20);

// https://docs.qmk.fm/#/feature_programmable_button
#[derive(Debug)]
enum ProgrammableKeys {
    MACROUNKOWN = 0,
    MACRO1 = 656,
    MACRO2 = 657,
    MACRO3 = 658,
    MACRO4 = 659,
    MACRO5 = 660,
    MACRO6 = 661,
    MACRO7 = 662,
    MACRO8 = 663,
    MACRO9 = 664,
    MACRO10 = 665,
    MACRO11 = 666,
    MACRO12 = 667,
    MACRO13 = 668,
    MACRO14 = 669,
    MACRO15 = 670,
    MACRO16 = 671,
    MACRO17 = 672,
    MACRO18 = 673,
    MACRO19 = 674,
    MACRO20 = 675,
    MACRO21 = 676,
    MACRO22 = 677,
    MACRO23 = 678,
    MACRO24 = 679,
    MACRO25 = 680,
    MACRO26 = 681,
    MACRO27 = 682,
    MACRO28 = 683,
    MACRO29 = 684,
    MACRO30 = 685,
    MACRO31 = 686,
    MACRO32 = 687
}

impl ProgrammableKeys {
    fn from_u32(value: u32) -> ProgrammableKeys {
        match value {
            656 => ProgrammableKeys::MACRO1,
            657 => ProgrammableKeys::MACRO2,
            658 => ProgrammableKeys::MACRO3,
            659 => ProgrammableKeys::MACRO4,
            660 => ProgrammableKeys::MACRO5,
            661 => ProgrammableKeys::MACRO6,
            662 => ProgrammableKeys::MACRO7,
            663 => ProgrammableKeys::MACRO8,
            664 => ProgrammableKeys::MACRO9,
            665 => ProgrammableKeys::MACRO10,
            666 => ProgrammableKeys::MACRO11,
            667 => ProgrammableKeys::MACRO12,
            668 => ProgrammableKeys::MACRO13,
            669 => ProgrammableKeys::MACRO14,
            670 => ProgrammableKeys::MACRO15,
            671 => ProgrammableKeys::MACRO16,
            672 => ProgrammableKeys::MACRO17,
            673 => ProgrammableKeys::MACRO18,
            674 => ProgrammableKeys::MACRO19,
            675 => ProgrammableKeys::MACRO20,
            676 => ProgrammableKeys::MACRO21,
            677 => ProgrammableKeys::MACRO22,
            678 => ProgrammableKeys::MACRO23,
            679 => ProgrammableKeys::MACRO24,
            680 => ProgrammableKeys::MACRO25,
            681 => ProgrammableKeys::MACRO26,
            682 => ProgrammableKeys::MACRO27,
            683 => ProgrammableKeys::MACRO28,
            684 => ProgrammableKeys::MACRO29,
            685 => ProgrammableKeys::MACRO30,
            686 => ProgrammableKeys::MACRO31,
            687 => ProgrammableKeys::MACRO32,
            _ => {
                println!("Unknown programmable key {}", value);
                ProgrammableKeys::MACROUNKOWN
            },
        }
    }
}

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

fn process_keys(key: ProgrammableKeys){
    match key {
        ProgrammableKeys::MACRO1 => {
            println!("MACRO1 PRESSED");
        },
        ProgrammableKeys::MACRO2 => {
            println!("MACRO2 PRESSED");
        },
        ProgrammableKeys::MACRO3 => {
            println!("MACRO3 PRESSED");
        },
        ProgrammableKeys::MACRO4 => {
            println!("MACRO4 PRESSED");
        },
        ProgrammableKeys::MACRO5 => {
            println!("MACRO5 PRESSED");
        },
        ProgrammableKeys::MACRO6 => {
            println!("MACRO6 PRESSED");
        },
        ProgrammableKeys::MACRO7 => {
            println!("MACRO7 PRESSED");
        },
        ProgrammableKeys::MACRO8 => {
            println!("MACRO8 PRESSED");
        },
        ProgrammableKeys::MACRO9 => {
            println!("MACRO9 PRESSED");
        },
        ProgrammableKeys::MACRO10 => {
            println!("MACRO10 PRESSED");
        },
        ProgrammableKeys::MACRO11 => {
            println!("MACRO11 PRESSED");
        },
        ProgrammableKeys::MACRO12 => {
            println!("MACRO12 PRESSED");
        },
        ProgrammableKeys::MACRO13 => {
            println!("MACRO13 PRESSED");
        },
        ProgrammableKeys::MACRO14 => {
            println!("MACRO14 PRESSED");
        },
        ProgrammableKeys::MACRO15 => {
            println!("MACRO15 PRESSED");
        },
        ProgrammableKeys::MACRO16 => {
            println!("MACRO16 PRESSED");
        },
        ProgrammableKeys::MACRO17 => {
            println!("MACRO17 PRESSED");
        },
        ProgrammableKeys::MACRO18 => {
            println!("MACRO18 PRESSED");
        },
        ProgrammableKeys::MACRO19 => {
            println!("MACRO19 PRESSED");
        },
        ProgrammableKeys::MACRO20 => {
            println!("MACRO20 PRESSED");
        },
        ProgrammableKeys::MACRO21 => {
            println!("MACRO21 PRESSED");
        },
        ProgrammableKeys::MACRO22 => {
            println!("MACRO22 PRESSED");
        },
        ProgrammableKeys::MACRO23 => {
            println!("MACRO23 PRESSED");
        },
        ProgrammableKeys::MACRO24 => {
            println!("MACRO24 PRESSED");
        },
        ProgrammableKeys::MACRO25 => {
            println!("MACRO25 PRESSED");
        },
        ProgrammableKeys::MACRO26 => {
            println!("MACRO26 PRESSED");
        },
        ProgrammableKeys::MACRO27 => {
            println!("MACRO27 PRESSED");
        },
        ProgrammableKeys::MACRO28 => {
            println!("MACRO28 PRESSED");
        },
        ProgrammableKeys::MACRO29 => {
            println!("MACRO29 PRESSED");
        },
        ProgrammableKeys::MACRO30 => {
            println!("MACRO30 PRESSED");
        },
        ProgrammableKeys::MACRO31 => {
            println!("MACRO31 PRESSED");
        },
        ProgrammableKeys::MACRO32 => {
            println!("MACRO32 PRESSED");
        },
        _ => {
            println!("MACROUNKOWN PRESSED");
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

fn main() {
    let programmable_keys_vec: Vec<ProgrammableKeys> = Vec::new();
    let programmable_keys_arc = Arc::new(Mutex::new(programmable_keys_vec));
    let mut input = Libinput::new_with_udev(Interface);

    let queue = programmable_keys_arc.clone();
    let reader_thread = std::thread::spawn(move || {
        loop {
            std::thread::sleep(QUEUE_CHECKING_DELAY);

            match queue.lock() {
                Ok(mut borrowed_queue) => {
                    //println!("queue: {:?}", borrowed_queue);
                    while let Some(key) = borrowed_queue.pop() {
                        //println!("ack popped: {:?}", key);
                        process_keys(key);
                    }
                },
                Err(e) => {
                    eprintln!("Error locking queue: {:?}", e);
                }
            }
        }
    });

    match input.udev_assign_seat("seat0") {
        Ok(_) => {
            watch_events(input, &programmable_keys_arc);
        },
        Err(_) => eprintln!("Failed to assign seat"),
    }

    match reader_thread.join() {
        Ok(_) => {},
        Err(_) => eprintln!("Failed to join reader thread"),
    }

}