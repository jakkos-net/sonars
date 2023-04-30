use std::collections::VecDeque;

use bevy::prelude::{Plugin, Res, ResMut, Resource};
use bevy_egui::{
    egui::{self, emath, epaint, Color32, Pos2, Rect, Stroke},
    EguiContexts,
};

use crate::sound::SoundControl;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<VisualData>()
            .init_resource::<VisualsControls>()
            .add_system(visuals);
    }
}

#[derive(Resource)]
pub struct VisualData {
    wave_history: VecDeque<Vec<f32>>,
    wave_history_len: usize,
    wave_history_points: usize,
}

#[derive(Resource)]
pub struct VisualsControls {
    pub time_scale: f32,
    pub fade_off: f32,
    pub height: f32,
    pub thickness: f32,
}

impl Default for VisualsControls {
    fn default() -> Self {
        Self {
            time_scale: 0.1,
            fade_off: 2.0,
            height: 0.5,
            thickness: 1.0,
        }
    }
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
    mut egui_context: EguiContexts,
    mut data: ResMut<VisualData>,
    controls: Res<VisualsControls>,
    sound_control: Res<SoundControl>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.ctx().request_repaint();
        let time = ui.input(|input| input.time) as f32;

        let (_id, rect) = ui.allocate_space(ui.available_size());

        let height = controls.height;
        let n = data.wave_history_points;
        let time_scale = controls.time_scale;
        let sound_fn = sound_control.current();
        data.wave_history.push_front(
            (0..=n)
                .map(|i| {
                    let t = (i as f32 / (n as f32)) * time_scale + time;
                    let y = sound_fn(t)[0] * height;
                    y
                })
                .collect(),
        );
        let data = data.as_mut();
        data.wave_history.truncate(data.wave_history_len);

        let to_screen =
            emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);
        let thickness = controls.thickness;
        let fade_off = controls.fade_off;
        data.wave_history.iter().enumerate().for_each(|(i, ys)| {
            let l_norm = (((data.wave_history_len - i) as f32) / (data.wave_history_len as f32))
                .powf(fade_off);
            let l = (l_norm * 255.0) as u8;
            let color = Color32::from_additive_luminance(l);

            let points: Vec<Pos2> = ys
                .iter()
                .enumerate()
                .map(|(i, y)| {
                    let t = i as f32 / (n as f32);
                    to_screen * bevy_egui::egui::pos2(t, *y)
                })
                .collect();

            ui.painter()
                .add(epaint::Shape::line(points, Stroke::new(thickness, color)));
        });
    });
}
