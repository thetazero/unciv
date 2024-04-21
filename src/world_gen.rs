use crate::tile;

use noise::{NoiseFn, Simplex};

fn compute_tile_kind(height: f64, biome: f64) -> tile::TileKind {
    if height < -0.1 {
        return tile::TileKind::Water;
    } else if height < 0.5 {
        if biome < 0. {
            return tile::TileKind::Desert;
        } else {
            return tile::TileKind::Forest;
        }
    } else {
        return tile::TileKind::Mountain;
    }
}

fn scaled_simplex_2d(simplex: Simplex, x: f64, y: f64, scale: f64) -> f64 {
    simplex.get([x * scale, y * scale])
}

pub fn spawn_tile_data(x_count: i32, y_count: i32) -> Vec<tile::Tile> {
    let simplex_2d = Simplex::new(2);

    let mut tiles = Vec::new();
    for x in 0..x_count {
        for y in 0..y_count {
            let x_float = x as f64;
            let y_float = y as f64;

            let height = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.15);
            let biome = scaled_simplex_2d(simplex_2d, x_float, y_float, 0.03);

            let kind: tile::TileKind = compute_tile_kind(height, biome);
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
