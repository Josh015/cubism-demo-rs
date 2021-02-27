use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{renderer::RenderResources, shader::ShaderDefs},
};

/// A material with "standard" properties used in PBR lighting
#[derive(Debug, RenderResources, ShaderDefs, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
pub struct CustomMaterial {
    pub albedo: Color,
    #[shader_def]
    pub albedo_texture: Option<Handle<Texture>>,
    #[render_resources(ignore)]
    #[shader_def]
    pub unlit: bool,
}

impl Default for CustomMaterial {
    fn default() -> Self {
        CustomMaterial {
            albedo: Color::rgb(1.0, 1.0, 1.0),
            albedo_texture: None,
            unlit: false,
        }
    }
}

impl From<Color> for CustomMaterial {
    fn from(color: Color) -> Self {
        CustomMaterial {
            albedo: color,
            ..Default::default()
        }
    }
}

impl From<Handle<Texture>> for CustomMaterial {
    fn from(texture: Handle<Texture>) -> Self {
        CustomMaterial {
            albedo_texture: Some(texture),
            ..Default::default()
        }
    }
}
