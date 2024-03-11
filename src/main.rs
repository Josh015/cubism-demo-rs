mod components;
mod serialization;
mod systems;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use serialization::*;
use systems::SystemsPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin};

fn main() {
    let config: Config = load_config_from_ron_file("assets/config/demo.ron");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.title.clone(),
                resolution: WindowResolution::new(config.width as f32, config.height as f32),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgb(0.35, 0.35, 0.35)))
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.
        // system())
        .insert_resource(config)
        .add_plugins(SystemsPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
