pub mod lang;
pub mod sound;
pub mod math;

use bevy::{prelude::*, time::Stopwatch};
use bevy_egui::{
    egui::{self, DragValue, TextEdit},
    EguiContext, EguiPlugin,
};
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
    pub last_edit: Stopwatch,
    pub waiting_to_compile: bool,
    pub auto_compile_delay: f32,
    pub auto_compile: bool,
}

impl Default for CodeEditorData {
    fn default() -> Self {
        Self {
            src: "sin(330.0 * t) + sin(35.0 * t) * sin(3.0 * t) + sin(660.0 * t) * sin(23.0 * t) * sin(0.75 * t)"
                .into(),
            error_text: "".into(),
            last_edit: Stopwatch::new(),
            waiting_to_compile: false,
            auto_compile_delay: 0.5,
            auto_compile: true,
        }
    }
}

fn coding_ui(
    mut egui_context: ResMut<EguiContext>,
    mut editor: ResMut<CodeEditorData>,
    mut sound: NonSendMut<SoundControl>,
    time: Res<Time>,
) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        let text_edit = ui.add(TextEdit::multiline(&mut editor.src).code_editor());
        editor.last_edit.tick(time.delta());

        if text_edit.changed() {
            editor.waiting_to_compile = true;
            editor.last_edit = Stopwatch::new();
        }

        ui.checkbox(&mut editor.auto_compile, "Auto compile?");

        let should_compile = if editor.auto_compile {
            ui.horizontal(|ui| {
                ui.label("Auto compile delay:");
                ui.add(DragValue::new(&mut editor.auto_compile_delay).clamp_range(0.0..=5.0));
            });

            editor.waiting_to_compile
                && editor.last_edit.elapsed().as_secs_f32() >= editor.auto_compile_delay
        } else {
            ui.button("Compile!").clicked()
        };

        if should_compile || !sound.is_playing() {
            editor.error_text = match sound.set(&editor.src) {
                Ok(()) => "Compilation succesful!".into(),
                Err(e) => e.to_string(),
            };

            editor.waiting_to_compile = false;
        };

        if editor.waiting_to_compile {
            editor.error_text = if editor.auto_compile {
                format!(
                    "Waiting for auto compile delay... {:.1}s",
                    editor.auto_compile_delay - editor.last_edit.elapsed().as_secs_f32()
                )
            } else {
                "Hit compile to hear changes!".into()
            }
        }

        ui.label(&editor.error_text);
    });
}
