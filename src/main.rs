use bevy::prelude::*;

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

#[derive(Component)]
struct Speed {
    speed: f32,
}

#[derive(Component)]
struct Player;



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
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

    commands.spawn((
        Player,
        Health {
            current: 100,
            max: 100,
        },
        Speed {
            speed: 300.,
        },
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                ..default()
            },
            ..default()
        }
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_transform, player_speed) = query.single_mut();
    let mut direction = Vec3::new(0.,0., 0.);

    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }
    direction = direction.normalize_or_zero();

    player_transform.translation.x += direction.x * player_speed.speed * time.delta_seconds();
    player_transform.translation.y += direction.y * player_speed.speed * time.delta_seconds();
}
