use bevy::{prelude::*, render::camera::Camera};
use lazy_static::*;

const INSTRUCTIONS: &str = r#"
---- Views ----
1: Default
2: Right
3: Left
4: Top
"#;
// ---- Modes ----
// TAB: Debug view
// "#;

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

fn setup(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let material = color_materials.add(Color::NONE.into());

    commands
        // Camera
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                DEFAULT_CAMERA_TRANSORMS.1,
                DEFAULT_CAMERA_TRANSORMS.0,
            )),
            ..Default::default()
        })
        .spawn(CameraUiBundle::default())
        // root node
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material,
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    value: INSTRUCTIONS.to_string(),
                    font,
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                        ..Default::default()
                    },
                },
                ..Default::default()
            });
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

        // Right
        if keyboard_input.just_released(KeyCode::Key2) {
            transform.translation = Vec3::new(0.0, 0.0, 4.0);
            transform.rotation = Quat::identity();
        }

        // Left
        if keyboard_input.just_released(KeyCode::Key3) {
            transform.translation = Vec3::new(-4.0, 0.0, 0.0);
            transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), -90f32.to_radians());
        }

        // Top
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
