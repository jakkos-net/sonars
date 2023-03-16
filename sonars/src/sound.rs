use bevy::prelude::{Plugin, ResMut, Resource};
use std::sync::Mutex;

use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub mod wasm;
        use crate::sound::wasm::*;
    } else {
        pub mod native;
        use crate::sound::native::*;
    }
}

const SAMPLE_RATE: u32 = 48_000;
const INV_SAMPLE_RATE: f32 = (1.0 / (SAMPLE_RATE as f64)) as f32;

pub struct SoundPlugin;

pub type SoundFn = Box<dyn Fn(f32) -> f32 + Send + Sync>;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SoundControl>();
        app.init_non_send_resource::<SoundResources>();
        app.add_system(update_sound);
    }
}

fn update_sound(mut sound_control: ResMut<SoundControl>) {
    sound_control.update();
}

#[derive(Resource)]
pub struct SoundControl {
    queue: SegQueue<SoundFn>,
    next_fn: SoundFn,
}

impl Default for SoundControl {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            next_fn: Box::new(|_| 0.0),
        }
    }
}

impl SoundControl {
    pub fn push(&self, new_fn: SoundFn) {
        self.queue.push(new_fn);
    }

    fn update(&mut self) {
        while !self.queue.is_empty() {
            self.next_fn = self.queue.pop().unwrap();
        }
    }

    pub fn current(&self) -> &SoundFn {
        &self.next_fn
    }
}

pub fn push_sound(new_fn: SoundFn) {
    *CURRENT_SOUND_FN.lock().unwrap() = new_fn;
}

static CURRENT_SOUND_FN: Lazy<Mutex<SoundFn>> = Lazy::new(|| Mutex::new(Box::new(|_| 0.0)));
