use bevy::{
    prelude::*,
    render::{pipeline::RenderPipeline, render_graph::base::MainPass},
};

use super::custom_pipeline::*;
use super::CustomMaterial;

#[derive(Bundle)]
pub struct CustomBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<CustomMaterial>,
    pub main_pass: MainPass,
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for CustomBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                CUSTOM_PIPELINE_HANDLE.typed(),
            )]),
            mesh: Default::default(),
            visible: Default::default(),
            material: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
