use bevy::{color::palettes::basic::WHITE, prelude::*, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, hide_cursor))
        .add_systems(Update, (draw_circle, move_circle))
        .run();
}

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn draw_circle(mut gizmos: Gizmos, query: Query<&Position>) {
    gizmos.circle_2d(Vec2::new(query.single().x, query.single().y), 10., WHITE);
}

fn move_circle(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Position>) {
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        query.single_mut().y = query.single_mut().y + 5.;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        query.single_mut().y = query.single_mut().y - 5.;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        query.single_mut().x = query.single_mut().x - 5.;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        query.single_mut().x = query.single_mut().x + 5.;
    }
}

fn hide_cursor(mut primary_window: Query<&mut Window, With<PrimaryWindow>>) {
    let window = &mut primary_window.single_mut();
    window.cursor.visible = false;
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(Position { x: 0., y: 0. });
}
