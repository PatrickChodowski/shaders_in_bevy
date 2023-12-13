use bevy::pbr::MaterialPlugin;
use bevy::pbr::ExtendedMaterial;
use bevy::pbr::StandardMaterial;
use bevy::prelude::{Plugin,App};

pub mod fire_material;



use fire_material::FireMaterialExtension;

pub struct CustomMaterialsPlugin;

impl Plugin for CustomMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<ExtendedMaterial<StandardMaterial, FireMaterialExtension>,>::default())
        ;
    }
}




