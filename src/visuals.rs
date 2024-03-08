use std::collections::VecDeque;

use bevy::{
    app::{FixedUpdate, PostUpdate},
    prelude::{Plugin, Res, ResMut, Resource},
    time::Time,
};
use bevy_egui::{
    egui::{self, emath, epaint, Color32, Pos2, Rect, Stroke},
    EguiContexts,
};

use crate::{
    fft::{fft, FreqMag, FFT_BUFFER_SIZE},
    sound::{Float, FloatOut, SoundControl, INV_SAMPLE_RATE, SAMPLE_RATE},
};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<VisualData>()
            .init_resource::<VisualsControls>()
            .add_systems(PostUpdate, draw_visuals)
            .add_systems(FixedUpdate, update_data);
    }
}

#[derive(Resource)]
struct VisualData {
    wave_history: VecDeque<Vec<FloatOut>>,
    fft_data: Vec<FreqMag>,
}

#[derive(Resource)]
pub struct VisualsControls {
    pub wave_inv_time_scale: Float,
    pub wave_fade_off: f32,
    pub wave_height_scale: f32,
    pub wave_line_width: f32,
    pub fft_line_width: f32,
    pub wave_time_rounding: f64,
    pub wave_history_len: usize,
    pub wave_samples: usize,
}

impl Default for VisualsControls {
    fn default() -> Self {
        Self {
            wave_inv_time_scale: 100.0,
            wave_fade_off: 2.0,
            wave_height_scale: 0.9,
            wave_line_width: 0.5,
            fft_line_width: 1.0,
            wave_time_rounding: 60.0,
            wave_history_len: 1,
            wave_samples: 2048,
        }
    }
}

impl Default for VisualData {
    fn default() -> Self {
        Self {
            wave_history: Default::default(),
            fft_data: Default::default(),
        }
    }
}

fn update_data(
    time: Res<Time>,
    mut data: ResMut<VisualData>,
    controls: Res<VisualsControls>,
    sound_control: Res<SoundControl>,
) {
    let time = time.elapsed_seconds_f64();
    let height = controls.wave_height_scale;
    let n = controls.wave_samples;
    let sound_fn = sound_control.current_soundfn();
    let wave_time_scale = 1.0 / controls.wave_inv_time_scale;
    data.wave_history.push_front(
        (0..=n)
            .map(|i| {
                let t = (i as f64 / (n as f64)) * wave_time_scale + time;
                let y = sound_fn(t)[0] * height as Float;
                y as FloatOut
            })
            .collect(),
    );
    let data = data.as_mut();
    data.wave_history.truncate(controls.wave_history_len);
    let mut fft_buffer: Vec<_> = (0..FFT_BUFFER_SIZE)
        .into_iter()
        .map(|buffer_idx| {
            let t = time + (buffer_idx as f64 * INV_SAMPLE_RATE);
            sound_fn(t)[0] as FloatOut
        })
        .collect();

    data.fft_data = fft(&mut fft_buffer)
        .map(|x| x.collect())
        .unwrap_or_default();
}

fn draw_visuals(
    mut egui_context: EguiContexts,
    data: ResMut<VisualData>,
    controls: Res<VisualsControls>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.ctx().request_repaint();
        let n = controls.wave_samples;
        let to_screen = emath::RectTransform::from_to(
            Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0),
            ui.ctx().available_rect(),
        );
        data.wave_history.iter().enumerate().for_each(|(i, ys)| {
            let l_norm = (((controls.wave_history_len - i) as FloatOut)
                / (controls.wave_history_len as FloatOut))
                .powf(controls.wave_fade_off as FloatOut);
            let l = (l_norm * 255.0) as u8;
            let color = Color32::from_additive_luminance(l);

            let points: Vec<Pos2> = ys
                .iter()
                .enumerate()
                .map(|(i, y)| {
                    let t = i as FloatOut / (n as FloatOut);
                    to_screen * bevy_egui::egui::pos2(t, *y)
                })
                .collect();

            ui.painter().add(epaint::Shape::line(
                points,
                Stroke::new(controls.wave_line_width, color),
            ));
        });

        let max_mag: f32 = data
            .fft_data
            .iter()
            .map(|fm| fm.mag)
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap_or(1.0);

        let margin = 0.025;
        let non_margin = 1.0 - margin * 2.0;

        let color = Color32::from_rgb(255, 0, 0);
        let points: Vec<Pos2> = data
            .fft_data
            .iter()
            .map(|FreqMag { freq, mag }| {
                let x = (2.0 * freq / (SAMPLE_RATE as f32)) * non_margin + margin;
                let y = ((mag / max_mag) * 2.0 * non_margin + margin - 1.0) * -1.0;
                to_screen * bevy_egui::egui::pos2(x, y)
            })
            .collect();

        ui.painter().add(epaint::Shape::line(
            points,
            Stroke::new(controls.fft_line_width, color),
        ));
    });
}
