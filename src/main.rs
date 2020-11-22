use bevy::{prelude::*, render::camera::Camera, window::WindowMode};

mod base;
mod player;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy-Game".to_string(),
            width: 1920,
            height: 1080,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(player::update_player_look_direction.system())
        .add_system(player::update_player_acceleration.system())
        .add_system(base::apply_movement.system())
        .add_system(update_camera.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .spawn((player::Player, player::LookDirection::default(), base::MovementData::default(), Transform::from_translation(Vec3::new(-3.0, 5.0, 8.0))));
}

fn update_camera(
    player_query: Query<(&player::Player, &player::LookDirection, &Transform)>,
    mut camera_query: Query<(&Camera, &mut Transform)>,
) {
    for (_, look_direction, player_transform) in player_query.iter() {
        let direction = Vec3::new(0.0, 0.0, 1.0);
        let direction = Vec3::new(
            direction.x(),
            direction.y() * look_direction.pitch.cos() - direction.z() * look_direction.pitch.sin(),
            direction.z() * look_direction.pitch.cos() - direction.y() * look_direction.pitch.sin(),
        );

        let direction = Vec3::new(
            direction.x() * look_direction.yaw.cos() - direction.z() * look_direction.yaw.sin(),
            direction.y(),
            direction.z() * look_direction.yaw.cos() - direction.x() * look_direction.yaw.sin(),
        );

        let direction = direction.normalize();

        for (_camera, mut transform) in camera_query.iter_mut() {
            *transform = Transform::from_translation(player_transform.translation).looking_at(player_transform.translation + direction, Vec3::unit_y());
        }
    }
}
