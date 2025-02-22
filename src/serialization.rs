use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, de::DeserializeOwned};
use std::{fs::File, path::PathBuf};

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum WaveVoxelAnimation {
    Ripple,
    Wave,
}

#[derive(Debug, Deserialize, Resource)]
pub struct Config {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub instructions: String,
    pub wave_voxel_tiling: f32,
    pub wave_voxel_speed: f32,
    pub wave_voxel_height: f32,
    pub auto_rotate_entity_speed: f32,
    pub cameras: Vec<Srt>,
    pub pillars: Vec<PillarConfig>,
    pub light_rings: Vec<LightRingConfig>,
    pub grids: Vec<GridConfig>,
}

#[derive(Debug, Deserialize)]
pub struct PillarConfig {
    pub color: Color,
    pub transforms: Srt,
}

#[derive(Debug, Deserialize)]
pub struct LightRingConfig {
    pub light_intensity: f32,
    pub lights_count: u32,
    pub height: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
    pub min_color: Color,
    pub max_color: Color,
    pub light_size: f32,
    pub light_range: f32,
    pub transforms: Srt,
}

#[derive(Debug, Deserialize)]
pub struct GridConfig {
    pub voxel_scale: f32,
    pub animation: Option<WaveVoxelAnimation>,
    pub roughness: f32,
    pub pixmap_path: String,
    pub transforms: Srt,
}

#[derive(Debug, Deserialize)]
pub struct Srt {
    pub scale: (f32, f32, f32),
    pub rotations: Vec<(f32, f32, f32, f32)>,
    pub translation: (f32, f32, f32),
}

impl Srt {
    pub fn to_transform(&self) -> Transform {
        Transform::from_matrix(Mat4::from_scale_rotation_translation(
            self.scale.into(),
            self.rotations
                .iter()
                .map(|b| {
                    Quat::from_axis_angle(
                        Vec3::new(b.0, b.1, b.2),
                        b.3.to_radians(),
                    )
                })
                .fold(Quat::IDENTITY, |a, b| a * b)
                .normalize(),
            self.translation.into(),
        ))
    }
}

pub fn open_local_file(path: &str) -> File {
    let input_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    File::open(input_path)
        .expect(&format!("Failed opening file: {:#?}", path)[..])
}

pub fn load_config_from_ron_file<T: DeserializeOwned>(path: &str) -> T {
    let f = open_local_file(path);

    match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Failed to load config: {}", e);

            std::process::exit(1);
        },
    }
}
