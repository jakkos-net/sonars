pub mod editor;
mod fft;
pub mod lang;
pub mod math;
mod sound;
pub mod visuals;

use bevy::prelude::*;
use math::*;

use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};
use visuals::VisualsPlugin;

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
fn ui(mut egui_context: EguiContexts, time: Res<Time>) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        ui.label("hello world!");
        ui.label(format!("time: {:.2}", time.elapsed().as_secs_f32()));
    });
}

fn setup(mut sound: ResMut<sound::SoundControl>) {
    sound.start();
    sound.push_soundfn(Box::new(|t| {
        //here

        // let note = seq![440.0, 440.0, 330.0, 660.0](t);
        // let d = detune!(2, 0.25, |k, t| quant(tri((note + k) * t), 10))(t);
        // let e = env![0.0, 1.0, 0.0];
        // let pat = seq![|t| 1.0 - id(t), id, id](t);
        // let out = e(pat) * d;
        let m = sin(200.0 * t) * 50.0;
        // let m = 0.0;
        let out = sin((800.0 + m) * t);
        // let out = sin(440.0 * t);
        //out

        let vol = 0.1;
        let out = clip(vol * out);
        [out, out]
    }));
}
