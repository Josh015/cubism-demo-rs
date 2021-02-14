use bevy::prelude::*;

mod grids;
use grids::*;

mod camera;
use camera::*;

mod light_rings;
use light_rings::*;

mod pillars;
use pillars::*;

mod shared;
use shared::*;

pub struct DemoPlugin;
impl Plugin for DemoPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SharedData>()
            .add_startup_system(camera_setup.system())
            .add_system(keyboard_input.system())
            .add_startup_system(spawn_voxel_grids.system())
            .add_system(animate_grid_voxels.system())
            .add_startup_system(spawn_voxel_light_rings.system())
            .add_system(animate_light_ring.system())
            .add_system(animate_light_ring_voxels.system())
            .add_startup_system(spawn_pillars.system());
    }
}
