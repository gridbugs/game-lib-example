#![feature(vec_resize_default)]
extern crate game_lib;

use std::mem;
use std::slice;
use std::time::Duration;
use std::os::raw::c_void;

use game_lib::state::GameState;
use game_lib::input::Input;
use game_lib::tile::Tile;

const SPACE: u8 = 0x20;
const PERIOD: u8 = 0x2E;
const HASH: u8 = 0x23;
const AT: u8 = 0x40;

struct Cell {
    seq: u64,
    depth: u8,
    ascii: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            seq: 0,
            depth: 0,
            ascii: SPACE,
        }
    }
}

fn ascii_from_tile(tile: Tile) -> u8 {
    match tile {
        Tile::Player => AT,
        Tile::Floor => PERIOD,
        Tile::Wall => HASH,
    }
}

pub struct State {
    game_state: GameState,
    buffer: Vec<Cell>,
    seq: u64,
}

impl State {
    fn new() -> Self {
        let game_state = GameState::new();
        let size = (game_state.spatial_hash.width() * game_state.spatial_hash.height()) as usize;
        let mut buffer = Vec::with_capacity(size);
        buffer.resize_default(size);
        State {
            game_state,
            buffer,
            seq: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn alloc_state() -> *mut c_void {
    let state = Box::new(State::new());
    Box::into_raw(state) as *mut c_void
}

#[no_mangle]
pub extern "C" fn alloc_buffer(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    ptr as *mut c_void
}

#[no_mangle]
pub unsafe fn width(state: *mut State) -> u32 {
    (*state).game_state.spatial_hash.width()
}

#[no_mangle]
pub unsafe fn height(state: *mut State) -> u32 {
    (*state).game_state.spatial_hash.height()
}

#[no_mangle]
pub unsafe fn tick(state: *mut State, input_buffer: *const u8, num_inputs: usize, period_millis: f64) {
    let period = Duration::from_millis(period_millis as u64);
    let inputs = slice::from_raw_parts(input_buffer, num_inputs);
    (*state).game_state.tick(inputs.iter().map(|i| Input::from_u8(*i).expect("Input out of bounds")), period);
}

#[no_mangle]
pub unsafe fn buffer_ascii(state: *mut State, buffer: *mut u8, size: usize) {
    (*state).seq += 1;

    let width = (*state).game_state.spatial_hash.width();
    let entity_store = &(*state).game_state.entity_store;
    let buffer = slice::from_raw_parts_mut(buffer, size);
    for (id, depth) in entity_store.depth.iter() {
        if let Some(coord) = entity_store.coord.get(&id) {
            if let Some(tile) = entity_store.tile.get(&id) {
                let buffer_index = (width as usize) * (coord.y as usize) + (coord.x as usize);
                if let Some(cell) = (*state).buffer.get_mut(buffer_index) {
                    if cell.seq < (*state).seq || cell.depth < *depth {
                        cell.seq = (*state).seq;
                        cell.depth = *depth;
                        cell.ascii = ascii_from_tile(*tile);
                        buffer[buffer_index] = cell.ascii;
                    }
                }
            }
        }
    }
}
