mod utils;

use js_sys::Math;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_thread as thread;

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

static mut GRID_STATE: Option<Vec<Vec<Firefly>>> = None;

#[derive(Clone, Copy, Debug)]
enum LightState {
    On,
    Off,
}

#[derive(Debug)]
struct Firefly {
    x: usize,
    y: usize,
    state: Arc<Mutex<LightState>>,
    channel: Arc<Mutex<(Sender<Message>, Receiver<Message>)>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Blink,
}

#[wasm_bindgen]
pub fn start() {
    unsafe {
        if let Some(fireflies) = &GRID_STATE {
            let (y, x) = (
                get_random_index(fireflies.len()),
                get_random_index(fireflies[0].len()),
            );
            fireflies[y][x]
                .channel
                .lock()
                .unwrap()
                .0
                .send(Message::Blink)
                .unwrap();
        }
    }
}

fn get_random_index(n: usize) -> usize {
    // Generates a random floating-point number between 0.0 (inclusive) and 1.0 (exclusive)
    let random_float = Math::random();

    // Scale to the range [0, n) and convert to u32 for an integer index
    (random_float * n as f64).floor() as usize
}

#[wasm_bindgen]
pub fn get_grid_row(i: usize) -> Vec<u8> {
    unsafe {
        if let Some(fireflies) = &GRID_STATE {
            let mut row_state = Vec::new();
            let flies = &fireflies[i];
            for fly in flies {
                let fly_state = fly.state.lock().unwrap();
                match *fly_state {
                    LightState::On => {
                        row_state.push(1);
                    }
                    LightState::Off => {
                        row_state.push(0);
                    }
                }
            }
            return row_state;
        }
    }
    vec![]
}

impl Firefly {
    // Method to create a new firefly
    fn new(x: usize, y: usize) -> Self {
        let channel = mpsc::channel();
        let channel = Arc::new(Mutex::new(channel));
        let firefly = Firefly {
            x,
            y,
            state: Arc::new(Mutex::new(LightState::Off)),
            channel,
        };
        firefly
    }

    // Method to start firefly's behavior in a separate thread
    fn start(&mut self) {
        let state = Arc::clone(&self.state);
        let (x, y) = (self.x.clone(), self.y.clone());
        let channel = Arc::clone(&self.channel);
        thread::spawn(move || {
            while let Ok(message) = channel.lock().unwrap().1.recv() {
                log!("Test");
                match message {
                    Message::Blink => {
                        Firefly::blink(Arc::clone(&state));
                        Firefly::inform_neighbours(x, y);
                    }
                }
                log!(
                    "Firefly {},{} is now in state {:?}",
                    x,
                    y,
                    state.lock().unwrap()
                );
            }
        });
    }

    fn blink(state: Arc<Mutex<LightState>>) {
        {
            let mut state = state.lock().unwrap();
            *state = LightState::On;
        }
        thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            {
                let mut state = state.lock().unwrap();
                *state = LightState::Off;
            }
        });
    }

    fn inform_neighbours(x: usize, y: usize) {
        unsafe {
            if let Some(fireflies) = &GRID_STATE {
                let rows = fireflies.len();
                let cols = fireflies[0].len();
                let up = &fireflies[(y - 1) % rows][x];
                let down = &fireflies[(y + 1) % rows][x];
                let left = &fireflies[y][(x - 1) % cols];
                let right = &fireflies[y][(x + 1) % cols];
                let neighbours = vec![up, down, left, right];
                for neighbour in neighbours {
                    neighbour
                        .channel
                        .lock()
                        .unwrap()
                        .0
                        .send(Message::Blink)
                        .unwrap();
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn setup_fireflies(rows: usize, cols: usize) {
    utils::set_panic_hook();

    let mut fireflies = Vec::new();

    // Connect neighbors with torus-like communication
    for y in 0..rows {
        fireflies.push(Vec::new());
        for x in 0..cols {
            let mut firefly = Firefly::new(x, y);

            firefly.start();

            fireflies[y].push(firefly);
        }
    }

    unsafe {
        GRID_STATE = Some(fireflies);
    }
}
