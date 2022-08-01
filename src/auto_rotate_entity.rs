use crate::prelude::*;

#[derive(Component)]
pub struct AutoRotateEntity;

pub fn rotate_on_local_axis_system(
    config: Res<DemoConfig>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<AutoRotateEntity>>,
) {
    // Rotate the child entity around its local y-axis.
    let rotation = Quat::from_axis_angle(
        Vec3::Y,
        config.auto_rotate_entity_speed * time.delta_seconds(),
    );

    for mut transform in &mut query {
        transform.rotate(rotation);
    }
}
