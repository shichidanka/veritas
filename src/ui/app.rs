use std::sync::Once;

use egui::{Color32, Context};

use crate::ui::widgets;

#[derive(Default)]
pub struct AppState {
    
}

pub fn ui(ctx: &Context, app_state: &mut AppState) {
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
