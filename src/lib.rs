mod utils;

use js_sys::Math;
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
const COUPLING_STRENGTH: f32 = 0.01;

#[derive(Clone, Copy, Debug)]
struct State {
    phase: f32,
    frequency: f32,
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Firefly {
    position: Position,
    state: Arc<Mutex<State>>,
}

#[wasm_bindgen]
pub fn get_grid_row(i: usize) -> Vec<u8> {
    unsafe {
        if let Some(fireflies) = &GRID_STATE {
            let mut row_state = Vec::new();
            let flies = &fireflies[i];
            for fly in flies {
                let fly_state = fly.state.lock().unwrap();
                if fly_state.phase > std::f32::consts::PI {
                    row_state.push(1);
                } else {
                    row_state.push(0);
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
        let rand_factor_phase = Math::random() as f32;
        let state = State {
            phase: rand_factor_phase * 2.0 * std::f32::consts::PI,
            frequency: std::f32::consts::PI * 0.01,
        };
        let firefly = Firefly {
            position: Position { x, y },
            state: Arc::new(Mutex::new(state)),
        };
        firefly
    }

    // Method to start firefly's behavior in a separate thread
    fn start(&mut self) {
        let state = Arc::clone(&self.state);
        let position = self.position.clone();
        let mut neighbors = None;
        thread::spawn(move || loop {
            match &neighbors {
                Some(neighbors) => Firefly::update(&state, &neighbors),
                None => {
                    neighbors = Firefly::get_neighbours(position.x, position.y);
                }
            }
            thread::sleep(Duration::from_millis(10));
        });
    }

    fn update(state: &Arc<Mutex<State>>, neighbors: &Vec<&Firefly>) {
        let last_state;
        {
            last_state = state.lock().unwrap().clone();
        }
        // Update the phase based on natural frequency and coupling with neighbors
        let neighbor_phase_difference: f32 = neighbors
            .iter()
            .map(|n| {
                let other_state = n.state.lock().unwrap();
                (other_state.phase - last_state.phase).sin()
            })
            .sum();
        let phase_tick = last_state.frequency
            + (COUPLING_STRENGTH / neighbors.len() as f32) * neighbor_phase_difference;
        let total_phase = 2.0 * std::f32::consts::PI;
        {
            let mut state = state.lock().unwrap();
            state.phase += phase_tick;
            if state.phase >= total_phase {
                state.phase -= total_phase;
            }
        }
    }

    fn get_neighbours(x: usize, y: usize) -> Option<Vec<&'static Firefly>> {
        unsafe {
            if let Some(fireflies) = &GRID_STATE {
                let rows = fireflies.len();
                let cols = fireflies[0].len();
                let up = &fireflies[(y + rows - 1) % rows][x];
                let down = &fireflies[(y + 1) % rows][x];
                let left = &fireflies[y][(x + cols - 1) % cols];
                let right = &fireflies[y][(x + 1) % cols];
                let neighbours = vec![up, down, left, right];
                return Some(neighbours);
            } else {
                None
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
            let firefly = Firefly::new(x, y);
            fireflies[y].push(firefly);
            fireflies[y][x].start();
        }
    }

    unsafe {
        GRID_STATE = Some(fireflies);
    }
}
