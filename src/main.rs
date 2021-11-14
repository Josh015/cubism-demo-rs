use bevy::{
    prelude::*,
    render::{camera::Camera, mesh::shape},
};
use lazy_static::*;
use rand::distributions::{Distribution, Uniform};
use std::{cmp, collections::HashMap};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

const INSTRUCTIONS: &str = r#"
---- Views ----
1: Front
2: Right
3: Left
4: Top
"#;
// ---- Modes ----
// TAB: Debug view
// "#;

const RING_ROTATION_SPEED: f32 = 1.0;

const GRID_WAVE_TILING: f32 = 10.0;
const GRID_WAVE_SPEED: f32 = 2.0;
const WALL_VOXEL_SCALE: f32 = 0.87;
const WALL_GRID_SCALE: f32 = 1.8;

const SPRITE_XPM: [&str; 21] = [
    "16 16 4 1",
    " 	c None",
    ".	c #FFA044",
    "+	c #F84848",
    "@	c #5C40E4",
    " ..   ++++   .. ",
    " ... +@@@@+ ... ",
    " @@ +@@@@@@+ @@ ",
    " @@.@.+..+.@.@@ ",
    " @@...@..@...@@ ",
    "  @@........@@  ",
    "  @@@..@@..@@@  ",
    "  @@@+.@@.+@@@  ",
    "   @++++++++@   ",
    "   @++++++++@   ",
    "   +++@@@@+++   ",
    "   @@@@++@@@@   ",
    "   +++@@@@+++   ",
    "   ++++++++++   ",
    "    @@@  @@@    ",
    "    @@@  @@@    ",
];

const MAGENTA_XPM: [&str; 22] = [
    "20 20 1 1",
    ".	c #E61A80",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
];

const ORANGE_XPM: [&str; 22] = [
    "20 20 1 1",
    ".	c #E6801A",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
];

const BLUE_XPM: [&str; 23] = [
    "20 20 2 1",
    " 	c None",
    ".	c #1A80E6",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "........    ........",
    "........    ........",
    "........    ........",
    "........    ........",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
    "....................",
];

