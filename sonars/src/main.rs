pub mod editor;
pub mod lang;
pub mod math;
pub mod sound;
pub mod visuals;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use editor::EditorPlugin;
use sound::SoundPlugin;
use visuals::VisualsPlugin;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SoundPlugin)
        .add_plugin(EditorPlugin)
        .add_plugin(VisualsPlugin)
        .run();
}
