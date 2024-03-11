pub mod editor;
mod fft;
pub mod lang;
pub mod math;
mod sound;
pub mod visuals;

use bevy::prelude::*;
use math::*;

use bevy_egui::{
    egui::{self, CollapsingHeader, DragValue},
    EguiContexts, EguiPlugin,
};
use visuals::{VisualsControls, VisualsPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(sound::SoundPlugin)
        .add_plugins(VisualsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui)
        .run();
}
fn ui(
    mut egui_context: EguiContexts,
    time: Res<Time>,
    mut visual_controls: ResMut<VisualsControls>,
) {
    egui::SidePanel::left("controls panel").show(egui_context.ctx_mut(), |ui| {
        CollapsingHeader::new("Sound")
            .default_open(true)
            .show(ui, |ui| {
                ui.label(format!("Time: {:.2}", time.elapsed().as_secs_f32()));
                ui.horizontal(|ui| {
                    if ui.button("Play").clicked() {
                        todo!()
                    }
                    if ui.button("Pause").clicked() {
                        todo!()
                    }
                });
                if ui.button("Restart audio server").clicked() {
                    todo!()
                }
            });

        CollapsingHeader::new("Wave")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("No. samples:");
                    ui.add(DragValue::new(&mut visual_controls.wave_samples));
                });
                ui.horizontal(|ui| {
                    ui.label("Height:");
                    ui.add(DragValue::new(&mut visual_controls.wave_height_scale));
                });
                ui.horizontal(|ui| {
                    ui.label("Line width:");
                    ui.add(DragValue::new(&mut visual_controls.wave_line_width));
                });
                ui.horizontal(|ui| {
                    ui.label("Time rounding:");
                    ui.add(DragValue::new(&mut visual_controls.wave_time_rounding));
                });
                ui.horizontal(|ui| {
                    ui.label("Time scale:");
                    ui.add(DragValue::new(&mut visual_controls.wave_inv_time_scale));
                    if ui.button("-").clicked() {
                        visual_controls.wave_inv_time_scale -= 1.0;
                    }
                    if ui.button("+").clicked() {
                        visual_controls.wave_inv_time_scale += 1.0;
                    }
                    if ui.button("-10").clicked() {
                        visual_controls.wave_inv_time_scale -= 10.0;
                    }
                    if ui.button("+10").clicked() {
                        visual_controls.wave_inv_time_scale += 10.0;
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("No. ghosts");
                    ui.add(DragValue::new(&mut visual_controls.wave_history_len));
                });
                ui.horizontal(|ui| {
                    ui.label("Ghost fadeoff:");
                    ui.add(DragValue::new(&mut visual_controls.wave_fade_off));
                });
            });

        CollapsingHeader::new("FFT")
            .default_open(true)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Line width:");
                    ui.add(DragValue::new(&mut visual_controls.fft_line_width));
                });
            })
    });
}

fn setup(mut sound: ResMut<sound::SoundControl>) {
    sound.start();
    sound.push_soundfn(Box::new(|t| {
        // let note = seq![440.0, 440.0, 330.0, 660.0](t);
        // let d = detune!(2, 0.25, |k, t| quant(tri((note + k) * t), 3))(t);
        // let e = env![0.0, 1.0, 0.0];
        // let pat = seq![|t| 1.0 - id(t), id, id](t);
        // let out = e(pat) * d;

        let m = sin(1.0 * t) * 400.0;
        // let out = m;
        // let out = sqr((800.0 + m) * t);

        // let m = sin(200.0 * t) * 400.0;
        let out = sin((800.0 + m) * t);

        // let out = saw(440.0 * t);

        // let out = sqr(440.0 * t);
        // let out = sin(440.0 * t);

        // let out = sin(880.0 * t) + sin(440. * t) + sin(220. * t);
        // let out = out / 3.0;

        let vol = 0.1;
        let out = vol * out;
        // let out = clip(out) * vol;
        [out, out]
    }));
}
