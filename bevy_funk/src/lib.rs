use bevy::{
    app::Update,
    ecs::system::Commands,
    log::info,
    prelude::{Plugin, Res, ResMut, Resource},
    time::Time,
};
use dyn_clone::DynClone;
use std::sync::Mutex;
use std::sync::{atomic::AtomicUsize, Arc};

use crossbeam_queue::SegQueue;
use once_cell::sync::Lazy;

use crate::native::SoundResources;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        pub mod wasm;
        use crate::wasm::*;
    } else {
        pub mod native;
    }
}

const SAMPLE_RATE: u32 = 44_000;
const INV_SAMPLE_RATE: f32 = (1.0 / (SAMPLE_RATE as f64)) as f32;
const TIME_DIFFERENCE_THRESHOLD: f32 = 200.0 / 1000.0;
static SAMPLE_INDEX: AtomicUsize = AtomicUsize::new(0);

pub struct SoundPlugin;

pub type SoundFn = Box<dyn SoundFnTrait>;
pub trait SoundFnTrait: Fn(f32) -> [f32; 2] + Send + Sync + DynClone {}
impl<T> SoundFnTrait for T where T: Fn(f32) -> [f32; 2] + Clone + Send + Sync {}

pub fn empty_sound_fn() -> SoundFn {
    Box::new(|_| [0.0, 0.0])
}

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SoundControl>();
        app.add_systems(Update, update);
    }
}

fn update(mut commands: Commands, mut sound_control: ResMut<SoundControl>, time: Res<Time>) {
    match sound_control.state {
        State::Starting => {
            commands.init_resource::<SoundResources>();
            info!("Sound init!");
            sound_control.state = State::Running
        }
        State::Running => {
            sound_control.update(time.elapsed_seconds_f64());

            let sample_index = SAMPLE_INDEX.load(std::sync::atomic::Ordering::Relaxed);
            let audio_time = sample_index as f32 * INV_SAMPLE_RATE;
            let app_time = sound_control.time() as f32;

            if (audio_time - app_time).abs() > TIME_DIFFERENCE_THRESHOLD {
                let new_index = (app_time * SAMPLE_RATE as f32) as usize;
                SAMPLE_INDEX.store(new_index, std::sync::atomic::Ordering::Relaxed);
            }
        }
        State::Stopped => {}
    }
}

#[derive(Resource)]
pub struct SoundControl {
    queue: SegQueue<SoundFn>,
    next_fn: SoundFn,
    sound_fn_changed: bool,
    start_time: f64,
    elapsed_time: f64,
    state: State,
}

#[derive(Debug)]
enum State {
    Stopped,
    Starting,
    Running,
}

impl Default for SoundControl {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            next_fn: empty_sound_fn(),
            sound_fn_changed: true,
            start_time: 0.0,
            elapsed_time: 0.0,
            state: State::Stopped,
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
            self.sound_fn_changed = true;
        }
        self.elapsed_time = time - self.start_time;

        if self.sound_fn_changed {
            let new_sound_fn = dyn_clone::clone_box(&*self.next_fn);
            set_sound(new_sound_fn);
            self.sound_fn_changed = false;
        }
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

    pub fn start(&mut self) {
        match &self.state {
            State::Stopped => self.state = State::Starting,
            x => panic! {"Sound is in state {:?}, you can only start sound when it's stopped!", x},
        }
    }
}

fn set_sound(new_fn: SoundFn) {
    *CURRENT_SOUND_FN.lock().unwrap() = Arc::new(new_fn);
}

static CURRENT_SOUND_FN: Lazy<Mutex<Arc<SoundFn>>> =
    Lazy::new(|| Mutex::new(Arc::new(empty_sound_fn())));
