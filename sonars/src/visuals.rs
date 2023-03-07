use std::collections::VecDeque;

use bevy::prelude::{Plugin, ResMut, Resource};
use bevy_egui::{
    egui::{self, emath, epaint, vec2, Color32, Frame, Pos2, Rect, Stroke, Ui},
    EguiContext,
};

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

fn visuals(mut egui_context: ResMut<EguiContext>, mut data: ResMut<VisualData>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        inner_visuals(ui, data.as_mut())
    });
}

pub fn inner_visuals(ui: &mut Ui, data: &mut VisualData) {
    // adapted from the egui.rs dancing strings demo
    ui.ctx().request_repaint();
    let time = ui.input().time as f32;

    let (_id, rect) = ui.allocate_space(ui.available_size());

    let height = 0.5;
    let mode = 2.0;
    let speed = 1.25;
    let n = data.wave_history_points;

    data.wave_history.push_front(
        (0..=n)
            .map(|i| {
                let t = (i as f32 / (n as f32)) + time;
                let amp = (time * speed * mode).sin() * height;
                let y = amp * (t * std::f32::consts::TAU / 2.0 * mode).sin();
                y
            })
            .collect(),
    );

    data.wave_history.truncate(data.wave_history_len);

    let to_screen =
        emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);
    let thickness = 7.0 / mode as f32;
    data.wave_history.iter().enumerate().for_each(|(i, ys)| {
        let l_norm =
            (((data.wave_history_len - i) as f32) / (data.wave_history_len as f32)).powf(2.0);
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
