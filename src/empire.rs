use crate::colors::bright_hue;
use crate::tile::Tile;
use bevy::prelude::*;

use core::iter::zip;
use rand::seq::IteratorRandom;

#[derive(Component)]
pub struct Empire {
    pub id: i32,
    pub color: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Inventory {
    pub wood: i32,
    pub stone: i32,
}

pub fn spawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Tile)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const NUMBER_OF_EMPIRES: i64 = 10;

    let mut empire_list = vec![];

    for i in 0..NUMBER_OF_EMPIRES {
        let color = materials.add(bright_hue(i as f32 / NUMBER_OF_EMPIRES as f32));
        let empire = commands
            .spawn((
                Empire {
                    id: i as i32,
                    color: color.clone(),
                },
                TransformBundle::default(),
                InheritedVisibility::default(),
            ))
            .id();

        empire_list.push((empire, color));
    }

    let mut rng = rand::thread_rng();
    let spawn_tiles = query
        .iter_mut()
        .choose_multiple(&mut rng, NUMBER_OF_EMPIRES as usize);

    for ((entity, mut tile), (empire, color)) in zip(spawn_tiles, empire_list) {
        commands.entity(empire).push_children(&[entity]);
        commands.entity(entity).insert(color.clone());

        tile.owner = Some(empire);
    }
}
