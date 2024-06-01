use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_rapier2d::prelude::*;
use std::time::Duration;


const PLAYER_SPRITE_X: f32 = 34.;
const PLAYER_SPRITE_Y: f32 = 48.;
const PLAYER_CAPSULE_TOP: f32 = 10.;
const PLAYER_CAPSULE_BOTTOM: f32 = -10.;

const TOWER_SPAWN_DISTANCE: f32 = 25.;
const SIMPLE_TOWER_SPAWN_CD: f32 = 5.;
const SIMPLE_TOWER_X: f32 = 21.;
const SIMPLE_TOWER_Y: f32 = 27.;

const HEALTH_BAR_RED: Color = Color::rgba(0.82,0.122,0.051,1.);
const HEALTH_BAR_GREEN: Color = Color::rgba(0.129,0.859,0.18,1.);
const HEALTH_BAR_GAP: f32= 10.;

const EYE_MONSTER_SPAWN_CD: u64 = 8;
const EYE_MONSTER_X: f32 = 60.;
const EYE_MONSTER_Y: f32 = 54.;
const EYE_MONSTER_CAPSULE_TOP: f32 = 0.;
const EYE_MONSTER_CAPSULE_BOT: f32 = 0.;

// Attributes

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

#[derive(Component)]
struct CurrentHealthBar;

#[derive(Component)]
struct MaxHealthBar;

#[derive(Component)]
struct Speed {
    speed: f32,
}

 #[derive(Component)]
struct HitBox {
    hitbox: Rect,
}

#[derive(Component)]
struct Damage {
    value: u32,
    cooldown: f32,
    last_damage_time: f32,
}

// Resources

#[derive(Resource)]
struct TowerStopwatch {
    time: Stopwatch,
}

#[derive(Resource)]
struct EyeMonsterSpawnTimer {
    timer: Timer,
}

// Entities

#[derive(Component)]
struct Player;

#[derive(Component)]
struct SimpleTower;

#[derive(Component)]
struct EyeMonster;

// Bundles

#[derive(Bundle)]
struct SimpleTowerBundle {
    health: Health,
    tower: SimpleTower,
    sprite: SpriteSheetBundle,
}

#[derive(Bundle)]
struct EyeMonsterBundle {
    health: Health,
    eyemonster: EyeMonster,
    sprite: SpriteSheetBundle,
    damage: Damage,
    hitbox: HitBox,
    speed: Speed,
    rigid_body: RigidBody,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(4.))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            player_movement,
            spawn_tower,
            draw_health_bars,
            eye_monster_spawner,
            eye_monster_movement,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    // load textures
    // player texture
    let texture: Handle<Image>  = asset_server.load("player_sprite.png");
    let player_size = Vec2::new(PLAYER_SPRITE_X, PLAYER_SPRITE_Y);
    let layout = TextureAtlasLayout::from_grid(
        player_size,
        4,
        3,
        None,
        None
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // spawn camera
    commands.spawn(Camera2dBundle::default());
    // spawn player
    commands.spawn((
        Player,
        Health {
            current: 100,
            max: 100,
        },
        Speed {
            speed: 300.,
        },
        HitBox {
            hitbox: Rect::from_center_size(Vec2::ZERO, player_size),
        },
        RigidBody::Dynamic,
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                ..default()
            },
            ..default()
        }
    )).insert(Collider::capsule(
            Vec2::new(0., PLAYER_CAPSULE_BOTTOM),
            Vec2::new(0., PLAYER_CAPSULE_TOP),
            PLAYER_SPRITE_X/2.))
    .insert(Velocity::zero())
    .insert(LockedAxes::ROTATION_LOCKED);
    // insert stopwatch
    let mut stopwatch = Stopwatch::new();
    stopwatch.set_elapsed(Duration::from_secs_f32(SIMPLE_TOWER_SPAWN_CD));
    commands.insert_resource(TowerStopwatch { time: stopwatch });
    // insert monster spawn timer
    commands.insert_resource(EyeMonsterSpawnTimer {
        timer: Timer::new(Duration::from_secs(EYE_MONSTER_SPAWN_CD),
            TimerMode::Repeating)
    });
    rapier_configuration.gravity = Vect::ZERO;
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Sprite,  &Speed), With<Player>>,
) {
    let (mut velocity, mut player_sprite, player_speed) = query.single_mut();
    let mut direction = Vec2::new(0.,0.);

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
    if direction.x < 0. {
        player_sprite.flip_x = true;
    }
    if direction.x > 0. {
        player_sprite.flip_x = false;
    }
    direction.x *= player_speed.speed;
    direction.y *= player_speed.speed;
    velocity.linvel = direction;
}

