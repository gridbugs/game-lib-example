use std::time::Duration;
use input::Input;
use entity_store::*;
use cgmath::*;
use prototypes;
use policy;

const WIDTH: u32 = 10;
const HEIGHT: u32 = 10;

#[derive(Debug, Clone)]
pub struct GameState {
    pub entity_store: EntityStore,
    pub spatial_hash: SpatialHashTable,
    player_id: EntityId,
    changes: Vec<EntityChange>,
}

impl GameState {
    pub fn new() -> Self {

        let mut allocator = EntityIdAllocator::new();
        let mut entity_store = EntityStore::new();
        let mut spatial_hash = SpatialHashTable::new(WIDTH, HEIGHT);

        let strings = vec![
            "##########",
            "#........#",
            "#....#...#",
            "#..@.#...#",
            "#....#...#",
            "#.####...#",
            "#........#",
            "#........#",
            "#........#",
            "##########",
        ];

        let player_id = allocator.allocate();

        let mut changes = Vec::new();

        let mut y = 0;
        for row in strings.iter() {
            let mut x = 0;
            for ch in row.chars() {
                let coord = vec2(x, y);
                match ch {
                    '#' => {
                        prototypes::wall(&mut changes, allocator.allocate(), coord);
                        prototypes::floor(&mut changes, allocator.allocate(), coord);
                    }
                    '.' => {
                        prototypes::floor(&mut changes, allocator.allocate(), coord);
                    }
                    '@' => {
                        prototypes::player(&mut changes, player_id, coord);
                        prototypes::floor(&mut changes, allocator.allocate(), coord);
                    }
                    _ => panic!(),
                }
                x += 1;
            }
            y += 1;
        }

        for mut change in changes.drain(..) {
            spatial_hash.update(&entity_store, &change, 0);
            entity_store.commit(change);
        }

        Self {
            entity_store,
            spatial_hash,
            player_id,
            changes,
        }
    }

    pub fn tick<I: Iterator<Item=Input>>(&mut self, inputs: I, _period: Duration) {
        let player_coord = self.entity_store.coord.get(&self.player_id).cloned().unwrap();
        for input in inputs {
            match input {
                Input::Up => self.changes.push(insert::coord(self.player_id, player_coord + vec2(0, -1))),
                Input::Down => self.changes.push(insert::coord(self.player_id, player_coord + vec2(0, 1))),
                Input::Left => self.changes.push(insert::coord(self.player_id, player_coord + vec2(-1, 0))),
                Input::Right => self.changes.push(insert::coord(self.player_id, player_coord + vec2(1, 0))),
                _ => {},
            }
        }

        for change in self.changes.drain(..) {

            if !policy::check(&change, &self.entity_store, &self.spatial_hash) {
                continue;
            }

            self.spatial_hash.update(&self.entity_store, &change, 0);
            self.entity_store.commit(change);
        }
    }
}
