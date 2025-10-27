use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TiledPlugin::default())
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let map_handle = asset_server.load("iso-map.tmx");
    commands.spawn((
        TiledMap(map_handle),
        TilemapRenderSettings {
            // Map size is 12x12 so we'll have render chunks that are:
            // 12 tiles wide and 1 tile tall.
            render_chunk_size: UVec2::new(3, 1),
            y_sort: true,
        },
    ));
}
