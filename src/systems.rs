use bevy::prelude::*;
use rand::{
    distributions::Uniform, prelude::Distribution, rngs::SmallRng, SeedableRng,
};
use std::{collections::HashMap, io::Read};

use crate::{components::*, serialization::*};

pub fn handle_keyboard_input(
    config: Res<Config>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<Camera>, With<Camera3d>)>,
    mut writer: EventWriter<AppExit>,
) {
    use KeyCode::*;

    let mut transform = query.single_mut();
    const CAMERA_BUTTONS: [KeyCode; 4] = [Digit1, Digit2, Digit3, Digit4];

    for (i, key_code) in CAMERA_BUTTONS.iter().enumerate() {
        if keyboard_input.just_pressed(*key_code) {
            *transform = config.cameras[i].to_transform();
        }
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        writer.send(AppExit::Success);
    }
}

pub fn animate_wave_voxels(
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &WaveVoxel)>,
) {
    let wave_simulation = (config.wave_voxel_speed * time.elapsed_seconds())
        % std::f32::consts::TAU;

    for (mut transform, wave_voxel) in &mut query {
        let waves = match wave_voxel.animation {
            WaveVoxelAnimation::Ripple => (wave_simulation
                + config.wave_voxel_tiling
                    * (wave_voxel.grid_position_2d.x
                        + wave_voxel.grid_position_2d.y))
                .sin(),
            WaveVoxelAnimation::Wave => {
                (wave_simulation
                    + config.wave_voxel_tiling * wave_voxel.grid_position_2d.x)
                    .sin()
                    + (wave_simulation
                        + config.wave_voxel_tiling
                            * wave_voxel.grid_position_2d.y)
                        .sin()
            },
        };

        transform.translation.y = 0.5 * config.wave_voxel_height * waves;
    }
}

