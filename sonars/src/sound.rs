use bevy::{
    ecs::system::SystemState,
    prelude::{EventReader, Plugin, ResMut, Resource, World},
};
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
        app.add_event::<SoundStartEvent>();
        app.add_system(update_sound);
        app.add_system(start_sound);
    }
}

#[derive(Clone)]
pub struct SoundStartEvent;

fn update_sound(mut sound_control: ResMut<SoundControl>) {
    sound_control.update();
}

// todo_minor: find a better way to start sound
fn start_sound(world: &mut World) {
    let mut events_state = SystemState::<EventReader<SoundStartEvent>>::new(world);
    let mut events = events_state.get_mut(world);
    let events: Vec<SoundStartEvent> = events.iter().cloned().collect(); // need to collect and clone to get rid of reference to world
    for _ in events.into_iter() {
        world.init_non_send_resource::<SoundResources>();
    }
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

// todo_major: we can't ever have the WASM audio processor block while trying to get the next sound function, we can't use a mutex

pub fn try_pop_sound() -> Option<SoundFn> {
    todo!()
}

static CURRENT_SOUND_FN: Lazy<Mutex<SoundFn>> = Lazy::new(|| Mutex::new(Box::new(|_| 0.0)));
