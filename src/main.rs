#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod app;
mod data;
mod export;
mod parser;
mod pdx_text;
mod renderer;
mod theme;
mod ui;

use app::PdxApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 800.0])
            .with_min_inner_size([1024.0, 768.0])
            .with_title("PDX Editor")
            .with_drag_and_drop(true),

        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "com.terabit.pdxeditor",
        options,
        Box::new(|cc| {
            // Apply advanced visual styling
            configure_comfort_theme(&cc.egui_ctx);

            // Setup custom fonts (Arabic/Unicode support)
            ui::setup_fonts(&cc.egui_ctx);

            Ok(Box::new(PdxApp::default()))
        }),
    )
}

fn configure_comfort_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();

    // --- Modern Geometry ---
    visuals.window_rounding = 12.0.into();
    visuals.widgets.noninteractive.rounding = 8.0.into();
    visuals.widgets.inactive.rounding = 6.0.into();
    visuals.widgets.hovered.rounding = 6.0.into();
    visuals.widgets.active.rounding = 6.0.into();

    // --- Refined Color Palette ---
    visuals.extreme_bg_color = egui::Color32::from_rgb(20, 20, 22);
    visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 33);
    visuals.widgets.noninteractive.bg_stroke =
        egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 45, 48));

    // --- Corrected Spacing & Style ---
    let mut style = (*ctx.style()).clone();

    style.spacing.menu_margin = egui::Margin::same(8.0);

    ctx.set_style(style);
    ctx.set_visuals(visuals);
}
