use avian2d::prelude::*;
use bevy::prelude::*;

use crate::objects::player;

#[derive(Default, Clone, Component)]
pub struct Well;

fn player_on_well(
    mut collision_reader: EventReader<CollisionStarted>,
    q_well: Query<Entity, With<Well>>,
    q_player: Query<Entity, With<player::Player>>,
) {
    for event in collision_reader
        .read()
        .filter(|event| q_well.contains(event.0) && q_player.contains(event.1))
    {
        println!(
            "{} and {} started colliding",
            event.0, event.1
        );
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_on_well);
    }
}