pub fn automatically_rotate_on_local_axis(
    config: Res<Config>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<AutomaticRotation>>,
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

pub fn spawn_demo_scene(
    config: Res<Config>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let unit_cube = meshes.add(Cuboid {
        half_size: Vec3::splat(0.5),
    });

    // ---- Camera ----
    commands.spawn(Camera3dBundle {
        transform: config.cameras[0].to_transform(),
        ..default()
    });

    // ---- Environment Lighting ----
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 180.0,
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-0.75, 1.5, 0.75)),
        point_light: PointLight {
            range: 50.0,
            intensity: 80_000.0,
            radius: 10.0,
            ..default()
        },
        ..default()
    });

    // ---- UI ----
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    config.instructions.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::srgb(0.8, 0.8, 0.8),
                    },
                ),
                ..default()
            });
        });

    // ---- Pillars ----
    for d in &config.pillars {
        commands.spawn(PbrBundle {
            transform: d.transforms.to_transform(),
            mesh: unit_cube.clone(),
            material: materials.add(StandardMaterial {
                base_color: d.color,
                perceptual_roughness: 1.0,
                // metallic: 1.0,
                ..default()
            }),
            ..default()
        });
    }

    // ---- Light Rings ----
    let unit_sphere = meshes.add(Sphere { radius: 0.5 });
    let axis_randomizer = Uniform::from(-1f32..=1f32);
    let color_randomizer = Uniform::from(0f32..=1f32);
    let mut rng = SmallRng::from_entropy();

    for d in &config.light_rings {
        let voxel_scale = Vec3::splat(d.light_size);
        let radius_randomizer = Uniform::from(d.inner_radius..=d.outer_radius);
        let height_randomizer =
            Uniform::from((-0.5 * d.height)..=(0.5 * d.height));

        commands
            .spawn(PbrBundle {
                transform: d.transforms.to_transform(),
                ..default()
            })
            .with_children(|parent| {
                // Light ring must be a child component so it can rotate around
                // its own local axis.
                parent
                    .spawn((AutomaticRotation, PbrBundle::default()))
                    .with_children(|parent| {
                        for _i in 0..d.lights_count {
                            // HACK: Force linear color interpolation.

                            let light_color = d.min_color.mix(
                                &d.max_color,
                                color_randomizer.sample(&mut rng),
                            );
                            let mut translation = Vec3::new(
                                axis_randomizer.sample(&mut rng),
                                0.0,
                                axis_randomizer.sample(&mut rng),
                            );

                            translation = translation.normalize()
                                * radius_randomizer.sample(&mut rng);
                            translation.y = height_randomizer.sample(&mut rng);

                            parent
                                .spawn(PbrBundle {
                                    mesh: unit_sphere.clone(),
                                    material: materials.add(StandardMaterial {
                                        base_color: (light_color.to_linear()
                                            * d.light_intensity
                                            * 2.5)
                                            .into(),
                                        unlit: true,
                                        ..default()
                                    }),
                                    transform: Transform::from_matrix(
                                        Mat4::from_scale_rotation_translation(
                                            voxel_scale,
                                            Quat::IDENTITY,
                                            translation,
                                        ),
                                    ),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(PointLightBundle {
                                        point_light: PointLight {
                                            color: light_color,
                                            intensity: d.light_intensity
                                                * 1100.0,
                                            range: d.light_range,
                                            radius: 0.5 * d.light_size,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        }
                    });
            });
    }

    // ---- Grids ----
    for d in &config.grids {
        // XPM headers take the form "20 20 2 1", "16 16 4 1", etc.
        const XPM_TYPE_HEADER_OFFSET: usize = 1;
        const XPM_INFO_HEADER_OFFSET: usize = 1 + XPM_TYPE_HEADER_OFFSET;

        let mut f = open_local_file(&d.pixmap_path);
        let mut file_contents = String::new();

        f.read_to_string(&mut file_contents)
            .expect("Failed to read file");

        let normalized_line_endings = &str::replace(
            &str::replace(&file_contents[..], "\r\n", "\n")[..],
            "\r",
            "\n",
        )[..];
        let xpm_data =
            normalized_line_endings.split('\n').collect::<Vec<&str>>();
        let header: Vec<&str> = xpm_data[1].split_ascii_whitespace().collect();
        let width: usize = header[0].parse().unwrap();
        let height: usize = header[1].parse().unwrap();
        let palette_size: usize = header[2].parse().unwrap();
        let mut palette = HashMap::with_capacity(palette_size);

        // Map palette indices to color materials.
        for i in 1..=palette_size {
            // XPM palette entries take the form " \tc None", ".\tc #000000",
            // etc.
            let palette_row = xpm_data[i + XPM_TYPE_HEADER_OFFSET];
            let palette_index = palette_row.chars().next().unwrap();
            let color_value =
                palette_row.split_ascii_whitespace().last().unwrap();

            match color_value {
                "None" | "none" => {},
                _ => {
                    // Strip '#' off "#RRGGBB" before converting it to a Color.
                    let hex_color = color_value.strip_prefix('#').unwrap();
                    palette.insert(
                        palette_index,
                        materials.add(StandardMaterial {
                            base_color: Srgba::hex(hex_color).unwrap().into(),
                            perceptual_roughness: d.roughness,
                            // metallic: 1.0,
                            ..default()
                        }),
                    );
                },
            };
        }

        // Ensure that the largest dimension will be scaled into [0, 1].
        let scale_factor = width.max(height) as f32;
        let voxel_scale = Vec3::splat(d.voxel_scale / scale_factor);
        let width_minus_one = (width - 1) as f32;
        let width_offset = width_minus_one * 0.5;
        let height_minus_one = (height - 1) as f32;
        let height_offset = height_minus_one * 0.5;

        commands
            .spawn(PbrBundle {
                transform: d.transforms.to_transform(),
                ..default()
            })
            .with_children(|parent| {
                // Voxelize the 2D image into a 3D grid.
                for h in 0..height {
                    let row =
                        xpm_data[h + palette_size + XPM_INFO_HEADER_OFFSET];

                    for w in 0..width {
                        // Convert each pixel to a voxel with the same color.
                        let palette_index = row.chars().nth(w).unwrap();
                        let Some(material) = palette.get(&palette_index) else {
                            continue;
                        };
                        let mut voxel = parent.spawn(PbrBundle {
                            transform: Transform::from_matrix(
                                Mat4::from_scale_rotation_translation(
                                    voxel_scale,
                                    Quat::IDENTITY,
                                    Vec3::new(
                                        (w as f32 - width_offset)
                                            / (width as f32),
                                        0.0,
                                        (h as f32 - height_offset)
                                            / (height as f32),
                                    ),
                                ),
                            ),
                            mesh: unit_cube.clone(),
                            material: material.clone(),
                            ..default()
                        });

                        // Add an optional animation to the new voxel.
                        let Some(animation) = d.animation else {
                            continue;
                        };
                        voxel.insert(WaveVoxel {
                            animation,
                            grid_position_2d: Vec2::new(
                                w as f32 / width_minus_one,
                                h as f32 / height_minus_one,
                            ),
                        });
                    }
                }
            });
    }
}

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_keyboard_input,
                automatically_rotate_on_local_axis,
                animate_wave_voxels,
            ),
        )
        .add_systems(Startup, spawn_demo_scene);
    }
}
