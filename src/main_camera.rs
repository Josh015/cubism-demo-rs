use bevy::{prelude::*, render::camera::Camera};
use lazy_static::*;

lazy_static! {
    static ref DEFAULT_CAMERA_TRANSORMS: (Vec3, Quat) = {
        (
            Vec3::new(-3.0, 2.25, 3.0),
            (Quat::from_axis_angle(Vec3::unit_y(), -45f32.to_radians())
                * Quat::from_axis_angle(Vec3::unit_x(), -30f32.to_radians()))
            .normalize(),
        )
    };
}

fn setup(commands: &mut Commands) {
    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                DEFAULT_CAMERA_TRANSORMS.1,
                DEFAULT_CAMERA_TRANSORMS.0,
            )),
            ..Default::default()
        })
        // Light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(-4.0, 6.0, 4.0)),
            ..Default::default()
        });
}

fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (mut transform, _) in query.iter_mut() {
        // Default
        if keyboard_input.just_pressed(KeyCode::Key1) {
            transform.translation = DEFAULT_CAMERA_TRANSORMS.0;
            transform.rotation = DEFAULT_CAMERA_TRANSORMS.1;
        }

        // +Z
        if keyboard_input.just_released(KeyCode::Key2) {
            transform.translation = Vec3::new(0.0, 0.0, 4.0);
            transform.rotation = Quat::identity();
        }

        // -X
        if keyboard_input.just_released(KeyCode::Key3) {
            transform.translation = Vec3::new(-4.0, 0.0, 0.0);
            transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), -90f32.to_radians());
        }

        // +Y
        if keyboard_input.just_released(KeyCode::Key4) {
            transform.translation = Vec3::new(0.0, 4.0, 0.0);
            transform.rotation = Quat::from_axis_angle(Vec3::unit_x(), -90f32.to_radians());
        }
    }
}

pub struct MainCameraPlugin;
impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(keyboard_input.system());
    }
}
