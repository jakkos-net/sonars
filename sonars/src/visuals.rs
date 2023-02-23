use bevy_egui::egui::{emath, epaint, vec2, Color32, Frame, Pos2, Rect, Stroke, Ui};

pub fn test_visuals(ui: &mut Ui) {
    let color = if ui.visuals().dark_mode {
        Color32::from_additive_luminance(196)
    } else {
        Color32::from_black_alpha(240)
    };

    // adapted from the egui.rs dancing strings demo
    Frame::canvas(ui.style()).show(ui, |ui| {
        ui.ctx().request_repaint();
        let time = ui.input().time;

        let desired_size = ui.available_width() * vec2(1.0, 0.35);
        let (_id, rect) = ui.allocate_space(desired_size);

        let to_screen =
            emath::RectTransform::from_to(Rect::from_x_y_ranges(0.0..=1.0, -1.0..=1.0), rect);

        let mode = 5.0;
        let n = 120;
        let speed = 1.5;

        let points: Vec<Pos2> = (0..=n)
            .map(|i| {
                let t = i as f64 / (n as f64);
                let amp = (time * speed * mode).sin() / mode;
                let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                to_screen * bevy_egui::egui::pos2(t as f32, y as f32)
            })
            .collect();

        let thickness = 10.0 / mode as f32;
        ui.painter()
            .add(epaint::Shape::line(points, Stroke::new(thickness, color)));
    });
}
