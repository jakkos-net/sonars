pub mod lang;
pub mod sound;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use lang::Program;
use sound::{SoundControl, SoundPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SoundPlugin)
        .init_resource::<CodeEditorData>()
        .add_system(coding_ui)
        .run();
}

#[derive(Resource)]
struct CodeEditorData {
    pub src: String,
    pub error_text: String,
}

impl Default for CodeEditorData {
    fn default() -> Self {
        Self {
            src: "Woo I'm some source code... Apparently...".into(),
            error_text: "".into(),
        }
    }
}

fn coding_ui(
    mut egui_context: ResMut<EguiContext>,
    mut editor_data: ResMut<CodeEditorData>,
    mut sound: NonSendMut<SoundControl>,
) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        if ui.text_edit_multiline(&mut editor_data.src).changed() {
            editor_data.error_text = match Program::from_str(&editor_data.src) {
                Ok(p) => {
                    sound.set(p.to_fn().unwrap());
                    "Compilation succesful!".into()
                }
                Err(e) => e.to_string(),
            }
        };
        ui.label(&editor_data.error_text);
    });
}
