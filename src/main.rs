use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

mod objects;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins(TiledPlugin::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update_camera_position)
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
        // // Wait for map loading to complete and spawn a simple player-controlled object
        // .observe(|_: On<TiledEvent<MapCreated>>, mut commands: Commands| {
        // })
        .observe(
            |trigger: On<TiledEvent<ColliderCreated>>,
             mut commands: Commands,
             assets: Res<Assets<TiledMapAsset>>| {
                let mut entity = commands.entity(trigger.event().origin);

                // Automatically insert a `RigidBody::Static` component on all the colliders entities from the map
                entity.insert(RigidBody::Static);

                if let Some(obj) = trigger.event().get_object(&assets) {
                    println!("obj: {obj:?}");

                    let user_type = Some(obj.user_type.clone())
                        .filter(|t| !t.is_empty())
                        .or_else(|| {
                            obj.get_tile()
                                .and_then(|obj_tile| obj_tile.get_tile())
                                .and_then(|tile| tile.user_type.clone())
                                .filter(|t| !t.is_empty())
                        });

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
    camera.translation = player.translation;
}
