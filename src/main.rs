use bevy::{color::palettes::basic::GRAY, prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(HelloPlugin)
        .run();
}

#[derive(Component)]
pub struct Player {}
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, PartialEq)]
pub enum MovementState {
    Left,
    Right,
    IdleLeft,
    IdleRight,
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_sprite, move_hello, draw_base_line))
            .add_systems(Startup, (setup, hide_cursor));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let texture = asset_server.load("sprites/hello_walking.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 16, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    let window: &Window = window_query.get_single().unwrap();

    spawn_camera(&mut commands, &window);
    commands.spawn((
        Player {},
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0),
                scale: Vec3::splat(3.0),
                ..default()
            },
            texture,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 2,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        MovementState::IdleRight,
    ));
}

fn hide_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let window = &mut primary_window.single_mut();
    window.cursor.visible = false;
}

fn spawn_camera(commands: &mut Commands, window: &Window) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn draw_base_line(mut gizmos: Gizmos, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();
    let base_line = window.height() / 2.0 - 49.0;

    gizmos.line_2d(
        Vec2::new(0.0, base_line),
        Vec2::new(window.width(), base_line),
        GRAY,
    );
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
        &MovementState,
    )>,
) {
    for (indices, mut timer, mut atlas, state) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if *state == MovementState::IdleRight {
                atlas.index = 2;
            } else if *state == MovementState::IdleLeft {
                atlas.index = 10;
            } else {
                let offset = if *state == MovementState::Left { 8 } else { 0 };

                atlas.index = if atlas.index >= indices.last + offset || atlas.index < offset {
                    indices.first + offset
                } else {
                    atlas.index + 1
                };
            };

            println!("{}", atlas.index);
        }
    }
}

fn move_hello(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut MovementState, &mut Transform), With<Player>>,
) {
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        *query.single_mut().0 = MovementState::Left;
        query.single_mut().1.translation.x -= 3.0;
    } else if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        *query.single_mut().0 = MovementState::Right;
        query.single_mut().1.translation.x += 3.0;
    } else {
        if keys.just_released(KeyCode::KeyA) || keys.just_released(KeyCode::ArrowLeft) {
            *query.single_mut().0 = MovementState::IdleLeft;
        } else if keys.just_released(KeyCode::KeyD) || keys.just_released(KeyCode::ArrowRight) {
            *query.single_mut().0 = MovementState::IdleRight;
        }
    }
}
