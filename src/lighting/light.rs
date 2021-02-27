use bevy::{core::Byteable, prelude::*};

/// A point light
#[derive(Debug, Reflect)]
#[reflect(Component)]
pub struct CustomPointLight {
    pub color: Color,
    pub radius: f32,
}

impl Default for CustomPointLight {
    fn default() -> Self {
        CustomPointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            radius: 1.0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct CustomPointLightRaw {
    pub pos: [f32; 4],
    pub color: [f32; 4],
}

unsafe impl Byteable for CustomPointLightRaw {}

impl CustomPointLightRaw {
    pub fn from(
        light: &CustomPointLight,
        global_transform: &GlobalTransform,
    ) -> CustomPointLightRaw {
        let (x, y, z) = global_transform.translation.into();
        CustomPointLightRaw {
            pos: [x, y, z, 1.0 / light.radius],
            color: light.color.into(),
        }
    }
}

// // Ambient light color.
// #[derive(Debug)]
// pub struct AmbientLight {
//     pub color: Color,
// }

// impl Default for AmbientLight {
//     fn default() -> Self {
//         Self {
//             color: Color::rgb(0.05, 0.05, 0.05),
//         }
//     }
// }
