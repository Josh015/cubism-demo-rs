use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::PipelineDescriptor,
        shader::{ShaderStage, ShaderStages},
    },
};

pub const CUSTOM_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 13148362314012771392);

pub fn build_custom_pipeline(shaders: &mut Assets<Shader>) -> PipelineDescriptor {
    PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("custom.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("custom.frag"),
        ))),
    })
}
