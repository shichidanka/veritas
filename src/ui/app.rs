use std::sync::Once;
use egui::{Color32, Context, Slider};
use crate::ui::widgets;

#[derive(Default)]
pub struct AppState {
    show_windows: bool,
    show_damage_distribution: bool,
    show_damage_bars: bool,
    widget_opacity: f32,
}

impl AppState {
    fn new() -> Self {
        Self {
            show_windows: true,
            show_damage_distribution: true,
            show_damage_bars: true,
            widget_opacity: 0.5,
        }
    }
}

pub fn ui(ctx: &Context, app_state: &mut AppState) {
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

    let opacity = app_state.widget_opacity.clamp(0.0, 1.0);
    let window_frame = egui::Frame::none()
        .fill(Color32::from_black_alpha((255.0 * opacity) as u8));
        
    let mut style = (*ctx.style()).clone();
    style.visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(240, 240, 240);
    style.visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(240, 240, 240);
    style.visuals.widgets.hovered.fg_stroke.color = Color32::from_rgb(255, 255, 255);
    style.visuals.widgets.active.fg_stroke.color = Color32::from_rgb(255, 255, 255);
    
    ctx.set_style(style);

    egui::containers::Window::new("Main menu")
        .frame(window_frame)
        .show(ctx, |ui| {
            if ui.button(if app_state.show_windows {
                "Hide All Windows"
            } else {
                "Show All Windows"
            }).clicked() {
                app_state.show_windows = !app_state.show_windows;
            }
            
            ui.separator();
            
            ui.heading("Widget Settings");
            
            ui.checkbox(&mut app_state.show_damage_distribution, "Show Damage Distribution");
            ui.checkbox(&mut app_state.show_damage_bars, "Show Damage Bars");
            
            ui.add(
                Slider::new(&mut app_state.widget_opacity, 0.0..=1.0)
                    .text("Window Opacity")
            );
        });

    if app_state.show_windows {
        if app_state.show_damage_distribution {
            egui::containers::Window::new("Damage Distribution")
                .frame(window_frame)
                .show(ctx, |ui| {
                    widgets::show_damage_distribution_widget(app_state, ui);
                });
        }

        if app_state.show_damage_bars {
            egui::containers::Window::new("Damage by Character")
                .frame(window_frame)
                .show(ctx, |ui| {
                    widgets::show_damage_bar_widget(app_state, ui);
                });
        }
    }
}
