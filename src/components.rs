use bevy::{
    prelude::{Component, Vec2},
    render::view::Visibility,
    transform::components::Transform,
};

use crate::serialization::WaveVoxelAnimation;

#[derive(Component)]
#[require(Transform, Visibility)]
pub struct AutomaticRotation;

#[derive(Component)]
#[require(Transform)]
pub struct WaveVoxel {
    pub animation: WaveVoxelAnimation,
    pub grid_position_2d: Vec2,
}
