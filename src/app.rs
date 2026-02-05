use crate::data::{create_sample_document, Node, PdxDocument};
use crate::parser::{parse_content, serialize_content};
use crate::renderer::render_node;
use crate::theme::AppTheme;
use crate::ui::{export_html, export_pdf_file, export_png_file, insert_image, open_document, save_document};
use eframe::egui::{self, ColorImage, RichText, ScrollArea};
use std::collections::HashMap;
use std::path::PathBuf;

// ============================================================================
// Editor State
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorMode {
    Edit,
    Preview,
    Split,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EditorTab {
    Editor,
    Metadata,
    Styles,
}

pub struct PdxApp {
    document: PdxDocument,
    path: Option<PathBuf>,
    mode: EditorMode,
    active_tab: EditorTab,
    theme: AppTheme,
    raw_content: String,
    zoom_level: f32,
    last_save: Option<String>,
    status_message: String,
    loaded_images: HashMap<String, egui::TextureHandle>,
}

impl Default for PdxApp {
    fn default() -> Self {
        let document = create_sample_document();
        let raw_content = serialize_content(&document.content);

        Self {
            document,
            path: None,
            mode: EditorMode::Split,
            active_tab: EditorTab::Editor,
            theme: AppTheme::default(),
            raw_content,
            zoom_level: 1.0,
            last_save: None,
            status_message: "Ready".to_string(),
            loaded_images: HashMap::new(),
        }
    }
}

// ============================================================================
// Main Application
// ============================================================================

impl eframe::App for PdxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme.apply(ctx);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("📄 PDX Editor");
                ui.separator();

                self.render_file_menu(ui);
                self.render_view_menu(ui);
                self.render_theme_menu(ui);

                ui.separator();

                ui.selectable_value(&mut self.active_tab, EditorTab::Editor, "✏️ Editor");
                ui.selectable_value(&mut self.active_tab, EditorTab::Metadata, "ℹ️ Metadata");
                ui.selectable_value(&mut self.active_tab, EditorTab::Styles, "🎨 Styles");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            EditorTab::Editor => {
                self.render_editor_tab(ui, ctx);
            }
            EditorTab::Metadata => {
                self.render_metadata_tab(ui);
            }
            EditorTab::Styles => {
                self.render_styles_tab(ui);
            }
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.render_status_bar(ui);
        });
    }
}

