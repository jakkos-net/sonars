use bevy::prelude::{Plugin, ResMut};
use bevy_egui::{
    egui::{self, emath, epaint, vec2, Color32, Frame, Pos2, Rect, Stroke, Ui},
    EguiContext,
};

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(visuals);
    }
}

fn visuals(mut egui_context: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| inner_visuals(ui));
}

pub fn inner_visuals(ui: &mut Ui) {
    // adapted from the egui.rs dancing strings demo
    ui.ctx().request_repaint();
    let time = ui.input().time;

    let (_id, rect) = ui.allocate_space(ui.available_size());

    let to_screen =
        emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

    let mode = 5.0;
    let n = 120;
    let speed = 3.0;

    let points: Vec<Pos2> = (0..=n)
        .map(|i| {
            let t = i as f64 / (n as f64);
            let amp = (time * speed * mode).sin() / mode;
            let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
            to_screen * bevy_egui::egui::pos2(t as f32, y as f32)
        })
        .collect();

    let thickness = 10.0 / mode as f32;
    let color = Color32::from_additive_luminance(255);
    ui.painter()
        .add(epaint::Shape::line(points, Stroke::new(thickness, color)));
}