lazy_static! {
    static ref DEFAULT_CAMERA_TRANSFORMS: (Vec3, Quat) = {
        (
            Vec3::new(-3.0, 2.25, 3.0),
            (Quat::from_axis_angle(Vec3::Y, -45f32.to_radians())
                * Quat::from_axis_angle(Vec3::X, -30f32.to_radians()))
            .normalize(),
        )
    };

    static ref PILLAR_DESCRIPTIONS: [Mat4; 4] = {
        [
            // Pedestal
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.34, 0.7, 0.34),
                Quat::IDENTITY,
                Vec3::new(0.0, -0.75, 0.0),
            ),
            // X pillar
            Mat4::from_scale_rotation_translation(
                Vec3::new(2.0, 0.125, 0.125),
                Quat::IDENTITY,
                Vec3::new(-0.05, -1.0, -1.0),
            ),
            // Y pillar
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 2.0, 0.125),
                Quat::IDENTITY,
                Vec3::new(1.0, 0.05, -1.0),
            ),
            // Z pillar
            Mat4::from_scale_rotation_translation(
                Vec3::new(0.125, 0.125, 2.0),
                Quat::IDENTITY,
                Vec3::new(1.0, -1.0, 0.05),
            ),
        ]
    };

    static ref LIGHT_RING_DESCRIPTIONS: [LightRingDesc; 3] = {
        [
            // Cyan light ring
            LightRingDesc {
                // lights_count: 85,
                lights_count: 3,
                light_size: 0.125,
                height: 0.25,
                inner_radius: 0.25,
                outer_radius: 0.7,
                min_color: Color::rgb(0.05, 0.2, 0.3),
                max_color: Color::rgb(0.1, 0.5, 0.7),
                transform: Mat4::from_translation(-0.55 * Vec3::Y),
            },
            // Orange light ring
            LightRingDesc {
                // lights_count: 85,
                lights_count: 3,
                light_size: 0.125,
                height: 0.125,
                inner_radius: 0.25,
                outer_radius: 0.7,
                min_color: Color::rgb(0.4, 0.3, 0.05),
                max_color: Color::rgb(0.6, 0.5, 0.1),
                transform: Mat4::from_rotation_translation(
                    Quat::from_axis_angle(Vec3::X, 90f32.to_radians()),
                    -0.7 * Vec3::Z,
                ),
            },
            // Magenta light ring
            LightRingDesc {
                // lights_count: 85,
                lights_count: 3,
                light_size: 0.125,
                height: 0.125,
                inner_radius: 0.25,
                outer_radius: 0.7,
                min_color: Color::rgb(0.1, 0.1, 0.5),
                max_color: Color::rgb(0.6, 0.2, 0.7),
                transform: Mat4::from_rotation_translation(
                    Quat::from_axis_angle(Vec3::Z, -90f32.to_radians()),
                    0.7 * Vec3::X,
                ),
            },
        ]
    };

    static ref GRID_DESCRIPTIONS: [GridVoxelDesc; 4] = {
        [
            // Sprite
            GridVoxelDesc {
                voxel_scale: 1.0,
                xpm_data: &SPRITE_XPM,
                wave_height: 0.0,
                movement_type: GridVoxelMovementType::Static,
                roughness: 0.25,
                transform: Mat4::from_scale_rotation_translation(
                    Vec3::splat(0.55),
                    (Quat::from_axis_angle(Vec3::X, 90f32.to_radians())
                        * Quat::from_axis_angle(Vec3::Z, 45f32.to_radians()))
                    .normalize(),
                    -0.125 * Vec3::Y,
                ),
            },
            // Magenta ripple
            GridVoxelDesc {
                voxel_scale: WALL_VOXEL_SCALE,
                xpm_data: &MAGENTA_XPM,
                wave_height: 0.06,
                movement_type: GridVoxelMovementType::Ripple,
                roughness: 0.0,
                transform: Mat4::from_scale_rotation_translation(
                    Vec3::splat(WALL_GRID_SCALE),
                    Quat::from_axis_angle(Vec3::Z, -90f32.to_radians()),
                    Vec3::X,
                ),
            },
            // Orange ripple
            GridVoxelDesc {
                voxel_scale: WALL_VOXEL_SCALE,
                xpm_data: &ORANGE_XPM,
                wave_height: 0.06,
                movement_type: GridVoxelMovementType::Ripple,
                roughness: 0.0,
                transform: Mat4::from_scale_rotation_translation(
                    Vec3::splat(WALL_GRID_SCALE),
                    (Quat::from_axis_angle(Vec3::X, 90f32.to_radians())
                        * Quat::from_axis_angle(Vec3::Z, 180f32.to_radians()))
                    .normalize(),
                    -Vec3::Z,
                ),
            },
            // Blue wave
            GridVoxelDesc {
                voxel_scale: WALL_VOXEL_SCALE,
                xpm_data: &BLUE_XPM,
                wave_height: 0.12,
                movement_type: GridVoxelMovementType::Wave,
                roughness: 0.0,
                transform: Mat4::from_scale_rotation_translation(
                    Vec3::splat(WALL_GRID_SCALE),
                    Quat::from_axis_angle(Vec3::Y, -90f32.to_radians()),
                    -Vec3::Y,
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
    light_size: f32,
    transform: Mat4,
}

#[derive(Component)]
struct LightRing;

#[derive(Component)]
struct LightRingVoxel;

struct GridVoxelDesc {
    voxel_scale: f32,
    wave_height: f32,
    movement_type: GridVoxelMovementType,
    transform: Mat4,
    roughness: f32,
    xpm_data: &'static [&'static str],
}

#[derive(Clone, Copy)]
enum GridVoxelMovementType {
    Static,
    Ripple,
    Wave,
}

#[derive(Component)]
struct GridVoxel {
    movement_type: GridVoxelMovementType,
    wave_height: f32,
    wave_movement: f32,
    grid_x: f32,
    grid_y: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let unit_cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // ---- Camera ----
    commands
        // Camera
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                DEFAULT_CAMERA_TRANSFORMS.1,
                DEFAULT_CAMERA_TRANSFORMS.0,
            )),
            ..Default::default()
        });

    commands
        // Light
        .spawn_bundle(MeshBundle {
            transform: Transform::from_translation(Vec3::new(-4.0, 6.0, 4.0)),
            ..Default::default()
        })
        .insert(PointLight {
            range: 20.0,
            intensity: 200.0,
            ..Default::default()
        });

    commands
        .spawn_bundle(UiCameraBundle::default())
        // root node
        .commands()
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: color_materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    INSTRUCTIONS.to_string(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.8, 0.8, 0.8),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    // ---- Pillars ----
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.7, 0.7, 0.7),
        roughness: 1.0,
        // metallic: 1.0,
        ..Default::default()
    });

    for d in PILLAR_DESCRIPTIONS.iter() {
        commands.spawn_bundle(PbrBundle {
            transform: Transform::from_matrix(*d),
            material: material.clone(),
            mesh: unit_cube.clone(),
            ..Default::default()
        });
    }

    // ---- Light Rings ----
    for d in LIGHT_RING_DESCRIPTIONS.iter() {
        let voxel_scale = Vec3::splat(d.light_size);
        let mut rng = rand::thread_rng();
        let color_randomizer = Uniform::from(0f32..=1f32);
        let radius_randomizer = Uniform::from(d.inner_radius..=d.outer_radius);
        let height_randomizer =
            Uniform::from((-0.5 * d.height)..=(0.5 * d.height));
        let x_randomizer = Uniform::from(-1f32..=1f32);
        let z_randomizer = Uniform::from(-1f32..=1f32);

        commands
            .spawn_bundle(PbrBundle {
                transform: Transform::from_matrix(d.transform),
                ..Default::default()
            })
            .insert(LightRing)
            .with_children(|parent| {
                for _i in 0..d.lights_count {
                    let light_color = Color::from(
                        1.0 * Vec4::from(d.min_color).lerp(
                            Vec4::from(d.max_color),
                            color_randomizer.sample(&mut rng),
                        ),
                    );
                    let mut translation = Vec3::new(
                        x_randomizer.sample(&mut rng),
                        0.0,
                        z_randomizer.sample(&mut rng),
                    );

                    translation = translation.normalize()
                        * radius_randomizer.sample(&mut rng);
                    translation.y = height_randomizer.sample(&mut rng);

                    let light_intensity = std::f32::consts::PI;

                    parent
                        .spawn_bundle(PbrBundle {
                            mesh: unit_cube.clone(),
                            material: materials.add(StandardMaterial {
                                base_color: light_color * light_intensity,
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
                        .insert(PointLight {
                            color: light_color,
                            intensity: light_intensity * 0.5,
                            // range: 0.5,
                            range: 1.0,
                            radius: 0.5 * d.light_size,
                        })
                        .insert(LightRingVoxel);
                }
            });
    }

    // ---- Grids ----
    for d in GRID_DESCRIPTIONS.iter() {
        // XPM headers take the form "20 20 2 1", "16 16 4 1", etc.
        let header: Vec<&str> = d.xpm_data[0].split(' ').collect();
        let width: usize = header[0].parse().unwrap();
        let height: usize = header[1].parse().unwrap();
        let palette_size: usize = header[2].parse().unwrap();
        let mut palette = HashMap::with_capacity(palette_size);

        // Map palette indices to color materials.
        for i in 1..=palette_size {
            // XPM palette entries take the form " \tc None", ".\tc #000000", etc.
            let palette_row = d.xpm_data[i];
            let palette_index: char = palette_row.chars().next().unwrap();
            let color_value: &str = palette_row.split(' ').last().unwrap();

            match color_value {
                "None" => {}
                _ => {
                    // Strip '#' off "#RRGGBB" before converting it to a Color.
                    let hex_color: String =
                        color_value.chars().skip(1).collect();
                    palette.insert(
                        palette_index,
                        materials.add(StandardMaterial {
                            base_color: Color::hex(hex_color).unwrap(),
                            roughness: d.roughness,
                            // metallic: 1.0,
                            ..Default::default()
                        }),
                    );
                }
            };
        }

        // Ensure that the largest dimension will be scaled into [0, 1].
        let scale_factor = cmp::max(width, height) as f32;
        let voxel_scale = Vec3::splat(d.voxel_scale / scale_factor);
        let width_minus_one = (width - 1) as f32;
        let width_offset = width_minus_one * 0.5;
        let height_minus_one = (height - 1) as f32;
        let height_offset = height_minus_one * 0.5;

        commands
            .spawn_bundle(MeshBundle {
                transform: Transform::from_matrix(d.transform),
                // mesh: cube.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                for h in 0..height {
                    let row = d.xpm_data[h + palette_size + 1];

                    for w in 0..width {
                        let palette_index = row.chars().nth(w).unwrap();

                        if let Some(material) = palette.get(&palette_index) {
                            parent
                                .spawn_bundle(PbrBundle {
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
                                })
                                .insert(GridVoxel {
                                    movement_type: d.movement_type,
                                    wave_height: d.wave_height,
                                    wave_movement: 0.0,
                                    grid_x: w as f32 / width_minus_one,
                                    grid_y: h as f32 / height_minus_one,
                                });
                        }
                    }
                }
            });
    }
}

fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>,
) {
    for (mut transform, _) in query.iter_mut() {
        // Front
        if keyboard_input.just_pressed(KeyCode::Key1) {
            transform.translation = DEFAULT_CAMERA_TRANSFORMS.0;
            transform.rotation = DEFAULT_CAMERA_TRANSFORMS.1;
        }

        // Right
        if keyboard_input.just_released(KeyCode::Key2) {
            transform.translation = Vec3::new(0.0, 0.0, 4.0);
            transform.rotation = Quat::IDENTITY;
        }

        // Left
        if keyboard_input.just_released(KeyCode::Key3) {
            transform.translation = Vec3::new(-4.0, 0.0, 0.0);
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, -90f32.to_radians());
        }

        // Top
        if keyboard_input.just_released(KeyCode::Key4) {
            transform.translation = Vec3::new(0.3, 4.0, -0.3);
            transform.rotation =
                (Quat::from_axis_angle(Vec3::X, -90f32.to_radians())
                    * Quat::from_axis_angle(Vec3::Z, -45f32.to_radians()))
                .normalize();
        }
    }
}

