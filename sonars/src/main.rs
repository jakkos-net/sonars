pub mod editor;
pub mod lang;
pub mod math;
pub mod visuals;

use bevy::prelude::*;
use bevy_funk::{SoundControl, SoundPlugin, SoundStartEvent};
use math::{bjorklund::bjork, hat, sat};
use std::f32::consts::TAU;

use bevy_egui::{
    egui::{self},
    EguiContexts, EguiPlugin,
};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SoundPlugin)
        .add_startup_system(setup)
        .add_system(ui)
        .run();
}

fn setup(sound: Res<SoundControl>, mut sound_start_ev_writer: EventWriter<SoundStartEvent>) {
    sound_start_ev_writer.send(SoundStartEvent);
    sound.push_soundfn(Box::new(|t| {
        let n = seq!(t; 440.0, 320.0, 660.0);
        let s = (n * TAU * t).sin();
        let b = sat(bjork(7, 4, t));
        let out = s * hat((b * 5.0) % 1.0);

        let vol = 0.2;
        let out = vol * out;
        [out, out]
    }));
}

fn ui(mut egui_context: EguiContexts, time: Res<Time>) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        ui.label("hello world!");
        ui.label(format!(
            "time: {:.2}",
            time.startup().elapsed().as_secs_f32()
        ));
    });
}
