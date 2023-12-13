use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .insert_resource(StandardMaterialResource::default())
        .add_systems(Update, egui_update)
        ;
    }
}


/// ref: https://docs.rs/bevy/latest/bevy/prelude/struct.StandardMaterial.html
#[derive(Resource)]
pub struct StandardMaterialResource {
    pub base_color:           Color,
    pub perceptual_roughness: f32, // Clamped to [0.089, 1.0] in shader, default is 0.5 
    pub metallic:             f32, // [0.0, 1.0] default is 0.0
    pub reflectance:          f32, // [0.0, 1.0] default is 0.5
}

impl Default for StandardMaterialResource {
    fn default() -> Self {
        StandardMaterialResource{
                                 base_color:           Color::WHITE, 
                                 perceptual_roughness: 0.5, 
                                 metallic:             0.0,
                                 reflectance:          0.5
                                }
    }
}



pub fn egui_update(mut contexts:                 EguiContexts,
                   mut occupied_screen_space:    ResMut<OccupiedScreenSpace>,
                   mut smr:                      ResMut<StandardMaterialResource>){

    let ctx = contexts.ctx_mut();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
          .resizable(true)
          .show(ctx, |ui| {

            ui.separator();

            let mut base_color_array: [f32; 4] = smr.base_color.into();
            ui.color_edit_button_rgba_unmultiplied(&mut base_color_array);
            smr.base_color = base_color_array.into();

            ui.columns(2, |columns| {
              columns[0].label("Rougness");
              columns[0].label("Metallic");
              columns[0].label("Reflectance");
              columns[1].add(egui::DragValue::new(&mut smr.perceptual_roughness).speed(0.1));
              columns[1].add(egui::DragValue::new(&mut smr.metallic).speed(0.1));
              columns[1].add(egui::DragValue::new(&mut smr.reflectance).speed(0.1));
            });

          })
          .response
          .rect
          .width();
    }



#[derive(Default, Resource, Debug)]
pub struct OccupiedScreenSpace {
    pub _left: f32,
    pub _top: f32,
    pub right: f32,
    pub _bottom: f32,
}
