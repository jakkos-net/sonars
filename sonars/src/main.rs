pub mod math;
pub mod sound;

use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContext, EguiPlugin,
};

use math::{bjorklund::bjork, sat};
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
            let s = (seq!(t; 440.0, 220.0, 330.0, 900.0) * TAU * t).sin();
            // let b = sat(bjork(7, 4, t));
            let out = s;
            out
        }))
        .unwrap();
}

fn ui(mut egui_context: ResMut<EguiContext>, time: Res<Time>) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        ui.label("hello world!");
        ui.label(format!(
            "time: {:.2}",
            time.startup().elapsed().as_secs_f32()
        ))
    });
}
