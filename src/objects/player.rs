use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Default, Clone, Component)]
pub struct Player;

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player_vel: Query<&mut LinearVelocity, With<Player>>,
) {
    const MOVE_SPEED: f32 = 200.;

    for mut vel in q_player_vel.iter_mut() {
        let mut direction = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec2::new(1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec2::new(1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec2::new(0.0, 1.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec2::new(0.0, 1.0);
        }

        vel.0 = direction.normalize_or_zero() * MOVE_SPEED;
    }
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}
