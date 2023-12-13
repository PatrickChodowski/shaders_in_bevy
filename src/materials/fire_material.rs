use bevy::{
    pbr::MaterialExtension,
    prelude::*,
    render::render_resource::*,
};


#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct FireMaterialExtension {
    #[uniform(100)]
    pub quantize_steps: u32,
}

impl MaterialExtension for FireMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/fire_material.wgsl".into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        "shaders/fire_material.wgsl".into()
    }
}
