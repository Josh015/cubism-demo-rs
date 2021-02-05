use bevy::prelude::*;
use rand::distributions::{Distribution, Uniform};

struct LightRing;
struct LightRingVoxel;

/// Spawns a field of randomly colored and positioned lights that form a
/// tube/ring shape and spins in place.
fn spawn_voxel_light_ring(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube: &Handle<Mesh>,
    lights_count: u32,
    height: f32,
    inner_radius: f32,
    outer_radius: f32,
    min_color: Color,
    max_color: Color,
    transform: Transform,
) {
    let voxel_scale = Vec3::splat(0.05);
    let mut rng = rand::thread_rng();
    let color_randomizer = Uniform::from(0f32..=1f32);
    let radius_randomizer = Uniform::from(inner_radius..=outer_radius);
    let height_randomizer = Uniform::from((-0.5 * height)..=(0.5 * height));
    let x_randomizer = Uniform::from(-1f32..=1f32);
    let z_randomizer = Uniform::from(-1f32..=1f32);

    commands
        .spawn(PbrBundle {
            transform,
            ..Default::default()
        })
        .with(LightRing)
        .with_children(|parent| {
            for _i in 0..lights_count {
                let mut translation = Vec3::new(
                    x_randomizer.sample(&mut rng),
                    0.0,
                    z_randomizer.sample(&mut rng),
                );

                translation = translation.normalize() * radius_randomizer.sample(&mut rng);
                translation.y = height_randomizer.sample(&mut rng);

                parent
                    .spawn(PbrBundle {
                        transform: Transform::from_matrix(Mat4::from_scale_rotation_translation(
                            voxel_scale,
                            Quat::identity(),
                            translation,
                        )),
                        material: materials
                            .add(
                                Color::from(Vec4::from(min_color).lerp(
                                    Vec4::from(max_color),
                                    color_randomizer.sample(&mut rng),
                                ))
                                .into(),
                            )
                            .clone(),
                        // Material.EmissiveTint = LightColor;
                        mesh: cube.clone(),
                        ..Default::default()
                    })
                    .with(LightRingVoxel);
            }
        });
}

/// Animate all light ring voxel entities.
fn animate_light_ring(time: Res<Time>, mut query: Query<(&mut Transform, &LightRing)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(Vec3::unit_y(), time.delta_seconds()));
    }
}

fn animate_light_ring_voxels(time: Res<Time>, mut query: Query<(&mut Transform, &LightRingVoxel)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(Vec3::unit_y(), -time.delta_seconds()));
    }
}

fn create_light_rings(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load cube mesh
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // Green-yellow light ring
    spawn_voxel_light_ring(
        commands,
        &mut materials,
        &cube,
        200,
        1.0,
        0.4,
        0.9,
        Color::rgb(0.3, 0.3, 0.05),
        Color::rgb(0.6, 0.7, 0.1),
        Transform::from_translation(-0.65 * Vec3::unit_y()),
    );

    // Cyan light ring
    spawn_voxel_light_ring(
        commands,
        &mut materials,
        &cube,
        100,
        0.125,
        0.4,
        1.0,
        Color::rgb(0.05, 0.4, 0.5),
        Color::rgb(0.1, 0.5, 0.7),
        Transform::from_translation(-1.2 * Vec3::unit_y()),
    );

    // Orange light ring
    spawn_voxel_light_ring(
        commands,
        &mut materials,
        &cube,
        100,
        0.125,
        0.25,
        1.0,
        Color::rgb(0.5, 0.4, 0.05),
        Color::rgb(0.6, 0.5, 0.1),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians()),
            -1.2 * Vec3::unit_z(),
        )),
    );

    // Magenta light ring
    spawn_voxel_light_ring(
        commands,
        &mut materials,
        &cube,
        100,
        0.125,
        0.25,
        1.0,
        Color::rgb(0.1, 0.1, 0.5),
        Color::rgb(0.6, 0.2, 0.7),
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_axis_angle(Vec3::unit_z(), -90f32.to_radians()),
            1.2 * Vec3::unit_x(),
        )),
    );
}

pub struct LightRingsPlugin;
impl Plugin for LightRingsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_light_rings.system())
            .add_system(animate_light_ring.system())
            .add_system(animate_light_ring_voxels.system());
    }
}
