use std::{cmp, collections::HashMap};

use bevy::prelude::*;

const GRID_WAVE_TILING: f32 = 10.0;
const GRID_WAVE_HEIGHT: f32 = 0.03;
const GRID_WAVE_SPEED: f32 = 2.0;

#[derive(Clone, Copy)]
pub enum GridVoxelMovementType {
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

/// Takes xpm image data and converts its pixels into a grid of cube entities.
pub fn spawn_voxel_grid(
    commands: &mut Commands,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    cube: &Handle<Mesh>,
    voxel_scale: f32,
    xpm_data: &[&str],
    movement_type: GridVoxelMovementType,
    transform: Mat4,
) {
    // XPM headers take the form "20 20 2 1", "16 16 4 1", etc.
    let header: Vec<&str> = xpm_data[0].split(" ").collect();
    let width: usize = header[0].parse().unwrap();
    let height: usize = header[1].parse().unwrap();
    let palette_size: usize = header[2].parse().unwrap();
    let mut palette = HashMap::with_capacity(palette_size);

    // Map palette indices to color materials.
    for i in 1..=palette_size {
        // XPM palette entries take the form " \tc None", ".\tc #000000", etc.
        let palette_row = xpm_data[i];
        let palette_index: char = palette_row.chars().nth(0).unwrap();
        let color_value: &str = palette_row.split(" ").last().unwrap();

        match color_value {
            "None" => {}
            _ => {
                // Strip '#' off "#RRGGBB" before converting it to a Color.
                let hex_color: String = color_value.chars().skip(1).collect();
                palette.insert(
                    palette_index,
                    materials.add(Color::hex(hex_color).unwrap().into()),
                );
            }
        };
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
            transform: Transform::from_matrix(transform),
            // mesh: cube.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            for h in 0..height {
                let row = xpm_data[h + palette_size + 1];

                for w in 0..width {
                    let palette_index = row.chars().nth(w).unwrap();

                    if let Some(material) = palette.get(&palette_index) {
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
                                movement_type,
                                wave_movement: 0.0,
                                x: w as f32 / (width - 1) as f32,
                                y: h as f32 / (height - 1) as f32,
                            });
                    }
                }
            }
        });
}

/// Animate all grid voxel entities based on their movement type.
fn animate_grid_voxels(time: Res<Time>, mut query: Query<(&mut Transform, &mut GridVoxel)>) {
    for (mut transform, mut voxel) in query.iter_mut() {
        voxel.wave_movement = (voxel.wave_movement + (GRID_WAVE_SPEED * time.delta_seconds()))
            % (2.0 * std::f32::consts::PI);

        match voxel.movement_type {
            GridVoxelMovementType::Ripple => {
                transform.translation.y = GRID_WAVE_HEIGHT
                    * (voxel.wave_movement + GRID_WAVE_TILING * (voxel.x + voxel.y)).sin();
            }
            GridVoxelMovementType::Wave => {
                transform.translation.y = GRID_WAVE_HEIGHT
                    * ((voxel.wave_movement + GRID_WAVE_TILING * voxel.x).sin()
                        + (voxel.wave_movement + GRID_WAVE_TILING * voxel.y).sin());
            }
            _ => {}
        }
    }
}

pub struct GridsPlugin;
impl Plugin for GridsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(animate_grid_voxels.system());
    }
}
