pub mod lang;
pub mod math;
pub mod sound;
pub mod visuals;

use bevy::{prelude::*, time::Stopwatch};
use bevy_egui::{
    egui::{self, CollapsingHeader, DragValue, TextEdit},
    EguiContext, EguiPlugin,
};
use lang::compile;
use sound::{SoundControl, SoundPlugin};
use visuals::VisualsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SoundPlugin)
        .add_plugin(VisualsPlugin)
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
            src: "sin(330.0*t) * sin(33.0*t) * euc(7., 3., t)".into(),
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
        let text_edit = ui.add(TextEdit::multiline(&mut editor.src).code_editor().desired_rows(20).desired_width(600.0));
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

        if should_compile {
            editor.error_text = match compile(&editor.src) {
                Ok(sound_fn) => {
                    sound.push(sound_fn);
                    "Compilation succesful!".into()
                },
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

        CollapsingHeader::new("Notes").default_open(true).show(ui,|ui|{
            ui.label("t is the current time in seconds");
            ui.label("Currently, numbers have to have a decimal point, e.g. you have to write 7. instead of 7");
            ui.label("Currently, program only loops 2 seconds of audio");
            ui.label("sin(t) -> sine of t, where t is in seconds, 1 cycle per second");
            ui.label("abs(t) -> absolute value of t");
            ui.label("euc(steps, pulses, t) -> euclidean rhythm, pulses cannot be bigger than steps");
            ui.label("ln(t) -> natural log of t");
            ui.label("log(t, p) -> log of t, base p");
            ui.label("power(t, p) -> t to the power of p");
        });
    });
}