fn rotate_light_rings(
    time: Res<Time>,
    mut query: Query<(
        &mut Transform,
        Option<&LightRing>,
        Option<&LightRingVoxel>,
    )>,
) {
    // Rotate the light rings while rotating their voxels the opposite way.
    let rotation = Quat::from_axis_angle(
        Vec3::Y,
        RING_ROTATION_SPEED * time.delta_seconds(),
    );
    let inverse_rotation = rotation.inverse();

    for (mut transform, light_ring, light_ring_voxel) in query.iter_mut() {
        if light_ring.is_some() {
            transform.rotate(rotation);
        } else if light_ring_voxel.is_some() {
            transform.rotate(inverse_rotation);
        }
    }
}

fn animate_grid_voxels(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut GridVoxel)>,
) {
    for (mut transform, mut voxel) in query.iter_mut() {
        voxel.wave_movement = (voxel.wave_movement
            + (GRID_WAVE_SPEED * time.delta_seconds()))
            % (2.0 * std::f32::consts::PI);

        match voxel.movement_type {
            GridVoxelMovementType::Ripple => {
                transform.translation.y = 0.5
                    * voxel.wave_height
                    * (voxel.wave_movement
                        + GRID_WAVE_TILING * (voxel.grid_x + voxel.grid_y))
                        .sin();
            }
            GridVoxelMovementType::Wave => {
                transform.translation.y = 0.5
                    * voxel.wave_height
                    * (0.5
                        * ((voxel.wave_movement
                            + GRID_WAVE_TILING * voxel.grid_x)
                            .sin()
                            + (voxel.wave_movement
                                + GRID_WAVE_TILING * voxel.grid_y)
                                .sin()));
            }
            _ => {}
        }
    }
}

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Cubism".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system())
        .add_startup_system(setup.system())
        .add_system(keyboard_input.system())
        .add_system(rotate_light_rings.system())
        .add_system(animate_grid_voxels.system())
        .run();
}
