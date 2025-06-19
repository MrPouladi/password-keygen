//For now this will just be windows
#![windows_subsystem = "windows"]

use std::error::Error;
use std::os::windows;
use rdev::{Event, EventType, Key};
use rand::Rng;
use copypasta::{ClipboardContext, ClipboardProvider};

//Charset
const CHARSET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789\
!@#$%^&*()-_=+[]{}|;:',.<>?/`~";

// Generates random password via charset and rand
pub fn generate_pass() -> Result<String, Box<dyn Error>> {
    let mut secure_password = String::new();
    let mut rng = rand::thread_rng();
    let charset = CHARSET.as_bytes();

    for _ in 0..24 {
        let index = rng.gen_range(0..charset.len());
        let c = charset[index] as char;
        secure_password.push(c);
    }

    Ok(secure_password)
}

// Callback function for key events
fn handle_event(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        //Uses * as the bind key (will change in future updates)
        if key == Key::KpMultiply {
            match generate_pass() {
                Ok(password) => {
                    let mut ctx = ClipboardContext::new().unwrap();
                    ctx.set_contents(password).expect("Could not set clipboard");
                }
                Err(e) => eprintln!("Error generating password: {}", e),
            }
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    // This will block and listen for events
    if let Err(error) = rdev::listen(handle_event) {
        eprintln!("Error listening to events: {:?}", error);
    }

    Ok(())
}