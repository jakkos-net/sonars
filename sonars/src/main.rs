use bevy::ecs::system::Resource;
use rodio::{Source, OutputStream, Sink};
use std::sync::mpsc::{channel, Sender, Receiver, SyncSender, sync_channel};
use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = 1.0 / (SAMPLE_RATE as f32);

enum MExpr{
    Sin(E),
    Add(E, E),
    Mul(E, E),
    Div(E, E),
    Sub(E, E),
    Mod(E, E),
    Pow(E,E),
    Num(f32),
    Invoke(String),
    Time
}




struct SoundControl{
    sound_func_tx: SyncSender<Box<dyn Fn(f32) -> f32 + Send + Sync>>
}

impl SoundControl{
    pub fn set(&self, f:Box<dyn Fn(f32) -> f32 + Send + Sync>)
    {
        self.sound_func_tx.send(f).ok();
    }
}

fn main() {

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let (tx,rx) = sync_channel::<Box<dyn Fn(f32) -> f32 + Send + Sync>>(100);
    let source = SamplesSource {
        sample_func: Box::new(|_| 0.0),
        new_sample_func: rx,
        t: 0,
    };
    sink.append(source);

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(start_sound)
        .insert_resource(SoundControl{sound_func_tx: tx})
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}


enum MStmt{
    Assign(String, Vec<String>, MExpr)
}


type E = Box<MExpr>;

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
pub struct SamplesSource {
    sample_func: Box<dyn Fn(f32) -> f32 + Send + Sync>,
    new_sample_func: Receiver<Box<dyn Fn(f32) -> f32 + Send + Sync>>,
    t: usize
}

impl Source for SamplesSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        SAMPLE_RATE
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        None
    }
}

impl Iterator for SamplesSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {

        if let Ok(new_sample_func) = self.new_sample_func.try_recv(){
            self.sample_func = new_sample_func
        }
        let f = (self.sample_func)(self.t as f32  * INV_SAMPLE_RATE);
        self.t += 1;
        Some(f)
    }
}