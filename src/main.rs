mod demo;
mod files;

use bevy::{
    ecs::prelude::*,
    input::Input,
    math::*,
    pbr2::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    prelude::{
        bevy_main, App, AssetServer, Assets, BuildChildren, KeyCode,
        MeshBundle, Transform,
    },
    render2::{
        camera::{Camera, PerspectiveCameraBundle, PerspectiveProjection},
        color::Color,
        mesh::{shape, Mesh},
        view::Msaa,
    },
    window::WindowDescriptor,
    PipelinedDefaultPlugins,
};
use demo::*;
use rand::distributions::{Distribution, Uniform};
use serde::Deserialize;
use std::{collections::HashMap, io::Read};
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
        .add_plugins(PipelinedDefaultPlugins)
        // .add_plugin(PrintDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_system(PrintDiagnosticsPlugin::print_diagnostics_system.
        // system())
        .insert_resource(config)
        .init_resource::<WaveSimulation>()
        .add_startup_system(demo::setup)
        .add_system(demo::keyboard_input_system)
        .add_system(auto_rotate_entity::rotate_on_local_axis_system)
        .add_system(wave_voxel::animate_wave_voxels_system)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}
