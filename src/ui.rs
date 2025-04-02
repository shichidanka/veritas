use std::{
    mem::{self, transmute},
    sync::Once,
};

use egui::{
    Align2, Color32, Context, FontId, Key, Modifiers, Pos2, Rect, RichText, ScrollArea, Slider,
    Stroke, StrokeKind, Widget,
};
use egui_directx11::app::EguiDx11;
use windows::{
    core::HRESULT,
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        Graphics::
            Dxgi::{IDXGISwapChain_Vtbl, DXGI_PRESENT}
        ,
        UI::WindowsAndMessaging::{
            CallWindowProcW, SetWindowLongPtrA, GWLP_WNDPROC,
            WNDPROC,
        },
    },
};

// This is so bad let's not do this
use crate::directx;


static mut APP: Option<EguiDx11<i32>> = None;
static mut OLD_WND_PROC: Option<WNDPROC> = None;
// UnsafeCell
unsafe extern "stdcall" fn hk_wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    APP.as_mut().unwrap().wnd_proc(msg, wparam, lparam);

    CallWindowProcW(OLD_WND_PROC.unwrap(), hwnd, msg, wparam, lparam)
}

pub fn present(
    swap_chain_vtbl: *const IDXGISwapChain_Vtbl,
    sync_interval: u32,
    flags: DXGI_PRESENT,
) -> HRESULT {
    unsafe {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            unsafe {
                // struct State {
                //     counter: u32,
                // }
                // let mut shared_state = State { counter: 5 };
                APP = Some(EguiDx11::init_with_state(mem::transmute(&(swap_chain_vtbl)), ui, 0));
                OLD_WND_PROC = Some(transmute(SetWindowLongPtrA(
                    APP.as_ref().unwrap().hwnd,
                    GWLP_WNDPROC,
                    hk_wnd_proc as usize as _,
                )));
            }
        });

        APP.as_mut().unwrap().present(mem::transmute(&(swap_chain_vtbl)));

        unsafe { directx::Present_Detour.call(swap_chain_vtbl, sync_interval, flags) }
    }
}

static mut FRAME: i32 = 0;
fn ui(ctx: &Context, i: &mut i32) {
    unsafe {
        // You should not use statics like this, it's made
        // this way for the sake of example.
        static mut UI_CHECK: bool = true;
        static mut TEXT: Option<String> = None;
        static mut VALUE: f32 = 0.;
        static mut COLOR: [f32; 3] = [0., 0., 0.];
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

        if TEXT.is_none() {
            TEXT = Some(String::from("Test"));
        }

        ctx.debug_painter().text(
            Pos2::new(0., 0.),
            Align2::LEFT_TOP,
            "Bruh",
            FontId::default(),
            Color32::RED,
        );

        egui::containers::Window::new("Main menu").show(ctx, |ui| {
            ctx.settings_ui(ui);
            ui.label(RichText::new("Test").color(Color32::BLACK));
            ui.label(RichText::new("Other").color(Color32::WHITE));
            ui.separator();

            ui.label(RichText::new(format!("I: {}", *i)).color(Color32::LIGHT_RED));

            let input = ctx.input(|input| input.pointer.clone());
            ui.label(format!(
                "X1: {} X2: {}",
                input.button_down(egui::PointerButton::Extra1),
                input.button_down(egui::PointerButton::Extra2)
            ));

            let mods = ui.input(|input| input.modifiers);
            ui.label(format!(
                "Ctrl: {} Shift: {} Alt: {}",
                mods.ctrl, mods.shift, mods.alt
            ));

            if ui.input(|input| {
                input.modifiers.matches(Modifiers::CTRL) && input.key_pressed(Key::R)
            }) {
                println!("Pressed");
            }

            unsafe {
                ui.checkbox(&mut UI_CHECK, "Some checkbox");
                ui.text_edit_singleline(TEXT.as_mut().unwrap());
                ScrollArea::vertical().max_height(200.).show(ui, |ui| {
                    for i in 1..=100 {
                        ui.label(format!("Label: {}", i));
                    }
                });

                Slider::new(&mut VALUE, -1.0..=1.0).ui(ui);

                ui.color_edit_button_rgb(&mut COLOR);
            }

            ui.label(format!(
                "{:?}",
                &ui.input(|input| input.pointer.button_down(egui::PointerButton::Primary))
            ));
            if ui.button("You can't click me yet").clicked() {
                *i += 1;
            }
        });

        // egui::Window::new("Image").show(ctx, |ui| unsafe {
        //     const IMG: ImageSource = egui::include_image!("logo.bmp");

        //     ui.image(IMG);
        // });

        egui::Window::new("xd").show(ctx, |ui| unsafe {
            ctx.memory_ui(ui);
        });

        egui::Window::new("stuff").show(ctx, |ui| unsafe {
            ctx.inspection_ui(ui);
        });

        ctx.debug_painter().rect(
            Rect {
                min: Pos2::new(200.0, 200.0),
                max: Pos2::new(250.0, 250.0),
            },
            10.0,
            Color32::from_rgba_premultiplied(255, 0, 0, 150),
            Stroke::NONE,
            StrokeKind::Inside,
        );

        // this is supposed to be color channel testing to identify if any channels have been misplaced
        ctx.debug_painter().circle(
            Pos2::new(350.0, 350.0),
            35.0,
            Color32::from_rgba_premultiplied(255, 0, 0, 0),
            Stroke::NONE,
        );

        ctx.debug_painter().circle(
            Pos2::new(450.0, 350.0),
            35.0,
            Color32::from_rgba_premultiplied(0, 255, 0, 0),
            Stroke::NONE,
        );

        ctx.debug_painter().circle(
            Pos2::new(550.0, 350.0),
            35.0,
            Color32::from_rgba_premultiplied(0, 0, 255, 0),
            Stroke::NONE,
        );

        ctx.debug_painter().circle(
            Pos2::new(650.0, 350.0),
            35.0,
            Color32::from_rgba_premultiplied(0, 0, 0, 255),
            Stroke::new(5f32, Color32::from_rgba_premultiplied(0, 0, 255, 255)),
        );
    }
}
