pub mod sound;

use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use sound::{SoundControl, SoundPlugin};

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SoundPlugin)

        .init_resource::<CodeSource>()

        .add_startup_system(start_sound)

        .add_system(coding_ui)
        .run();

}

struct CodeSource(pub String);

impl Default for CodeSource{
    fn default() -> Self {
        Self("Woo I'm some source code... Apparently...".into())
    }
}

fn coding_ui(mut egui_context: ResMut<EguiContext>, mut source: ResMut<CodeSource>) {
    egui::Window::new("Editor").show(egui_context.ctx_mut(), |ui| {
        ui.text_edit_multiline(&mut source.0);
    });
}

fn start_sound(sound: Res<SoundControl>) {

    let tau = 2.0 * PI;
    let sample_func = Box::new(move |t:f32|{

        let outer_wave = (t * tau).sin().abs();
        let outer_wave2 = (t * tau + tau*0.25).sin().abs();
        let inner_wave =  |a:f32| (t * tau * a).sin().round();
        let detune = |func: &dyn Fn(f32) -> f32, input:f32, detune:f32|  (func(input - detune) + func(input) + func(input + detune))/3.0;
        let res = outer_wave * detune(&inner_wave, 440.0, 1.2) * outer_wave2;
        res.clamp(-1.0, 1.0)
    });

    sound.set(sample_func)
}