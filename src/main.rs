mod auto_rotate_entity;
mod config;
mod demo;
mod files;
mod wave_voxel;

mod prelude {
    pub use crate::{auto_rotate_entity::*, config::*, demo::*, wave_voxel::*};
    pub use bevy::prelude::*;
}

use crate::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use rand::{distributions::Uniform, prelude::Distribution};
use std::{collections::HashMap, io::Read};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

fn main() {
    let config: DemoConfig =
        files::load_config_from_file("assets/config/demo.ron");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.title.clone(),
                resolution: WindowResolution::new(config.width as f32, config.height as f32),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::default())
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.
        // system())
        .insert_resource(config)
        .add_startup_system(setup)
        .add_systems((demo::keyboard_input_system, auto_rotate_entity::rotate_on_local_axis_system, wave_voxel::animate_wave_voxels_system, bevy::window::close_on_esc))
        .run();
}

pub fn setup(
    config: Res<DemoConfig>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // ---- Camera ----
    commands.spawn(Camera3dBundle {
        transform: config.cameras[0].to_transform(),
        ..Default::default()
    });

    // ---- Light ----
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(-4.0, 6.0, 4.0)),
        point_light: PointLight {
            range: 20.0,
            intensity: 2500.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // ---- UI ----
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    config.instructions.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                ),
                ..Default::default()
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
                ..Default::default()
            }),
            ..Default::default()
        });
    }

    // ---- Light Rings ----
    let unit_sphere = meshes.add(Mesh::from(shape::UVSphere {
        radius: 0.5,
        sectors: 30,
        stacks: 30,
    }));
    let axis_randomizer = Uniform::from(-1f32..=1f32);
    let color_randomizer = Uniform::from(0f32..=1f32);

    for d in &config.light_rings {
        let voxel_scale = Vec3::splat(d.light_size);
        let mut rng = rand::thread_rng();
        let radius_randomizer = Uniform::from(d.inner_radius..=d.outer_radius);
        let height_randomizer =
            Uniform::from((-0.5 * d.height)..=(0.5 * d.height));

        commands
            .spawn(PbrBundle {
                transform: d.transforms.to_transform(),
                ..Default::default()
            })
            .with_children(|parent| {
                // Light ring must be a child component so it can rotate around
                // its own local axis.
                parent
                    .spawn((AutoRotateEntity, PbrBundle::default()))
                    .with_children(|parent| {
                        for _i in 0..d.lights_count {
                            // HACK: Force linear color interpolation.
                            let light_color =
                                Color::from(Vec4::from(d.min_color).lerp(
                                    Vec4::from(d.max_color),
                                    color_randomizer.sample(&mut rng),
                                ));
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
                                        base_color: light_color
                                            * d.light_intensity
                                            * 0.8,
                                        unlit: true,
                                        ..Default::default()
                                    }),
                                    transform: Transform::from_matrix(
                                        Mat4::from_scale_rotation_translation(
                                            voxel_scale,
                                            Quat::IDENTITY,
                                            translation,
                                        ),
                                    ),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(PointLightBundle {
                                        point_light: PointLight {
                                            color: light_color,
                                            intensity: d.light_intensity,
                                            range: d.light_range,
                                            radius: 0.5 * d.light_size,
                                            ..Default::default()
                                        },
                                        ..Default::default()
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

        let mut f = files::open_local_file(&d.pixmap_path);
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
                            base_color: Color::hex(hex_color).unwrap(),
                            perceptual_roughness: d.roughness,
                            // metallic: 1.0,
                            ..Default::default()
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
                ..Default::default()
            })
            .with_children(|parent| {
                // Voxelize the 2D image into a 3D grid.
                for h in 0..height {
                    let row =
                        xpm_data[h + palette_size + XPM_INFO_HEADER_OFFSET];

                    for w in 0..width {
                        // Convert each pixel to a voxel with the same color.
                        let palette_index = row.chars().nth(w).unwrap();
                        let Some(material) = palette.get(&palette_index) else { continue };
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
                            ..Default::default()
                        });

                        // Add an optional animation to the new voxel.
                        let Some(animation) = d.animation else { continue };
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
