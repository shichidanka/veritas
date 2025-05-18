use edio11::{input::InputResult, Overlay, WindowMessage, WindowProcessOptions};
use egui::Key;
use egui::KeyboardShortcut;
use egui::Label;
use egui::Modifiers;
use egui::Stroke;
use egui::TextEdit;
use egui::{
    epaint::text::{FontInsert, InsertFontFamily},
    CentralPanel, Color32, Context, Frame, Slider, Window,
};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::{Input::KeyboardAndMouse::VK_MENU, WindowsAndMessaging::WM_KEYDOWN},
};

use crate::kreide::functions::unityengine::Application_set_targetFrameRate;
use crate::LOCALES;

use super::config::Config;

#[derive(Default, PartialEq)]
pub enum GraphUnit {
    #[default]
    Turn,
    ActionValue,
}

#[derive(Default)]
pub struct App {
    pub show_menu: bool,
    pub show_console: bool,
    show_damage_distribution: bool,
    show_damage_bars: bool,
    show_real_time_damage: bool,
    show_enemy_stats: bool,
    show_av_metrics: bool,
    widget_opacity: f32,
    pub graph_x_unit: GraphUnit,
    pub should_hide: bool,
    streamer_mode: bool,
    streamer_msg: String,
    fps: i32,
    config: Config
}

pub const HIDE_UI: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::H);
pub const SHOW_MENU: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::M);

