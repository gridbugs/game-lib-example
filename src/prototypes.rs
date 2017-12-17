use append::Append;
use cgmath::Vector2;
use entity_store::*;
use tile::Tile;

pub fn player<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::player(id));
    changes.append(insert::collider(id));
    changes.append(insert::tile(id, Tile::Player));
    changes.append(insert::depth(id, 1));
}

pub fn wall<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::solid(id));
    changes.append(insert::tile(id, Tile::Wall));
    changes.append(insert::depth(id, 1));
}

pub fn floor<A: Append<EntityChange>>(changes: &mut A, id: EntityId, coord: Vector2<i32>) {
    changes.append(insert::coord(id, coord));
    changes.append(insert::tile(id, Tile::Floor));
    changes.append(insert::depth(id, 0));
}
