mod components;
mod files;

use bevy::{
    ecs::prelude::*,
    input::Input,
    math::*,
    pbr2::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    prelude::{
        bevy_main, App, AssetServer, Assets, BuildChildren, KeyCode,
        MeshBundle, Transform,
    },
    render2::{
        camera::{Camera, PerspectiveCameraBundle},
        color::Color,
        mesh::{shape, Mesh},
        view::Msaa,
    },
    window::WindowDescriptor,
    PipelinedDefaultPlugins,
};
use components::*;
use rand::distributions::{Distribution, Uniform};
use serde::Deserialize;
use std::{collections::HashMap, io::Read};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

#[derive(Debug, Deserialize)]
struct DemoConfig {
    title: String,
    width: u32,
    height: u32,
    instructions: String,
    cameras: Vec<Srt>,
    pillars: Vec<PillarConfig>,
    light_rings: Vec<LightRingConfig>,
    grids: Vec<GridConfig>,
}

#[derive(Debug, Deserialize)]
struct PillarConfig {
    color: Color,
    transforms: Srt,
}

#[derive(Debug, Deserialize)]
struct LightRingConfig {
    light_intensity: f32,
    lights_count: u32,
    height: f32,
    inner_radius: f32,
    outer_radius: f32,
    min_color: Color,
    max_color: Color,
    light_size: f32,
    light_range: f32,
    transforms: Srt,
}

#[derive(Debug, Deserialize)]
struct GridConfig {
    voxel_scale: f32,
    animation: Option<WaveVoxelAnimation>,
    roughness: f32,
    pixmap_path: String,
    transforms: Srt,
}

#[derive(Debug, Deserialize)]
struct Srt {
    scale: (f32, f32, f32),
    rotations: Vec<(f32, f32, f32, f32)>,
    translation: (f32, f32, f32),
}

impl Srt {
    fn to_transform(&self) -> Transform {
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::new(self.scale.0, self.scale.1, self.scale.2),
            self.rotations
                .iter()
                .map(|b| {
                    Quat::from_axis_angle(
                        Vec3::new(b.0, b.1, b.2),
                        b.3.to_radians(),
                    )
                })
                .fold(Quat::IDENTITY, |a, b| a * b)
                .normalize(),
            Vec3::new(
                self.translation.0,
                self.translation.1,
                self.translation.2,
            ),
        ))
    }
}

fn setup(
    config: Res<DemoConfig>,
    mut commands: Commands,
    // asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // ---- Camera ----
    commands
        // Camera
        .spawn_bundle(PerspectiveCameraBundle {
            transform: config.cameras[0].to_transform(),
            ..Default::default()
        });

    commands
        // Light
        .spawn_bundle(MeshBundle {
            transform: Transform::from_translation(Vec3::new(-4.0, 6.0, 4.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(PointLightBundle {
                point_light: PointLight {
                    range: 20.0,
                    intensity: 2500.0,
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    // commands
    //     .spawn_bundle(UiCameraBundle::default())
    //     // root node
    //     .commands()
    //     .spawn_bundle(NodeBundle {
    //         style: Style {
    //             position_type: PositionType::Absolute,
    //             position: Rect {
    //                 left: Val::Px(10.0),
    //                 top: Val::Px(10.0),
    //                 ..Default::default()
    //             },
    //             ..Default::default()
    //         },
    //         material: color_materials.add(Color::NONE.into()),
    //         ..Default::default()
    //     })
    //     .with_children(|parent| {
    //         parent.spawn_bundle(TextBundle {
    //             text: Text::with_section(
    //                 config.instructions.to_string(),
    //                 TextStyle {
    //                     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                     font_size: 40.0,
    //                     color: Color::rgb(0.8, 0.8, 0.8),
    //                 },
    //                 Default::default(),
    //             ),
    //             ..Default::default()
    //         });
    //     });

    // ---- Pillars ----
    for d in config.pillars.iter() {
        commands.spawn_bundle(PbrBundle {
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
    let unit_sphere = meshes.add(Mesh::from(shape::Icosphere {
        radius: 0.5,
        subdivisions: 2,
    }));
    let axis_randomizer = Uniform::from(-1f32..=1f32);
    let color_randomizer = Uniform::from(0f32..=1f32);

    for d in config.light_rings.iter() {
        let voxel_scale = Vec3::splat(d.light_size);
        let mut rng = rand::thread_rng();
        let radius_randomizer = Uniform::from(d.inner_radius..=d.outer_radius);
        let height_randomizer =
            Uniform::from((-0.5 * d.height)..=(0.5 * d.height));

        commands
            .spawn_bundle(PbrBundle {
                transform: d.transforms.to_transform(),
                ..Default::default()
            })
            .with_children(|parent| {
                // Light ring must be a child component so it can rotate around
                // its own local axis.
                parent
                    .spawn_bundle(MeshBundle {
                        ..Default::default()
                    })
                    .insert(AutoRotateEntity)
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
                                .spawn_bundle(PbrBundle {
                                    mesh: unit_sphere.clone(),
                                    material: materials.add(StandardMaterial {
                                        base_color: light_color
                                            * d.light_intensity.min(2.0f32),
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
                                    parent.spawn_bundle(PointLightBundle {
                                        point_light: PointLight {
                                            color: light_color,
                                            intensity: d.light_intensity,
                                            range: d.light_range,
                                            radius: 0.5 * d.light_size,
                                            shadows_enabled: false,
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
    for d in config.grids.iter() {
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
            normalized_line_endings.split("\n").collect::<Vec<&str>>();
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
            let palette_index: char = palette_row.chars().next().unwrap();
            let color_value: &str =
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
            .spawn_bundle(MeshBundle {
                transform: d.transforms.to_transform(),
                // mesh: cube.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                for h in 0..height {
                    let row =
                        xpm_data[h + palette_size + XPM_INFO_HEADER_OFFSET];

                    for w in 0..width {
                        let palette_index = row.chars().nth(w).unwrap();

                        if let Some(material) = palette.get(&palette_index) {
                            let mut voxel = parent.spawn_bundle(PbrBundle {
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

                            if let Some(animation) = d.animation {
                                voxel.insert(WaveVoxel {
                                    animation,
                                    grid_position_2d: Vec2::new(
                                        w as f32 / width_minus_one,
                                        h as f32 / height_minus_one,
                                    ),
                                });
                            }
                        }
                    }
                }
            });
    }
}

fn keyboard_input(
    config: Res<DemoConfig>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (mut transform, _) in query.iter_mut() {
        // Front
        if keyboard_input.just_pressed(KeyCode::Key1) {
            *transform = config.cameras[0].to_transform();
        }

        // Right
        if keyboard_input.just_released(KeyCode::Key2) {
            *transform = config.cameras[1].to_transform();
        }

        // Left
        if keyboard_input.just_released(KeyCode::Key3) {
            *transform = config.cameras[2].to_transform();
        }

        // Top
        if keyboard_input.just_released(KeyCode::Key4) {
            *transform = config.cameras[3].to_transform();
        }
    }
}

#[bevy_main]
fn main() {
    let config: DemoConfig =
        files::load_config_from_file("assets/config/demo.ron");

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: config.title.clone(),
            width: config.width as f32,
            height: config.height as f32,
            ..Default::default()
        })
        .add_plugins(PipelinedDefaultPlugins)
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.
        // system())
        .insert_resource(config)
        .add_plugin(ComponentsPlugin)
        .add_startup_system(setup.system())
        .add_system(keyboard_input.system())
        .run();
}
