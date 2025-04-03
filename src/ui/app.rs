use std::sync::Once;

use egui::{CentralPanel, Color32, Context, Frame, Label, Vec2, Window};

use crate::ui::widgets;

#[derive(Default)]
pub struct AppState {
    pub keybind: Option<egui::Key>,
    pub show_menu: bool,
    pub window_size: Vec2
}

impl AppState {
    pub fn set_keybind(&mut self, key: egui::Key) {
        self.keybind = Some(key);
    }
}


pub fn ui(ctx: &Context, app_state: &mut AppState) {
    if app_state.show_menu {
        CentralPanel::default()
        .frame(Frame {
            fill: Color32::GRAY.gamma_multiply(0.5),
            ..Default::default()
        })
        .show(ctx, |ui| {
            Window::new("overlay_menu")
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(Label::new("This is a fullscreen overlay"));
                        if ui.button("Close Overlay").clicked() {
                            app_state.show_menu = !app_state.show_menu;
                        }
                    });
                });
        });
    }

    unsafe {
        static ONCE: Once = Once::new();

        ONCE.call_once(|| {
            // Uncomment this to set other fonts.
            // let mut fonts = FontDefinitions::default();
            // let mut tweak = FontTweak::default();
            // fonts.font_data.insert(
            //     "my_font".to_owned(),
            //     FontData::from_static(include_bytes!("Lobster-Regular.ttf")).tweak(tweak),
            // );
            // fonts
            //     .families
            //     .get_mut(&FontFamily::Proportional)
            //     .unwrap()
            //     .insert(0, "my_font".to_owned());
            // fonts
            //     .families
            //     .get_mut(&FontFamily::Monospace)
            //     .unwrap()
            //     .push("my_font".to_owned());
            // ctx.set_fonts(fonts);

            // egui_extras::install_image_loaders(ctx);
        });

        // https://github.com/emilk/egui/discussions/4228
        egui::containers::Window::new("Main menu").frame(egui::Frame::NONE).show(ctx, |ui| {
            widgets::show_damage_distribution_widget(app_state, ui);
        });
    }
}
