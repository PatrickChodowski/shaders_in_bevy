
mod gui;
use gui::{GUIPlugin, egui_update, StandardMaterialResource};

mod materials;
use materials::{CustomMaterialsPlugin, fire_material::FireMaterialExtension};

use bevy::prelude::*;
use bevy::pbr::{ExtendedMaterial,OpaqueRendererMethod};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CustomMaterialsPlugin)
        .add_plugins(GUIPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update.after(egui_update).run_if(resource_changed::<StandardMaterialResource>()))
        .run();
}

fn setup(
    mut commands:           Commands,
    mut meshes:             ResMut<Assets<Mesh>>,
    mut _materials:         ResMut<Assets<StandardMaterial>>,
    mut fire_materials:     ResMut<Assets<ExtendedMaterial<StandardMaterial, FireMaterialExtension>>>,
) {

    commands.spawn((MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0, subdivisions: 0})),
        // material: _materials.add(Color::rgb_u8(124, 144, 255).into()),

        material: fire_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::WHITE,
                opaque_render_method: OpaqueRendererMethod::Forward,
                ..default()
            },
            extension: FireMaterialExtension { quantize_steps: 6 },
        }),


        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    }, Display));



    commands.insert_resource(AmbientLight {color: Color::WHITE, brightness: 1.0});

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}



fn update(mut fire_materials:     ResMut<Assets<ExtendedMaterial<StandardMaterial, FireMaterialExtension>>>,
          d:                      Query<&Handle<ExtendedMaterial<StandardMaterial, FireMaterialExtension>>, With<Display>>,
          smr:                    Res<StandardMaterialResource>){

    for handle_mat in d.iter(){

        let mat = fire_materials.get_mut(handle_mat).unwrap();
        mat.base.base_color = smr.base_color;
        mat.base.perceptual_roughness = smr.perceptual_roughness;
        mat.base.metallic = smr.metallic;
        mat.base.reflectance = smr.reflectance;

    }

}

#[derive(Component)]
pub struct Display;
