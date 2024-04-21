use crate::tile;

use noise::{NoiseFn, Simplex};

fn compute_tile_kind(height: f64) -> tile::TileKind {
    if height < -0.1 {
        return tile::TileKind::Water;
    } else if height < 0.4 {
        return tile::TileKind::Forest;
    } else {
        return tile::TileKind::Mountain;
    }
}

pub fn spawn_tile_data(x_count: i32, y_count: i32) -> Vec<tile::Tile> {
    let simplex_2d = Simplex::new(2);

    let mut tiles = Vec::new();
    for x in 0..x_count {
        for y in 0..y_count {
            let x_float = x as f64 / 10.0;
            let y_float = y as f64 / 10.0;

            let height = simplex_2d.get([x_float, y_float]);

            let kind: tile::TileKind = compute_tile_kind(height);
            tiles.push(tile::Tile {
                x,
                y,
                kind: kind.clone(),
                neighbors: vec![],
                owner: None,
            });
        }
    }
    tiles
}
