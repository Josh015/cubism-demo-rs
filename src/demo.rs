use crate::prelude::*;

pub fn keyboard_input_system(
    config: Res<DemoConfig>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, (With<Camera>, With<Camera3d>)>,
) {
    let mut transform = query.single_mut();

    // Front
    if keyboard_input.just_pressed(KeyCode::Key1) {
        *transform = config.cameras[0].to_transform();
    }

    // Right
    if keyboard_input.just_pressed(KeyCode::Key2) {
        *transform = config.cameras[1].to_transform();
    }

    // Left
    if keyboard_input.just_pressed(KeyCode::Key3) {
        *transform = config.cameras[2].to_transform();
    }

    // Top
    if keyboard_input.just_pressed(KeyCode::Key4) {
        *transform = config.cameras[3].to_transform();
    }
}
