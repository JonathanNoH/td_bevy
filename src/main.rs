use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image>  = asset_server.load("disciple-45x51.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(45.,51.), 4, 3, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteSheetBundle {
        texture,
        atlas: TextureAtlas {
            layout: texture_atlas_layout,
            ..default()
        },
        ..default()
    });
}
