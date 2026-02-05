use eframe::egui;

// ============================================================================
// Theme System
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppTheme {
    Dark,
    Light,
    Sepia,
    Midnight,
    Comfort, // Eye-friendly theme for long writing sessions
}

impl AppTheme {
    pub fn text_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(20, 20, 20),
            AppTheme::Dark => egui::Color32::from_rgb(230, 230, 230),
            AppTheme::Sepia => egui::Color32::from_rgb(60, 50, 40),
            AppTheme::Midnight => egui::Color32::from_rgb(200, 210, 230),
            AppTheme::Comfort => egui::Color32::from_rgb(45, 55, 65), // Soft blue-gray
        }
    }

    pub fn background_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(250, 250, 250),
            AppTheme::Dark => egui::Color32::from_rgb(30, 30, 35),
            AppTheme::Sepia => egui::Color32::from_rgb(245, 235, 215),
            AppTheme::Midnight => egui::Color32::from_rgb(15, 20, 35),
            AppTheme::Comfort => egui::Color32::from_rgb(248, 250, 245), // Very soft green tint
        }
    }

    pub fn panel_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(255, 255, 255),
            AppTheme::Dark => egui::Color32::from_rgb(40, 40, 45),
            AppTheme::Sepia => egui::Color32::from_rgb(255, 248, 235),
            AppTheme::Midnight => egui::Color32::from_rgb(25, 30, 50),
            AppTheme::Comfort => egui::Color32::from_rgb(252, 253, 250), // Warm white with green tint
        }
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = match self {
            AppTheme::Light | AppTheme::Sepia | AppTheme::Comfort => egui::Visuals::light(),
            AppTheme::Dark | AppTheme::Midnight => egui::Visuals::dark(),
        };

        visuals.override_text_color = Some(self.text_color());
        visuals.panel_fill = self.panel_color();
        visuals.window_fill = self.panel_color();
        visuals.extreme_bg_color = self.background_color();

        if matches!(self, AppTheme::Midnight) {
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(35, 40, 60);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 45, 65);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 60, 85);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(60, 70, 100);
        }

        if matches!(self, AppTheme::Comfort) {
            // Reduced contrast for comfortable reading
            visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(245, 248, 243);
            visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(240, 245, 238);
            visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(230, 240, 225);
            visuals.widgets.active.bg_fill = egui::Color32::from_rgb(220, 235, 215);
        }

        ctx.set_visuals(visuals);
    }

    pub fn name(&self) -> &str {
        match self {
            AppTheme::Dark => "Dark",
            AppTheme::Light => "Light",
            AppTheme::Sepia => "Sepia",
            AppTheme::Midnight => "Midnight",
            AppTheme::Comfort => "Comfort",
        }
    }

    pub fn all_themes() -> Vec<AppTheme> {
        vec![
            AppTheme::Light,
            AppTheme::Dark,
            AppTheme::Midnight,
            AppTheme::Sepia,
            AppTheme::Comfort,
        ]
    }
}

impl Default for AppTheme {
    fn default() -> Self {
        AppTheme::Comfort // Default to comfort theme
    }
}