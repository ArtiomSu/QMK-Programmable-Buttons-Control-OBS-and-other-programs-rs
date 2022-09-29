use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::{RawFd, FromRawFd, IntoRawFd}};
use std::path::Path;

use input::event::KeyboardEvent;
use input::event::keyboard::{KeyboardEventTrait, KeyboardKeyEvent};
use input::{Libinput, LibinputInterface, Event};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

// https://docs.qmk.fm/#/feature_programmable_button
enum ProgrammableKeys {
    MACROUNKOWN = 0,
    MACRO1 = 656,
    MACRO2 = 657,
    MACRO3 = 658,
}

impl ProgrammableKeys {
    fn from_u32(value: u32) -> ProgrammableKeys {
        match value {
            656 => ProgrammableKeys::MACRO1,
            657 => ProgrammableKeys::MACRO2,
            658 => ProgrammableKeys::MACRO3,
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

fn process_keys(event: &KeyboardKeyEvent){
    let key = event.key();
    let state = event.key_state();
    println!("Key: {:?}, State: {:?}", key, state);

    match state {
        input::event::keyboard::KeyState::Pressed => {
            let prog_key = ProgrammableKeys::from_u32(key);
            match prog_key {
                ProgrammableKeys::MACRO1 => {
                    println!("MACRO1 PRESSED");
                },
                ProgrammableKeys::MACRO2 => {
                    println!("MACRO2 PRESSED");
                },
                ProgrammableKeys::MACRO3 => {
                    println!("MACRO3 PRESSED");
                },
                _ => {
                    println!("MACROUNKOWN PRESSED");
                }
            }
        },
        _ => {}
        
    }
}

fn main() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            match event {
                Event::Keyboard(event) => {
                    match event {
                        KeyboardEvent::Key(event) => {
                            process_keys(&event);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}