use super::shared::SharedData;
use bevy::prelude::*;
use lazy_static::*;

lazy_static! {
    static ref DESCRIPTIONS: [Mat4; 4] = {
        [
            // Pedestal
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.34, 0.7, 0.34),
                Quat::identity(),
                Vec3::new(0.0, -0.75, 0.0),
            ),
            // X pillar
            Mat4::from_scale_rotation_translation(
                Vec3::new(2.0, 0.125, 0.125),
                Quat::identity(),
                Vec3::new(-0.05, -1.0, -1.0),
            ),
            // Y pillar
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 2.0, 0.125),
                Quat::identity(),
                Vec3::new(1.0, 0.05, -1.0),
            ),
            // Z pillar
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 0.125, 2.0),
                Quat::identity(),
                Vec3::new(1.0, -1.0, 0.05),
            ),
        ]
    };
}

pub fn spawn_pillars(
    commands: &mut Commands,
    shared_data: Res<SharedData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::rgb(0.7, 0.7, 0.7).into());

    for d in DESCRIPTIONS.iter() {
        commands.spawn(PbrBundle {
            transform: Transform::from_matrix(*d),
            material: material.clone(),
            mesh: shared_data.unit_cube.clone(),
            ..Default::default()
        });
    }
}
