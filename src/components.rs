use std::{fs::File, path::PathBuf};

use bevy::{core::Time, math::Vec2, prelude::*};
use ron::de::from_reader;
use serde::Deserialize;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets/config/components.ron");
        let f = File::open(&input_path).expect("Failed opening config file");

        let config: ComponentsConfig = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);

                std::process::exit(1);
            },
        };

        app.insert_resource(config)
            .init_resource::<WaveSimulation>()
            .add_system(animate_wave_voxels.system())
            .add_system(auto_rotate_entity.system());
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum WaveVoxelType {
    Ripple,
    Wave,
}

// #[derive(Component)]
pub struct WaveVoxel {
    pub wave_voxel_type: WaveVoxelType,
    pub grid_position_2d: Vec2,
}

// #[derive(Component)]
pub struct AutoRotateEntity;

#[derive(Debug, Deserialize)]
struct ComponentsConfig {
    wave_voxel_tiling: f32,
    wave_voxel_speed: f32,
    wave_voxel_height: f32,
    auto_rotate_entity_speed: f32,
}

#[derive(Default)]
struct WaveSimulation(f32);

fn animate_wave_voxels(
    config: Res<ComponentsConfig>,
    time: Res<Time>,
    mut wave_simulation: ResMut<WaveSimulation>,
    mut query: Query<(&mut Transform, &WaveVoxel)>,
) {
    wave_simulation.0 += config.wave_voxel_speed * time.delta_seconds();
    wave_simulation.0 %= std::f32::consts::TAU;

    for (mut transform, grid_voxel) in query.iter_mut() {
        let waves = match grid_voxel.wave_voxel_type {
            WaveVoxelType::Ripple => (wave_simulation.0
                + config.wave_voxel_tiling
                    * (grid_voxel.grid_position_2d.x
                        + grid_voxel.grid_position_2d.y))
                .sin(),
            WaveVoxelType::Wave => {
                (wave_simulation.0
                    + config.wave_voxel_tiling * grid_voxel.grid_position_2d.x)
                    .sin()
                    + (wave_simulation.0
                        + config.wave_voxel_tiling
                            * grid_voxel.grid_position_2d.y)
                        .sin()
            },
        };

        transform.translation.y = 0.5 * config.wave_voxel_height * waves;
    }
}

fn auto_rotate_entity(
    config: Res<ComponentsConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &AutoRotateEntity)>,
) {
    // Rotate the light rings.
    let rotation = Quat::from_axis_angle(
        Vec3::Y,
        config.auto_rotate_entity_speed * time.delta_seconds(),
    );

    for (mut transform, _) in query.iter_mut() {
        transform.rotate(rotation);
    }
}
