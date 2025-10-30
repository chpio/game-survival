use avian2d::prelude::*;
use bevy::{dev_tools::fps_overlay, prelude::*, window};
use bevy_ecs_tiled::prelude::*;
use std::time::Duration;

mod objects;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .build()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: window::WindowMode::Windowed,
                        present_mode: window::PresentMode::Fifo,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(fps_overlay::FpsOverlayPlugin {
            config: fps_overlay::FpsOverlayConfig {
                refresh_interval: Duration::from_millis(500),
                ..default()
            },
        })
        .add_plugins(TiledPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(
            Update,
            (update_camera_position, toggle_fps, toggle_fullscreen),
        )
        .insert_resource(Gravity::ZERO)
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
        .add_plugins(PhysicsPlugins::default().with_length_unit(100.0))
        .add_plugins(PhysicsDebugPlugin::default())
        .add_plugins((objects::well::Plugin, objects::player::Plugin))
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::AutoMax {
                max_width: 500.,
                max_height: 500.,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        objects::player::Player,
        Name::new("PlayerControlledObject (Avian2D physics)"),
        Collider::circle(10.),
        LockedAxes::ROTATION_LOCKED,
        Transform::from_xyz(50., -50., 0.),
    ));

    commands
        .spawn((
            TiledMap(asset_server.load("map.tmx")),
            TilemapRenderSettings {
                render_chunk_size: UVec2::new(3, 1),
                y_sort: true,
            },
            TilemapAnchor::Center,
        ))
        .observe(
            |collider_created: On<TiledEvent<ColliderCreated>>,
             mut commands: Commands,
             assets: Res<Assets<TiledMapAsset>>| {
                // Automatically insert a `RigidBody::Static` component on all the colliders entities from the map
                commands
                    .entity(*collider_created.event().event.collider_of)
                    .insert(RigidBody::Static);

                if let Some(obj) = collider_created.event().get_object(&assets) {
                    let user_type = Some(obj.user_type.clone())
                        .filter(|t| !t.is_empty())
                        .or_else(|| {
                            obj.get_tile()
                                .and_then(|obj_tile| obj_tile.get_tile())
                                .and_then(|tile| tile.user_type.clone())
                                .filter(|t| !t.is_empty())
                        });

                    let mut collider = commands.entity(collider_created.event().origin);

                    match user_type.as_ref().map(String::as_str) {
                        Some("well") => {
                            collider.insert((objects::well::Well, Sensor, CollisionEventsEnabled));
                        }
                        _ => {}
                    }
                };
            },
        );
}

fn update_camera_position(
    player: Single<&Transform, (With<objects::player::Player>, Without<Camera>)>,
    mut camera: Single<&mut Transform, With<Camera>>,
) {
    // keep camera z-coordinate unchanged
    camera.translation = player.translation.with_z(camera.translation.z);
}

fn toggle_fps(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut fps_overlay: ResMut<fps_overlay::FpsOverlayConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        let enabled = !fps_overlay.enabled;
        fps_overlay.enabled = enabled;
        fps_overlay.frame_time_graph_config.enabled = enabled;
    }
}

fn toggle_fullscreen(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut window: Single<&mut Window, With<window::PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyF) {
        let new_mode = match window.mode {
            window::WindowMode::Windowed => {
                window::WindowMode::BorderlessFullscreen(MonitorSelection::Current)
            }
            _ => window::WindowMode::Windowed,
        };
        window.mode = new_mode;
    }
}
