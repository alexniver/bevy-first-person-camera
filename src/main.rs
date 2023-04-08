use bevy::{input::mouse::MouseMotion, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_rotate)
        .add_system(camera_move)
        .run();
}

const MOUSE_SENSITIVITY: f32 = 0.2;
const MOVE_SPEED: f32 = 2.;

#[derive(Component, Default)]
struct Cam {
    yaw: f32,
    pitch: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Cam::default(),
    ));
}

fn camera_rotate(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cam_query: Query<(&mut Transform, &mut Cam)>,
    time: Res<Time>,
) {
    if let Ok((mut t, mut cam)) = cam_query.get_single_mut() {
        let mut cursor_delta = Vec2::ZERO;
        for event in mouse_motion_events.iter() {
            cursor_delta += event.delta;
        }

        let dt = time.delta_seconds();

        cam.yaw += -dt * cursor_delta.x * MOUSE_SENSITIVITY;
        cam.pitch += -dt * cursor_delta.y * MOUSE_SENSITIVITY;

        t.rotation = Quat::from_euler(EulerRot::ZYX, 0., cam.yaw, cam.pitch);
    }
}

fn camera_move(
    keyboard: Res<Input<KeyCode>>,
    mut cam_query: Query<(&mut Transform, &Cam)>,
    time: Res<Time>,
) {
    for (key, dir) in [
        (KeyCode::W, Vec3::Z),
        (KeyCode::A, -Vec3::X),
        (KeyCode::S, -Vec3::Z),
        (KeyCode::D, Vec3::X),
        (KeyCode::LShift, -Vec3::Y),
        (KeyCode::Space, Vec3::Y),
    ]
    .iter()
    .cloned()
    {
        if keyboard.pressed(key) {
            if let Ok((mut t, cam)) = cam_query.get_single_mut() {
                let dt = time.delta_seconds();
                let yaw_rot = Quat::from_axis_angle(Vec3::Y, cam.yaw);
                let rot_x = yaw_rot * Vec3::X;
                let rot_y = yaw_rot * Vec3::Y;
                let rot_z = yaw_rot * Vec3::Z;
                t.translation += dt * dir.x * rot_x * MOVE_SPEED + dt * dir.y * rot_y * MOVE_SPEED
                    - dt * dir.z * rot_z * MOVE_SPEED;
            }
        }
    }
}
