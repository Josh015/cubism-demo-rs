use bevy::prelude::*;

fn create_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load cube mesh
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // ---- Pedestal & columns ----
    let material = materials.add(Color::rgb(0.7, 0.7, 0.7).into());
    let transforms: &[(Vec3, Vec3); 4] = &[
        (Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.340, 1.200, 0.340)),
        (Vec3::new(1.0, 0.05, -1.0), Vec3::new(0.125, 2.0, 0.125)),
        (Vec3::new(1.0, -1.0, 0.05), Vec3::new(0.125, 0.125, 2.0)),
        (Vec3::new(-0.05, -1.0, -1.0), Vec3::new(2.0, 0.125, 0.125)),
    ];

    for t in transforms {
        commands.spawn(PbrBundle {
            transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                t.1,
                Quat::identity(),
                t.0,
            )),
            material: material.clone(),
            mesh: cube.clone(),
            ..Default::default()
        });
    }
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_scene.system());
    }
}
