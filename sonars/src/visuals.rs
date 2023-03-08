use std::collections::VecDeque;

use bevy::prelude::{Plugin, Res, ResMut, Resource, Schedule};
use bevy_egui::{
    egui::{self, emath, epaint, vec2, Color32, Frame, Pos2, Rect, Stroke, Ui},
    EguiContext,
};

use crate::sound::{SoundControl, SoundFn};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<VisualData>().add_system(visuals);
    }
}

#[derive(Resource)]
pub struct VisualData {
    wave_history: VecDeque<Vec<f32>>,
    wave_history_len: usize,
    wave_history_points: usize,
}

impl Default for VisualData {
    fn default() -> Self {
        Self {
            wave_history: Default::default(),
            wave_history_len: 30,
            wave_history_points: 256,
        }
    }
}

fn visuals(
    mut egui_context: ResMut<EguiContext>,
    mut data: ResMut<VisualData>,
    sound_control: Res<SoundControl>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        inner_visuals(ui, data.as_mut(), sound_control.current())
    });
}

pub fn inner_visuals(ui: &mut Ui, data: &mut VisualData, sound_fn: &SoundFn) {
    // adapted from the egui.rs dancing strings demo
    ui.ctx().request_repaint();
    let time = ui.input().time as f32;

    let (_id, rect) = ui.allocate_space(ui.available_size());

    let height = 0.5;
    let n = data.wave_history_points;
    let time_scale = 0.01;

    data.wave_history.push_front(
        (0..=n)
            .map(|i| {
                let t = (i as f32 / (n as f32)) * time_scale + time;
                let y = sound_fn(t) * height;
                y
            })
            .collect(),
    );

    data.wave_history.truncate(data.wave_history_len);

    let to_screen =
        emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);
    let thickness = 1.0;
    data.wave_history.iter().enumerate().for_each(|(i, ys)| {
        let l_norm =
            (((data.wave_history_len - i) as f32) / (data.wave_history_len as f32)).powf(5.0);
        let l = (l_norm * 255.0) as u8;
        let color = Color32::from_additive_luminance(l);

        let points: Vec<Pos2> = ys
            .iter()
            .enumerate()
            .map(|(i, y)| {
                let t = i as f32 / (n as f32);
                to_screen * bevy_egui::egui::pos2(t as f32, *y)
            })
            .collect();

        ui.painter()
            .add(epaint::Shape::line(points, Stroke::new(thickness, color)));
    });
}
