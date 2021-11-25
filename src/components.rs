use bevy::{core::Time, math::Vec2, prelude::*};
use serde::Deserialize;

pub struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        let config: ComponentsConfig =
            crate::files::load_config_from_file("assets/config/components.ron");

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

    for (mut transform, wave_voxel) in query.iter_mut() {
        let waves = match wave_voxel.wave_voxel_type {
            WaveVoxelType::Ripple => (wave_simulation.0
                + config.wave_voxel_tiling
                    * (wave_voxel.grid_position_2d.x
                        + wave_voxel.grid_position_2d.y))
                .sin(),
            WaveVoxelType::Wave => {
                (wave_simulation.0
                    + config.wave_voxel_tiling * wave_voxel.grid_position_2d.x)
                    .sin()
                    + (wave_simulation.0
                        + config.wave_voxel_tiling
                            * wave_voxel.grid_position_2d.y)
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
    // Rotate the child entity around its local y-axis.
    let rotation = Quat::from_axis_angle(
        Vec3::Y,
        config.auto_rotate_entity_speed * time.delta_seconds(),
    );

    for (mut transform, _) in query.iter_mut() {
        transform.rotate(rotation);
    }
}
