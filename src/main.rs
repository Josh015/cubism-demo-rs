mod demo;
mod files;

use bevy::prelude::*;
use demo::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

#[bevy_main]
fn main() {
    let config: DemoConfig =
        files::load_config_from_file("assets/config/demo.ron");

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: config.title.clone(),
            width: config.width as f32,
            height: config.height as f32,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.
        // system())
        .insert_resource(config)
        .init_resource::<wave_voxel::WaveSimulation>()
        .add_startup_system(demo::setup)
        .add_system(demo::keyboard_input_system)
        .add_system(auto_rotate_entity::rotate_on_local_axis_system)
        .add_system(wave_voxel::animate_wave_voxels_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