impl PdxApp {
    fn render_file_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("📁 File", |ui| {
            if ui.button("🆕 New").clicked() {
                *self = Self::default();
                self.status_message = "New document created".to_string();
                ui.close_menu();
            }

            if ui.button("📂 Open...").clicked() {
                if let Some((doc, path)) = open_document() {
                    self.document = doc;
                    self.path = Some(path.clone());
                    self.raw_content = serialize_content(&self.document.content);
                    self.status_message = format!("Opened: {}", path.display());
                }
                ui.close_menu();
            }

            if ui.button("💾 Save").clicked() {
                if let Some(path) = save_document(&self.document, self.path.as_ref()) {
                    self.path = Some(path.clone());
                    self.last_save = Some(chrono::Local::now().format("%H:%M:%S").to_string());
                    self.status_message = format!("Saved: {}", path.display());
                }
                ui.close_menu();
            }

            if ui.button("💾 Save As...").clicked() {
                if let Some(path) = save_document(&self.document, None) {
                    self.path = Some(path.clone());
                    self.last_save = Some(chrono::Local::now().format("%H:%M:%S").to_string());
                    self.status_message = format!("Saved as: {}", path.display());
                }
                ui.close_menu();
            }

            ui.separator();

            ui.menu_button("📤 Export as...", |ui| {
                if ui.button("🌐 HTML").clicked() {
                    if export_html(&self.document).is_some() {
                        self.status_message = "Exported as HTML".to_string();
                    }
                    ui.close_menu();
                }

                if ui.button("📄 PDF").clicked() {
                    if export_pdf_file(&self.document).is_some() {
                        self.status_message = "Exported as PDF with Arabic support".to_string();
                    } else {
                        self.status_message = "PDF export failed".to_string();
                    }
                    ui.close_menu();
                }

                if ui.button("🖼️ PNG Image").clicked() {
                    if export_png_file().is_some() {
                        self.status_message = "Exported as PNG image".to_string();
                    } else {
                        self.status_message = "PNG export failed".to_string();
                    }
                    ui.close_menu();
                }
            });

            ui.separator();

            if ui.button("🖼️ Insert Image...").clicked() {
                if let Some(image_path) = insert_image() {
                    let image_markup = format!("\n![Image]({})\n", image_path);
                    self.raw_content.push_str(&image_markup);
                    self.document.content = parse_content(&self.raw_content);
                    self.status_message = "Image inserted".to_string();
                }
                ui.close_menu();
            }
        });
    }

    fn render_view_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("👁 View", |ui| {
            if ui.button("✏️ Edit Mode").clicked() {
                self.mode = EditorMode::Edit;
                ui.close_menu();
            }
            if ui.button("🔍 Preview Mode").clicked() {
                self.mode = EditorMode::Preview;
                ui.close_menu();
            }
            if ui.button("⚡ Split Mode").clicked() {
                self.mode = EditorMode::Split;
                ui.close_menu();
            }

            ui.separator();

            ui.label("Zoom:");
            if ui.button("🔍+ Zoom In").clicked() {
                self.zoom_level = (self.zoom_level + 0.1).min(2.5);
            }
            if ui.button("🔍- Zoom Out").clicked() {
                self.zoom_level = (self.zoom_level - 0.1).max(0.5);
            }
            if ui.button("🔍 Reset").clicked() {
                self.zoom_level = 1.0;
            }
        });
    }

    fn render_theme_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("🎨 Theme", |ui| {
            if ui
                .selectable_label(self.theme == AppTheme::Light, "☀️ Light")
                .clicked()
            {
                self.theme = AppTheme::Light;
                self.status_message = "Theme changed to Light".to_string();
                ui.close_menu();
            }
            if ui
                .selectable_label(self.theme == AppTheme::Dark, "🌙 Dark")
                .clicked()
            {
                self.theme = AppTheme::Dark;
                self.status_message = "Theme changed to Dark".to_string();
                ui.close_menu();
            }
            if ui
                .selectable_label(self.theme == AppTheme::Midnight, "🌌 Midnight")
                .clicked()
            {
                self.theme = AppTheme::Midnight;
                self.status_message = "Theme changed to Midnight".to_string();
                ui.close_menu();
            }
            if ui
                .selectable_label(self.theme == AppTheme::Sepia, "📜 Sepia")
                .clicked()
            {
                self.theme = AppTheme::Sepia;
                self.status_message = "Theme changed to Sepia".to_string();
                ui.close_menu();
            }
            if ui
                .selectable_label(self.theme == AppTheme::Comfort, "🌿 Comfort")
                .clicked()
            {
                self.theme = AppTheme::Comfort;
                self.status_message =
                    "Theme changed to Comfort (Eye-friendly for long sessions)".to_string();
                ui.close_menu();
            }
        });
    }

    fn render_status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(&self.status_message);
            ui.separator();

            if let Some(path) = &self.path {
                ui.label(format!(
                    "📁 {}",
                    path.file_name().unwrap().to_string_lossy()
                ));
            } else {
                ui.label("📁 Unsaved");
            }

            ui.separator();
            ui.label(format!("🔍 {}%", (self.zoom_level * 100.0) as i32));

            ui.separator();
            ui.label(format!("🌍 {}", self.document.metadata.language));

            ui.separator();
            ui.label(format!("🎨 {}", self.theme.name()));

            if let Some(save_time) = &self.last_save {
                ui.separator();
                ui.label(format!("💾 {}", save_time));
            }
        });
    }

    fn render_editor_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.load_images_from_content(ctx);

        match self.mode {
            EditorMode::Edit => {
                ScrollArea::vertical()
                    .id_salt("edit_scroll")
                    .show(ui, |ui| {
                        ui.heading("Editor");

                        let editor = egui::TextEdit::multiline(&mut self.raw_content)
                            .desired_width(f32::INFINITY)
                            .desired_rows(30)
                            .font(egui::TextStyle::Monospace);

                        if ui.add(editor).changed() {
                            self.document.content = parse_content(&self.raw_content);
                        }
                    });
            }

            EditorMode::Preview => {
                ScrollArea::vertical()
                    .id_salt("preview_scroll")
                    .show(ui, |ui| {
                        ui.heading("Preview");
                        ui.separator();
                        render_node(
                            ui,
                            &self.document.content,
                            &self.document.styles,
                            self.zoom_level,
                            &self.theme,
                            &self.loaded_images,
                        );
                    });
            }

            EditorMode::Split => {
                ui.columns(2, |cols| {
                    ScrollArea::vertical()
                        .id_salt("split_edit_scroll")
                        .show(&mut cols[0], |ui| {
                            ui.heading("Editor");

                            let editor = egui::TextEdit::multiline(&mut self.raw_content)
                                .desired_width(f32::INFINITY)
                                .desired_rows(30)
                                .font(egui::TextStyle::Monospace);

                            if ui.add(editor).changed() {
                                self.document.content = parse_content(&self.raw_content);
                            }
                        });

                    ScrollArea::vertical()
                        .id_salt("split_preview_scroll")
                        .show(&mut cols[1], |ui| {
                            ui.heading("Preview");
                            ui.separator();
                            render_node(
                                ui,
                                &self.document.content,
                                &self.document.styles,
                                self.zoom_level,
                                &self.theme,
                                &self.loaded_images,
                            );
                        });
                });
            }
        }
    }

    fn load_images_from_content(&mut self, ctx: &egui::Context) {
        fn collect_image_paths(node: &Node, paths: &mut Vec<String>) {
            match node {
                Node::Document { children } => {
                    for child in children {
                        collect_image_paths(child, paths);
                    }
                }
                Node::Image { path, .. } => {
                    paths.push(path.clone());
                }
                _ => {}
            }
        }

        let mut image_paths = Vec::new();
        collect_image_paths(&self.document.content, &mut image_paths);

        for path in image_paths {
            if !self.loaded_images.contains_key(&path) {
                if let Ok(img) = image::open(&path) {
                    let size = [img.width() as usize, img.height() as usize];
                    let rgba = img.to_rgba8();
                    let pixels = rgba.as_flat_samples();

                    let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

                    let texture =
                        ctx.load_texture(&path, color_image, egui::TextureOptions::default());

                    self.loaded_images.insert(path, texture);
                }
            }
        }
    }

    fn render_metadata_tab(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .id_salt("metadata_scroll")
            .show(ui, |ui| {
                ui.heading("Document Metadata");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Title:");
                    ui.text_edit_singleline(&mut self.document.metadata.title);
                });

                ui.horizontal(|ui| {
                    ui.label("Author:");
                    ui.text_edit_singleline(&mut self.document.metadata.author);
                });

                ui.horizontal(|ui| {
                    ui.label("Language:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.document.metadata.language)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.document.metadata.language,
                                "ar".to_string(),
                                "🇸🇦 Arabic",
                            );
                            ui.selectable_value(
                                &mut self.document.metadata.language,
                                "en".to_string(),
                                "🇬🇧 English",
                            );
                            ui.selectable_value(
                                &mut self.document.metadata.language,
                                "fr".to_string(),
                                "🇫🇷 French",
                            );
                        });
                });

                ui.separator();

                ui.label(format!("Created: {}", self.document.metadata.created));
                ui.label(format!("Modified: {}", self.document.metadata.modified));

                ui.separator();

                ui.label("Keywords:");
                for keyword in &self.document.metadata.keywords {
                    ui.label(format!("  • {}", keyword));
                }
            });
    }

    fn render_styles_tab(&mut self, ui: &mut egui::Ui) {
        ScrollArea::vertical()
            .id_salt("styles_scroll")
            .show(ui, |ui| {
                ui.heading("Document Styles");
                ui.separator();

                for (name, style) in &self.document.styles.styles {
                    ui.group(|ui| {
                        ui.heading(name);
                        ui.label(format!("Font Size: {}pt", style.font_size));
                        ui.label(format!("Font Weight: {:?}", style.font_weight));
                        ui.label(format!("Text Align: {:?}", style.text_align));
                        ui.label(format!("Direction: {:?}", style.direction));
                        ui.label(format!("Line Height: {}", style.line_height));
                    });
                    ui.add_space(8.0);
                }
            });
    }
}