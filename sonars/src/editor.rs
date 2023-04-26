use bevy::{
    prelude::{Commands, EventWriter, Plugin, Res, ResMut, Resource},
    time::{Stopwatch, Time},
};
use bevy_egui::{
    egui::{self, CollapsingHeader, DragValue, Slider, TextEdit},
    EguiContexts,
};

use crate::{
    lang::compile,
    sound::{push_sound, SoundControl, SoundStartEvent},
    visuals::VisualsControls,
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<CodeEditorData>().add_system(coding_ui);
    }
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
            src: "let signal = sin(330.0*t) * sin(33.0*t) * euc(7., 3., t);\n\nlet volume = 0.5;\n\nsignal*volume".into(),
            error_text: "".into(),
            last_edit: Stopwatch::new(),
            waiting_to_compile: false,
            auto_compile_delay: 0.5,
            auto_compile: true,
        }
    }
}

fn coding_ui(
    mut egui_context: EguiContexts,
    mut editor: ResMut<CodeEditorData>,
    mut visual_controls: ResMut<VisualsControls>,
    mut sound_start_ev_writer: EventWriter<SoundStartEvent>,
    sound: Res<SoundControl>,
    time: Res<Time>,
) {
    egui::SidePanel::new(egui::panel::Side::Left, "Editor").resizable(true).default_width(400.0).show(egui_context.ctx_mut(), |ui|{

        if ui.button("Start").clicked(){
            sound_start_ev_writer.send(SoundStartEvent);
        };

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
                    push_sound(compile(&editor.src).unwrap());
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
        CollapsingHeader::new("Visuals").default_open(false).show(ui, |ui|{
            ui.horizontal(|ui|{
                ui.label("time scale:");
                ui.add(Slider::new(&mut visual_controls.time_scale, 0.0..=1.0))
            });
            ui.horizontal(|ui|{
                ui.label("height:");
                ui.add(Slider::new(&mut visual_controls.height, 0.0..=1.0))
            });
            ui.horizontal(|ui|{
                ui.label("fade off:");
                ui.add(Slider::new(&mut visual_controls.fade_off, 0.0..=100.0))
            });
            ui.horizontal(|ui|{
                ui.label("thickness:");
                ui.add(Slider::new(&mut visual_controls.thickness, 0.0..=10.0))
            });
        });

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
