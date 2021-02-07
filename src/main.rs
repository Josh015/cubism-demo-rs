use bevy::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

mod grids;
use grids::*;

mod main_camera;
use main_camera::*;

mod light_rings;
use light_rings::*;

mod pillars;
use pillars::*;

mod shared;
use shared::*;

#[bevy_main]
fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_resource(WindowDescriptor {
            title: "Cubism".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.system())
        .init_resource::<SharedData>()
        .add_plugin(GridsPlugin)
        .add_plugin(MainCameraPlugin)
        .add_plugin(PillarsPlugin)
        .add_plugin(LightRingsPlugin)
        .run();
}
