use bevy::{
    ecs::system::SystemState,
    prelude::{EventReader, Plugin, Res, ResMut, Resource, World},
    time::Time,
};
use std::sync::Mutex;
use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub mod wasm;
        use crate::wasm::*;
    } else {
        pub mod native;
        use crate::native::*;
    }
}

const SAMPLE_RATE: u32 = 44_000;
const INV_SAMPLE_RATE: f32 = (1.0 / (SAMPLE_RATE as f64)) as f32;
const TIME_DIFFERENCE_THRESHOLD: f32 = 200.0 / 1000.0;
static SAMPLE_INDEX: AtomicUsize = AtomicUsize::new(0);

pub struct SoundPlugin;

pub type SoundFn = Box<dyn Fn(f32) -> [f32; 2] + Send + Sync>;

pub fn empty_sound_fn() -> SoundFn {
    Box::new(|_| [0.0, 0.0])
}

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

fn update_sound(mut sound_control: ResMut<SoundControl>, time: Res<Time>) {
    sound_control.update(time.elapsed_seconds_f64());

    let sample_index = SAMPLE_INDEX.load(std::sync::atomic::Ordering::Relaxed);
    let audio_time = sample_index as f32 * INV_SAMPLE_RATE;
    let app_time = sound_control.time() as f32;

    if (audio_time - app_time).abs() > TIME_DIFFERENCE_THRESHOLD {
        let new_index = (app_time * SAMPLE_RATE as f32) as usize;
        SAMPLE_INDEX.store(new_index, std::sync::atomic::Ordering::Relaxed);
    }
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
    start_time: f64,
    elapsed_time: f64,
}

impl Default for SoundControl {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            next_fn: empty_sound_fn(),
            start_time: 0.0,
            elapsed_time: 0.0,
        }
    }
}

impl SoundControl {
    pub fn push_soundfn(&self, new_fn: SoundFn) {
        self.queue.push(new_fn);
    }

    fn update(&mut self, time: f64) {
        while !self.queue.is_empty() {
            self.next_fn = self.queue.pop().unwrap();
        }
        self.elapsed_time = time - self.start_time;
    }

    pub fn set_time(&mut self, new_elapsed_time: f64) {
        let current_time = self.start_time + self.elapsed_time;
        self.elapsed_time = new_elapsed_time;
        self.start_time = current_time - self.elapsed_time;
    }

    pub fn time(&self) -> f64 {
        self.elapsed_time
    }

    pub fn current_soundfn(&self) -> &SoundFn {
        &self.next_fn
    }
}

pub fn push_sound(new_fn: SoundFn) {
    *CURRENT_SOUND_FN.lock().unwrap() = Arc::new(new_fn);
}

// todo_major: we can't ever have the WASM audio processor block while trying to get the next sound function, we can't use a mutex

pub fn try_pop_sound() -> Option<SoundFn> {
    todo!()
}

static CURRENT_SOUND_FN: Lazy<Mutex<Arc<SoundFn>>> =
    Lazy::new(|| Mutex::new(Arc::new(empty_sound_fn())));
