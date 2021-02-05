use std::{cmp, collections::HashMap};

use bevy::prelude::*;

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

/// Animate all grid voxel entities based on their movement type.
fn animate_grid_voxels(time: Res<Time>, mut query: Query<(&mut Transform, &mut GridVoxel)>) {
    for (mut transform, mut voxel) in query.iter_mut() {
        match voxel.movement_type {
            GridVoxelMovementType::Ripple => {
                voxel.wave_movement = (voxel.wave_movement + (1.0 * time.delta_seconds()))
                    % (2.0 * std::f32::consts::PI);
                transform.translation.y = (voxel.wave_movement + 10.0 * (voxel.x + voxel.y)).sin() * 0.025;
            }
            GridVoxelMovementType::Wave => {
                voxel.wave_movement = (voxel.wave_movement + (1.0 * time.delta_seconds()))
                    % (2.0 * std::f32::consts::PI);
                transform.translation.y = ((voxel.wave_movement + 10.0 * voxel.x).sin()
                    + (voxel.wave_movement + 10.0 * voxel.y).sin())
                    * 0.025;
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
