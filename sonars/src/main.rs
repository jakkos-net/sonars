pub mod editor;
pub mod lang;
pub mod math;
pub mod visuals;

use bevy::prelude::*;
use bevy_funk::{SoundControl, SoundPlugin};
use math::*;

use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(SoundPlugin)
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

fn setup(mut sound: ResMut<SoundControl>) {
    sound.start();
    sound.push_soundfn(Box::new(|t| {
        //here

        let d = detune!(5, 1.1, |k, t| sin(330.0 * k * t))(t);
        let e = env![0.0, 1.0, 0.0];
        let pat = seq![bjork!(7, 3), bjork!(9, 5)](t);
        let out = e(pat) * d;
        //out
        let vol = 0.2;
        let out = sat(vol * out);
        [out, out]
    }));
}