fn spawn_tower(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    query_player_transform: Query<&Transform, With<Player>>,
    mut stopwatch: ResMut<TowerStopwatch>,
    time: Res<Time>,
) {
    stopwatch.time.tick(time.delta());
    if keyboard_input.pressed(KeyCode::Space) &&
        stopwatch.time.elapsed_secs() > SIMPLE_TOWER_SPAWN_CD
    {
        let player_transform = query_player_transform.single();
        let mut spawn_location = player_transform.translation;
        spawn_location.y -= 14.;
        spawn_location.z = -spawn_location.y + SIMPLE_TOWER_Y / 2.;
        if player_transform.rotation == Quat::default() {
            spawn_location.x += TOWER_SPAWN_DISTANCE;
        } else {
            spawn_location.x -= TOWER_SPAWN_DISTANCE;
        }
        commands.spawn(SimpleTowerBundle {
            health: Health {
                current: 240,
                max: 240,
            },
            tower: SimpleTower,
            sprite: SpriteSheetBundle {
               texture: asset_server.load("SimpleTower.png"),
                transform: Transform {
                    translation: spawn_location,
                    scale: Vec3::new(0.5,0.5,1.),
                    ..default()
                },
                ..default()
            },
        });
        stopwatch.time.reset();
    }
}

fn draw_health_bars(
    mut commands: Commands,
    mut query: Query<(Entity, Option<&Children>, &Health, &HitBox)>,
    mut query_health_bar: Query<&mut Transform, With<CurrentHealthBar>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (parent, children, health, hitbox) in query.iter_mut() {
        if let Some(children) = children {
            for child in children {
                if let Ok(mut transform) = query_health_bar.get_mut(*child) {
                    transform.scale.x = health.current as f32;
                }
            }
        } else {
            // check max health
            //if health.current == health.max { continue };

            let gap = HEALTH_BAR_GAP + hitbox.hitbox.height()/2.;
            // spawn red part
            let red_health_bar = commands.spawn((
                MaxHealthBar,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    transform: Transform {
                        translation: Vec3::new(0., gap, 1.),
                        scale: Vec3::new(health.max as f32, 4., 1.),
                        ..default()
                    },
                    material: materials.add(HEALTH_BAR_RED),
                    ..default()
                }
            )).id();
            let green_health_bar = commands.spawn((
                CurrentHealthBar,
                MaterialMesh2dBundle {
                    mesh: meshes.add(Rectangle::default()).into(),
                    transform: Transform {
                        translation: Vec3::new(0., gap, 2.),
                        scale: Vec3::new(health.current as f32, 4., 1.),
                        ..default()
                    },
                    material: materials.add(HEALTH_BAR_GREEN),
                    ..default()
                }
            )).id();
            commands.entity(parent).push_children(&[red_health_bar, green_health_bar]);
        }
    }
}

fn eye_monster_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut config: ResMut<EyeMonsterSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    config.timer.tick(time.delta());

    if config.timer.finished() {
        commands.spawn(EyeMonsterBundle {
            health: Health {
                current: 100,
                max: 100,
            },
            eyemonster: EyeMonster,
            damage: Damage {
                value: 10,
                cooldown: 3.,
                last_damage_time: 0.,
            },
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(100., 100., -100.,),
                texture: asset_server.load("eyemonster.png"),
                ..default()
            },
            hitbox: HitBox {
                hitbox: Rect::from_center_size(Vec2::ZERO, Vec2::new(EYE_MONSTER_X, EYE_MONSTER_Y))
            },
            speed: Speed {
                speed: 150.,
            },
            rigid_body: RigidBody::Dynamic,
        })
        .insert(Velocity::zero())
        .insert(Collider::capsule(
                Vect::new(0., EYE_MONSTER_CAPSULE_BOT),
                Vect::new(0., EYE_MONSTER_CAPSULE_TOP),
                EYE_MONSTER_X/2.
            ))
        .insert(LockedAxes::ROTATION_LOCKED);
    }
}

fn eye_monster_movement(
    mut query_monster: Query<(&mut Velocity, &Transform, &Speed, &mut Sprite), (With<EyeMonster>, Without<Player>)>,
    query_player: Query<&Transform, (With<Player>, Without<EyeMonster>)>,
) {
    let player_coords = query_player.single().translation;
    for (mut velocity, transform, speed, mut sprite) in query_monster.iter_mut() {
        let direction = player_coords;
        let mut distance_vector = Vect::new(
            player_coords.x - transform.translation.x,
            player_coords.y - transform.translation.y,
        ).normalize_or_zero();
        distance_vector.x *= speed.speed;
        distance_vector.y *= speed.speed;

        velocity.linvel = distance_vector;
        if direction.x < 0. {
            sprite.flip_x = true;
        }
        if direction.x > 0. {
            sprite.flip_x = false;
        }
    }
}
