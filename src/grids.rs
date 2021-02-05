use std::{cmp, collections::HashMap};

use bevy::prelude::*;

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

#[derive(Clone, Copy)]
enum GridVoxelMovementType {
    Static,
    Ripple,
    Wave,
}

struct GridVoxel {
    movement_type: GridVoxelMovementType,
    wave_movement: f32,
    x: f32,
    y: f32,
}

/// Animate all grid voxel entities based on their movement type.
fn animate_grid_voxels(time: Res<Time>, mut query: Query<(&mut Transform, &mut GridVoxel)>) {
    for (mut transform, mut voxel) in query.iter_mut() {
        match voxel.movement_type {
            GridVoxelMovementType::Ripple => {
                voxel.wave_movement = (voxel.wave_movement + (1.0 * time.delta_seconds()))
                    % (2.0 * std::f32::consts::PI);
                transform.translation.y = (voxel.wave_movement + voxel.x + voxel.y).sin() * 0.025;
            }
            GridVoxelMovementType::Wave => {
                voxel.wave_movement = (voxel.wave_movement + (1.0 * time.delta_seconds()))
                    % (2.0 * std::f32::consts::PI);
                transform.translation.y = ((voxel.wave_movement + voxel.x).sin()
                    + (voxel.wave_movement + voxel.y).sin())
                    * 0.025;
            }
            _ => {}
        }
    }
}

/// Takes xpm image data and converts its pixels into a grid of cube entities.
fn spawn_voxel_grid(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube: &Handle<Mesh>,
    voxel_scale: f32,
    xpm_data: &[&str],
    movement_type: GridVoxelMovementType,
    transform: Transform,
) {
    // Build palette as hashmap of chars to materials
    let parts: Vec<&str> = xpm_data[0].split(" ").collect();
    let width: usize = parts[0].parse().unwrap();
    let height: usize = parts[1].parse().unwrap();
    let palette_size: usize = parts[2].parse().unwrap();
    let mut palette = HashMap::with_capacity(palette_size);

    for i in 1..=palette_size {
        let row = xpm_data[i];
        let palette_index: char = row.chars().nth(0).unwrap();
        let hex_color: &str = row.split(" ").last().unwrap();

        let material = match hex_color {
            "None" => None,
            _ => {
                let parts: Vec<&str> = hex_color.split("#").collect();
                let color = Color::hex(parts[1]).unwrap();

                Some(materials.add(color.into()))
            }
        };

        palette.insert(palette_index, material);
    }

    // Ensure that the largest dimension will be scaled into [0, 1].
    let scale_factor = cmp::max(width, height) as f32;
    let voxel_scale = Vec3::splat(voxel_scale / scale_factor);
    let half_width = width as f32 * 0.5;
    let width_offset = half_width - 0.5;
    let half_height = height as f32 * 0.5;
    let height_offset = half_height - 0.5;

    commands
        .spawn(PbrBundle {
            transform,
            // mesh: cube.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            for h in 0..height {
                let row = xpm_data[h + palette_size + 1];

                for w in 0..width {
                    let palette_index = row.chars().nth(w).unwrap();

                    if let Some(material) = &palette[&palette_index] {
                        parent
                            .spawn(PbrBundle {
                                transform: Transform::from_matrix(
                                    Mat4::from_scale_rotation_translation(
                                        voxel_scale,
                                        Quat::identity(),
                                        Vec3::new(
                                            (w as f32 - width_offset) / (width as f32),
                                            0.0,
                                            (h as f32 - height_offset) / (height as f32),
                                        ),
                                    ),
                                ),
                                mesh: cube.clone(),
                                material: material.clone(),
                                ..Default::default()
                            })
                            .with(GridVoxel {
                                wave_movement: 0.0,
                                movement_type,
                                x: w as f32 / (width - 1) as f32,
                                y: h as f32 / (height - 1) as f32,
                            });
                    }
                }
            }
        });
}

fn create_scene(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load cube mesh
    let cube = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));

    // ---- Voxel grids ----
    // Sprite
    let rotation1 = Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians());
    let rotation2 = Quat::from_axis_angle(Vec3::unit_z(), 45f32.to_radians());

    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        1.0,
        &SPRITE_XPM,
        GridVoxelMovementType::Static,
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            Vec3::splat(0.55),
            (rotation1 * rotation2).normalize(),
            Vec3::zero(),
        )),
    );

    let voxel_scale = 0.87;
    let grid_scale = Vec3::splat(1.8);

    // Magenta ripple
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        voxel_scale,
        &MAGENTA_XPM,
        GridVoxelMovementType::Ripple,
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            grid_scale,
            Quat::from_axis_angle(Vec3::unit_z(), -90f32.to_radians()),
            Vec3::unit_x(),
        )),
    );

    // Orange ripple
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        voxel_scale,
        &ORANGE_XPM,
        GridVoxelMovementType::Ripple,
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            grid_scale,
            Quat::from_axis_angle(Vec3::unit_x(), 90f32.to_radians()),
            -Vec3::unit_z(),
        )),
    );

    // Blue wave
    spawn_voxel_grid(
        commands,
        &mut materials,
        &cube,
        voxel_scale,
        &BLUE_XPM,
        GridVoxelMovementType::Wave,
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            grid_scale,
            Quat::identity(),
            -Vec3::unit_y(),
        )),
    );
}

pub struct GridsPlugin;
impl Plugin for GridsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_scene.system())
            .add_system(animate_grid_voxels.system());
    }
}
