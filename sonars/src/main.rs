pub mod math;
pub mod sound;

use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContext, EguiPlugin,
};

use math::{bjorklund::bjorklund, saturate};
use sound::{SoundControl, SoundPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SoundPlugin)
        .add_startup_system(setup)
        .add_system(ui)
        .run();
}

fn setup(mut sound: ResMut<SoundControl>) {
    sound
        .set(Box::new(|t| {
            let s = (440.0 * 2.0 * PI * t).sin();
            let bjork = saturate(bjorklund(7, 4, t));
            let out = s * bjork;
            out
        }))
        .unwrap();
}

fn ui(mut egui_context: ResMut<EguiContext>, time: Res<Time>) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        ui.label("hello world!");
    });
}