impl Overlay for App {
    fn update(&mut self, ctx: &egui::Context) {
        if ctx.input_mut(|i| i.consume_shortcut(&HIDE_UI)) {
            self.should_hide = !self.should_hide;
        }

        if !self.should_hide {
            if self.show_menu {
                egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
                    egui::menu::bar(ui, |ui| {
                        ui.menu_button(t!("Language"), |ui| {
                            for locale_code in rust_i18n::available_locales!() {
                                if let Some(locale) = LOCALES.get(locale_code) {
                                    if ui.button(*locale).clicked() {
                                        self.config.set_locale(locale_code.to_string());
                                        rust_i18n::set_locale(locale_code);
                                        ui.close_menu();
                                    }
                                }
                            }
                        });
                    });
                });
            }
        }

        if self.streamer_mode {
            egui::TopBottomPanel::bottom("statusbar")
                .resizable(true)
                .show(ctx, |ui| {
                    let label = Label::new(
                        &self.streamer_msg
                    ).selectable(false);
                    
                    ui.add(label);
                    ui.allocate_space(ui.available_size())
            });
        }

        if !self.should_hide {
            if self.show_menu {
                CentralPanel::default()
                    .frame(Frame {
                        fill: Color32::GRAY.gamma_multiply(0.25),
                        ..Default::default()
                    })
                    .show(ctx, |_ui: &mut egui::Ui| {
                        Window::new(t!("Menu"))
                            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                            .resizable(false)
                            .show(ctx, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading(t!("Widget Controls"));

                                    ui.checkbox(&mut self.streamer_mode, t!("Streamer Mode"));
                                    ui.checkbox(&mut self.show_console, t!("Show Logs"));
                                    ui.checkbox(
                                        &mut self.show_damage_distribution,
                                        t!("Show Damage Distribution"),
                                    );
                                    ui.checkbox(&mut self.show_damage_bars, t!("Show Damage Bars"));
                                    ui.checkbox(
                                        &mut self.show_real_time_damage,
                                        t!("Show Real-Time Damage"),
                                    );
                                    ui.checkbox(
                                        &mut self.show_enemy_stats,
                                        t!("Show Enemy Stats"),
                                    );

                                    ui.checkbox(&mut self.show_av_metrics, t!("Show AV Metrics"));

                                    ui.separator();
                                    ui.label(t!("Window Opacity"));
                                    ui.add(
                                        Slider::new(&mut self.widget_opacity, 0.0..=1.0).text(""),
                                    );

                                    ui.separator();
                                    ui.label(t!("FPS"));
                                    if ui.add(
                                        Slider::new(&mut self.fps, 1..=500).text("")
                                    ).changed() {
                                        self.config.set_fps(self.fps);
                                        Application_set_targetFrameRate(self.fps);
                                    }

                                    ui.separator();
                                    ui.label(t!("Streamer Message"));
                                    ui.add(
                                        TextEdit::singleline(&mut self.streamer_msg),
                                    );

                                    ui.separator();
                                    if ui.button(t!("Close")).clicked() {
                                        self.show_menu = false;
                                    }
                                });
                            });
                    });
            }

            if self.show_console {
                egui::Window::new(t!("Log"))
                    .resizable(true)
                    .default_height(300.0)
                    .default_width(400.0)
                    .min_width(200.0)
                    .min_height(100.0)
                    .show(ctx, |ui| {
                        let available = ui.available_size();
                        ui.set_min_size(available);
                        ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                            egui_logger::logger_ui().show(ui);
                        });
                    });
            }

            let opacity = self.widget_opacity.clamp(0.0, 1.0);
            let window_frame = egui::Frame::new()
                .fill(Color32::from_black_alpha((255.0 * opacity) as u8))
                .stroke(Stroke::new(0.5, Color32::WHITE))
                .inner_margin(8.0)
                .corner_radius(10.0);

            let transparent_frame = egui::Frame::new()
                .inner_margin(8.0)
                .corner_radius(10.0);

            if self.show_damage_distribution {
                egui::containers::Window::new("")
                    .id("Damage Distribution".into())
                    .frame(if self.show_menu { window_frame } else { transparent_frame })
                    .collapsible(false)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        self.show_damage_distribution_widget(ui);
                    });
            }

            if self.show_damage_bars {
                egui::containers::Window::new(t!("Damage by Character"))
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        self.show_damage_bar_widget(ui);
                    });
            }

            if self.show_real_time_damage {
                egui::containers::Window::new(t!("Real-Time Damage"))
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(200.0)
                    .show(ctx, |ui| {
                        self.show_real_time_damage_graph(ui);
                    });
            }

            if self.show_av_metrics {
                egui::containers::Window::new(t!("Action Value Metrics"))
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(150.0)
                    .show(ctx, |ui| {
                        self.show_av_metrics(ui);
                    });
            }

            if self.show_enemy_stats {
                egui::containers::Window::new(t!("Enemy Stats"))
                    .frame(window_frame)
                    .resizable(true)
                    .min_width(200.0)
                    .min_height(150.0)
                    .show(ctx, |ui| {
                        self.show_enemy_stats(ui);
                    });
            }

        }
    }

    fn window_process(
        &mut self,
        input: &InputResult,
        input_events: &Vec<egui::Event>,
    ) -> Option<WindowProcessOptions> {
        // Refactor later
        match input {
            InputResult::Key => {
                for e in input_events {
                    match e {
                        egui::Event::Key {
                            key,
                            physical_key: _,
                            pressed,
                            repeat: _,
                            modifiers,
                        } => {
                            if modifiers.matches_exact(SHOW_MENU.modifiers) && *key == SHOW_MENU.logical_key && *pressed {
                                self.show_menu = !self.show_menu;

                                return Some(WindowProcessOptions {
                                    // Simulate alt to get cursor
                                    window_message: Some(WindowMessage {
                                        msg: WM_KEYDOWN,
                                        wparam: WPARAM(VK_MENU.0 as _),
                                        lparam: LPARAM(0),
                                    }),
                                    ..Default::default()
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        };

        if self.show_menu {
            Some(WindowProcessOptions {
                should_capture_all_input: true,
                ..Default::default()
            })
        }
        else {
            Some(WindowProcessOptions::default())
        }
    }
}

impl App {
    pub fn new(ctx: Context) -> Self {
        let path = r"StarRail_Data\StreamingAssets\MiHoYoSDKRes\HttpServerResources\font\zh-cn.ttf";
        match std::fs::read(path) {
            Ok(font) => {
                // Start with the default fonts (we will be adding to them rather than replacing them).
                ctx.add_font(FontInsert::new(
                    "game_font",
                    egui::FontData::from_owned(font),
                    vec![
                        InsertFontFamily {
                            family: egui::FontFamily::Proportional,
                            priority: egui::epaint::text::FontPriority::Highest,
                        },
                        InsertFontFamily {
                            family: egui::FontFamily::Monospace,
                            priority: egui::epaint::text::FontPriority::Lowest,
                        },
                    ],
                ));
            }
            Err(e) => log::warn!(
                "{} : Could not locate {}. Defaulting to default font.",
                e,
                path
            ),
        }

        ctx.style_mut(|style| {
            style.visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        });

        let config = Config::new().unwrap();
        let fps = config.get_fps().clone();

        let app = Self {
            widget_opacity: 0.15,
            streamer_mode: true,
            config,
            fps,
            ..Default::default()
        };

        app.initialize_settings();
        app
    }

    fn initialize_settings(&self) {
        rust_i18n::set_locale(&self.config.get_locale());
        Application_set_targetFrameRate(*self.config.get_fps());
    }
}
