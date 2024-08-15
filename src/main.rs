use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}, window::WindowResized};
use rand::Rng;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
        .add_systems(Update, animate_color)
        .add_systems(Update, change_clear_color)
        .insert_resource(ColorChangeTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#mygame-canvas".into()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();

}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct AnimatedColor;

#[derive(Resource)]
struct ColorChangeTimer(Timer);

const BALL_RADIUS: f32 = 50.0;

fn setup(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((Player, AnimatedColor, MaterialMesh2dBundle {
        mesh: Mesh2dHandle(
            meshes.add(Circle { radius: BALL_RADIUS })
        ),
        material: materials.add(Color::WHITE),
        ..Default::default()
    }));
} 

const MOVE_SPEED: f32 = 8.0;
fn move_player(
    mut transforms: Query<&mut Transform, With<Player>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut windows: Query<&mut Window>,
) {
    let window = windows.single_mut();

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;


    for mut transform in transforms.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }

        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }

        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if 0.0 < direction.length() {
            transform.translation += MOVE_SPEED * direction.normalize();
        }


        if transform.translation.x > half_width {
            transform.translation.x = -half_width;
        } else if transform.translation.x < -half_width {
            transform.translation.x = half_width;
        }

        if transform.translation.y > half_height {
            transform.translation.y = -half_height;
        } else if transform.translation.y < -half_height {
            transform.translation.y = half_height;
        }
        
    }
        
        
}

fn animate_color(
    time: Res<Time>,
    mut timer: ResMut<ColorChangeTimer>,
    mut query: Query<&mut Handle<ColorMaterial>, With<AnimatedColor>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        timer.0.reset();
    }

    let t = timer.0.fraction();

    let color = Color::hsla(360.0 * t, 1.0, 0.5, 1.0);

    for material_handle in query.iter_mut() {
        if let Some(material) = materials.get_mut(&*material_handle) {
            material.color = color;
        }
    }
}



fn change_clear_color(
    input: Res<ButtonInput<KeyCode>>,
    mut clear_color: ResMut<ClearColor>
) {
    if input.just_pressed(KeyCode::Space) {
        clear_color.0 = generate_random_color()
    }
}

fn generate_random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::srgb(
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
    )
}
