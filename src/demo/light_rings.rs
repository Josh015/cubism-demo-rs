use bevy::prelude::*;
use lazy_static::*;
use rand::distributions::{Distribution, Uniform};

use super::shared::SharedData;

const RING_ROTATION_SPEED: f32 = 1.0;

lazy_static! {
    static ref DESCRIPTIONS: [LightRingDesc; 4] = {
        [
            // Green-yellow light ring
            LightRingDesc {
                lights_count: 200,
                height: 0.5,
                inner_radius: 0.4,
                outer_radius: 0.7,
                min_color: Color::rgb(0.3, 0.3, 0.05),
                max_color: Color::rgb(0.6, 0.7, 0.1),
                transform: Mat4::from_translation(-0.55 * Vec3::unit_y()),
            },
            // Cyan light ring
            LightRingDesc {
                lights_count: 100,
                height: 0.125,
                inner_radius: 0.4,
                outer_radius: 1.0,
                min_color: Color::rgb(0.05, 0.4, 0.5),
                max_color: Color::rgb(0.1, 0.5, 0.7),
                transform: Mat4::from_translation(-1.2 * Vec3::unit_y()),
            },
            // Orange light ring
            LightRingDesc {
                lights_count: 100,
                height: 0.125,
                inner_radius: 0.25,
                outer_radius: 1.0,
                min_color: Color::rgb(0.5, 0.4, 0.05),
                max_color: Color::rgb(0.6, 0.5, 0.1),
                transform: Mat4::from_rotation_translation(
                    Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians()),
                    -1.2 * Vec3::unit_z(),
                ),
            },
            // Magenta light ring
            LightRingDesc {
                lights_count: 100,
                height: 0.125,
                inner_radius: 0.25,
                outer_radius: 1.0,
                min_color: Color::rgb(0.1, 0.1, 0.5),
                max_color: Color::rgb(0.6, 0.2, 0.7),
                transform: Mat4::from_rotation_translation(
                    Quat::from_axis_angle(Vec3::unit_z(), -90f32.to_radians()),
                    1.2 * Vec3::unit_x(),
                ),
            },
        ]
    };
}

struct LightRingDesc {
    lights_count: u32,
    height: f32,
    inner_radius: f32,
    outer_radius: f32,
    min_color: Color,
    max_color: Color,
    transform: Mat4,
}

pub struct LightRing;
pub struct LightRingVoxel;

pub fn spawn_voxel_light_rings(
    commands: &mut Commands,
    shared_data: Res<SharedData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn voxel light rings
    for d in DESCRIPTIONS.iter() {
        let voxel_scale = Vec3::splat(0.025);
        let mut rng = rand::thread_rng();
        let color_randomizer = Uniform::from(0f32..=1f32);
        let radius_randomizer = Uniform::from(d.inner_radius..=d.outer_radius);
        let height_randomizer = Uniform::from((-0.5 * d.height)..=(0.5 * d.height));
        let x_randomizer = Uniform::from(-1f32..=1f32);
        let z_randomizer = Uniform::from(-1f32..=1f32);

        commands
            .spawn(PbrBundle {
                transform: Transform::from_matrix(d.transform),
                ..Default::default()
            })
            .with(LightRing)
            .with_children(|parent| {
                for _i in 0..d.lights_count {
                    let light_color = Color::from(
                        1.5 * Vec4::from(d.min_color)
                            .lerp(Vec4::from(d.max_color), color_randomizer.sample(&mut rng)),
                    );
                    let mut translation = Vec3::new(
                        x_randomizer.sample(&mut rng),
                        0.0,
                        z_randomizer.sample(&mut rng),
                    );

                    translation = translation.normalize() * radius_randomizer.sample(&mut rng);
                    translation.y = height_randomizer.sample(&mut rng);

                    parent
                        .spawn(PbrBundle {
                            mesh: shared_data.unit_cube.clone(),
                            material: materials.add(StandardMaterial {
                                albedo: light_color,
                                shaded: false,
                                ..Default::default()
                            }),
                            transform: Transform::from_matrix(
                                Mat4::from_scale_rotation_translation(
                                    voxel_scale,
                                    Quat::identity(),
                                    translation,
                                ),
                            ),
                            ..Default::default()
                        })
                        // .with(Light {
                        //     color: light_color,
                        //     ..Default::default()
                        // })
                        .with(LightRingVoxel);
                }
            });
    }
}

pub fn animate_light_ring(time: Res<Time>, mut query: Query<(&mut Transform, &LightRing)>) {
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(
            Vec3::unit_y(),
            RING_ROTATION_SPEED * time.delta_seconds(),
        ));
    }
}

pub fn animate_light_ring_voxels(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &LightRingVoxel)>,
) {
    // Rotate the cubes opposite the ring so that they always face the same way.
    for (mut transform, _) in query.iter_mut() {
        transform.rotate(Quat::from_axis_angle(
            Vec3::unit_y(),
            RING_ROTATION_SPEED * -time.delta_seconds(),
        ));
    }
}
