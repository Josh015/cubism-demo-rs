use crate::prelude::*;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum WaveVoxelAnimation {
    Ripple,
    Wave,
}

#[derive(Component)]
pub struct WaveVoxel {
    pub animation: WaveVoxelAnimation,
    pub grid_position_2d: Vec2,
}

pub fn animate_wave_voxels_system(
    config: Res<DemoConfig>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &WaveVoxel)>,
) {
    let wave_simulation = (config.wave_voxel_speed
        * time.time_since_startup().as_secs_f32())
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
