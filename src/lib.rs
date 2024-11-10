mod utils;

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_thread as thread;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Clone, Copy, Debug)]
enum LightState {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Firefly {
    x: usize,
    y: usize,
    state: LightState,
    neighbours: Vec<Arc<Mutex<(Sender<Message>, Receiver<Message>)>>>,
    channel: Arc<Mutex<(Sender<Message>, Receiver<Message>)>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TurnOn,
    TurnOff,
}

impl Firefly {
    // Method to create a new firefly
    fn new(
        x: usize,
        y: usize,
        neighbours: Vec<Arc<Mutex<(Sender<Message>, Receiver<Message>)>>>,
        channel: Arc<Mutex<(Sender<Message>, Receiver<Message>)>>,
    ) -> Self {
        let firefly = Firefly {
            x,
            y,
            state: LightState::Off,
            neighbours,
            channel,
        };
        firefly
    }

    // Method to toggle the light state
    fn _toggle_light(&mut self) {
        self.state = match self.state {
            LightState::On => LightState::Off,
            LightState::Off => LightState::On,
        };
        println!(
            "Firefly at ({}, {}) is now {:?}",
            self.x, self.y, self.state
        );
    }

    // Method to start firefly's behavior in a separate thread
    fn start(mut self) {
        thread::spawn(move || {
            while let Ok(message) = self.channel.lock().unwrap().1.recv() {
                match message {
                    Message::TurnOn => self.state = LightState::On,
                    Message::TurnOff => self.state = LightState::Off,
                }
                println!("Firefly at ({}, {}) received {:?}", self.x, self.y, message);
                self.neighbours.iter().for_each(|neighbor| {
                    neighbor.lock().unwrap().0.send(message).unwrap();
                });
            }
        });
    }
}

#[wasm_bindgen]
pub fn setup_fireflies(n: usize, m: usize) {
    utils::set_panic_hook();

    let mut channels = Vec::new();
    let mut fireflies = Vec::new();

    // Create the fireflies and store their channels
    for i in 0..n {
        channels.push(Vec::new());
        for _ in 0..m {
            let (tx, rx) = mpsc::channel();
            channels[i].push(Arc::new(Mutex::new((tx, rx))));
        }
    }

    // Connect neighbors with torus-like communication
    for i in 0..n {
        for j in 0..m {
            let mut neighbors = vec![];
            let up = (i + n - 1) % n;
            let down = (i + 1) % n;
            let left = (j + m - 1) % m;
            let right = (j + 1) % m;
            {
                neighbors.push(Arc::clone(&channels[up][j]));
                neighbors.push(Arc::clone(&channels[down][j]));
                neighbors.push(Arc::clone(&channels[i][left]));
                neighbors.push(Arc::clone(&channels[i][right]));
            }

            let firefly = Firefly::new(i, j, neighbors, Arc::clone(&channels[i][j]));
            fireflies.push(firefly.clone());

            // Start the firefly behavior
            firefly.start();
        }
    }
}
