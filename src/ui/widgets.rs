use egui::{Stroke, Ui};
use egui_plot::{Legend, Plot, PlotPoints, Polygon};


use crate::{battle::BattleContext, models::misc::Avatar};

use super::{app::AppState, core::helpers};

pub struct PieSegment {
    pub points: Vec<[f64; 2]>,
    pub value: f64,
}

pub fn show_damage_distribution_widget(app_state: &mut AppState, ui: &mut Ui) {
    let battle_context = BattleContext::get_instance();
    Plot::new("damage_pie")
        .legend(Legend::default().position(egui_plot::Corner::RightTop))
        .height(300.0)
        .width(ui.available_width() * 0.5)
        .data_aspect(1.0)
        .clamp_grid(true)
        .show_grid(false)
        .show_background(false)
        .show_axes([false; 2])
        .allow_drag(false)
        .allow_zoom(false)
        .allow_scroll(false)
        .show(ui, |plot_ui: &mut egui_plot::PlotUi<'_>| {
            let total_damage = battle_context.total_damage as f64;
            if total_damage > 0.0 {
                let segments =
                    create_pie_segments(&battle_context.real_time_damages, &battle_context.lineup);
                for (avatar, segment, i) in segments {
                    let color = helpers::get_character_color(i);
                    let percentage = segment.value / total_damage * 100.0;

                    let plot_points = PlotPoints::new(segment.points);
                    let polygon = Polygon::new(plot_points)
                        .stroke(Stroke::new(1.5, color))
                        .name(format!(
                            "{}: {:.1}% ({} dmg)",
                            avatar.name,
                            percentage,
                            helpers::format_damage(segment.value)
                        ));

                    plot_ui.polygon(polygon);
                }
            }
    });
}


fn create_pie_segments(real_time_damages: &Vec<f64>, avatars: &Vec<Avatar>) -> Vec<(Avatar, PieSegment, usize)> {
    let total = real_time_damages.into_iter().sum::<f64>();
    let mut segments = Vec::new();
    let mut start_angle = -std::f64::consts::FRAC_PI_2; 

    for (i, name) in avatars.iter().enumerate() {
        let damage = real_time_damages[i];
        let fraction = damage as f64 / total;
        let angle = fraction * std::f64::consts::TAU;
        let end_angle = start_angle + angle;

        segments.push((
            name.clone(),
            PieSegment {
                points: create_pie_slice(start_angle, end_angle),
                value: damage as f64,
            },
            i
        ));

        start_angle = end_angle;
    }

    segments
}

fn create_pie_slice(start_angle: f64, end_angle: f64) -> Vec<[f64; 2]> {
    let center = [0.0, 0.0];
    let radius = 0.8; 
    let mut points = vec![center];
    
    let steps = 50;
    let p = (end_angle - start_angle)/(steps as f64);
    for i in 0..=steps {
        let angle = start_angle + p*i as f64;
        let (sin, cos) = angle.sin_cos();
        points.push([cos * radius, sin * radius]);
    }
    points.push(center);
    
    points
}

