#![feature(vec_resize_default)]
extern crate cgmath;
extern crate game_lib;
extern crate prototty;
extern crate prototty_elements;

use std::time::Duration;
use std::thread;
use cgmath::Vector2;
use prototty::Context;
use prototty_elements::canvas::*;
use game_lib::state::GameState;
use game_lib::input::Input;
use game_lib::tile::Tile;

const ETX: char = '\u{3}';
const PERIOD_MS: u64 = 16;

struct DepthCell {
    depth: u8,
    seq: u64,
}

impl Default for DepthCell {
    fn default() -> Self {
        Self {
            depth: 0,
            seq: 0,
        }
    }
}

struct DepthBuffer {
    cells: Vec<DepthCell>,
    width: u32,
    height: u32,
    seq: u64,
}

impl DepthBuffer {
    fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let mut cells = Vec::with_capacity(size);
        cells.resize_default(size);
        Self {
            cells,
            width,
            height,
            seq: 0,
        }
    }

    fn clear(&mut self) {
        self.seq += 1;
    }

    fn check<C: Into<Vector2<i32>>>(&mut self, coord: C, depth: u8) -> bool {
        let coord = coord.into();
        if coord.x < 0 || coord.y < 0 {
            return false;
        }
        let coord: Vector2<u32> = coord.cast();
        if coord.x >= self.width || coord.y >= self.height {
            return false;
        }

        let index = coord.y * self.width + coord.x;

        let cell = &mut self.cells[index as usize];
        if cell.seq < self.seq || cell.depth < depth {
            cell.seq = self.seq;
            cell.depth = depth;
            true
        } else {
            false
        }
    }
}

fn main() {
    let mut game_state = GameState::new();
    let width = game_state.spatial_hash.width();
    let height = game_state.spatial_hash.height();

    let mut context = Context::new().unwrap();
    let mut canvas = Canvas::new((width as u16, height as u16));
    let mut input_buffer = Vec::with_capacity(8);
    let period = Duration::from_millis(PERIOD_MS);
    let mut depth_buffer = DepthBuffer::new(width, height);

    'outer: loop {

        while let Some(input) = context.poll_input().unwrap() {
            let input = match input {
                prototty::Input::Up => Input::Up,
                prototty::Input::Down => Input::Down,
                prototty::Input::Left => Input::Left,
                prototty::Input::Right => Input::Right,
                prototty::Input::Char(ETX) => break 'outer,
                _ => continue,
            };

            input_buffer.push(input);
        }

        game_state.tick(input_buffer.drain(..), period);

        depth_buffer.clear();

        for (id, depth) in game_state.entity_store.depth.iter() {
            if let Some(coord) = game_state.entity_store.coord.get(&id) {
                if let Some(tile) = game_state.entity_store.tile.get(&id) {
                    if let Some(canvas_cell) = canvas.get_mut((coord.x as i16, coord.y as i16)) {
                        if depth_buffer.check((coord.x, coord.y), *depth) {
                            let (ch, bold) = match *tile {
                                Tile::Player => ('@', true),
                                Tile::Wall => ('#', false),
                                Tile::Floor => ('.', false),
                            };

                            canvas_cell.character = ch;
                            canvas_cell.bold = bold;
                        }
                    }
                }
            }
        }

        context.render(&canvas).unwrap();
        thread::sleep(period);
    }
}
