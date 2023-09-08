use bevy::prelude::{Component, Vec2};

use crate::serialization::WaveVoxelAnimation;

#[derive(Component)]
pub struct AutomaticRotation;

#[derive(Component)]
pub struct WaveVoxel {
    pub animation: WaveVoxelAnimation,
    pub grid_position_2d: Vec2,
}
