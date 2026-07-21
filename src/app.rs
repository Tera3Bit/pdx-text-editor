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
    PdfViewer,
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

    // Find & Replace state
    find_text: String,
    replace_text: String,
    show_find_replace: bool,
    find_match_count: usize,

    // PDF Viewer state
    pdf_preview_texture: Option<egui::TextureHandle>,
    pdf_needs_refresh: bool,
    pdf_page_count: usize,

    // Word count
    word_count: usize,
    char_count: usize,
}

impl Default for PdxApp {
    fn default() -> Self {
        let document = create_sample_document();
        let raw_content = serialize_content(&document.content);
        let (wc, cc) = count_words_chars(&raw_content);

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
            find_text: String::new(),
            replace_text: String::new(),
            show_find_replace: false,
            find_match_count: 0,
            pdf_preview_texture: None,
            pdf_needs_refresh: true,
            pdf_page_count: 0,
            word_count: wc,
            char_count: cc,
        }
    }
}

fn count_words_chars(text: &str) -> (usize, usize) {
    let chars = text.chars().filter(|c| !c.is_whitespace()).count();
    let words = text.split_whitespace().count();
    (words, chars)
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
                self.render_edit_menu(ui);
                self.render_view_menu(ui);
                self.render_theme_menu(ui);

                ui.separator();

                ui.selectable_value(&mut self.active_tab, EditorTab::Editor, "✏️ Editor");
                ui.selectable_value(&mut self.active_tab, EditorTab::PdfViewer, "📄 PDF Viewer");
                ui.selectable_value(&mut self.active_tab, EditorTab::Metadata, "ℹ️ Metadata");
                ui.selectable_value(&mut self.active_tab, EditorTab::Styles, "🎨 Styles");
            });
        });

        // Find & Replace toolbar (only in editor tab)
        if self.show_find_replace && self.active_tab == EditorTab::Editor {
            egui::TopBottomPanel::top("find_replace_bar").show(ctx, |ui| {
                self.render_find_replace_bar(ui);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            EditorTab::Editor => {
                self.render_editor_tab(ui, ctx);
            }
            EditorTab::PdfViewer => {
                self.render_pdf_viewer_tab(ui, ctx);
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
                    let (wc, cc) = count_words_chars(&self.raw_content);
                    self.word_count = wc;
                    self.char_count = cc;
                    self.pdf_needs_refresh = true;
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
                    self.pdf_needs_refresh = true;
                    self.status_message = "Image inserted".to_string();
                }
                ui.close_menu();
            }
        });
    }

    fn render_edit_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("✏️ Edit", |ui| {
            if ui.button("🔍 Find & Replace (Ctrl+H)").clicked() {
                self.show_find_replace = !self.show_find_replace;
                ui.close_menu();
            }

            ui.separator();

            ui.label("Insert formatting:");

            if ui.button("**Bold**").clicked() {
                self.raw_content.push_str("**bold text**");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
                ui.close_menu();
            }

            if ui.button("*Italic*").clicked() {
                self.raw_content.push_str("*italic text*");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
                ui.close_menu();
            }

            if ui.button("__Underline__").clicked() {
                self.raw_content.push_str("__underlined text__");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
                ui.close_menu();
            }

            ui.separator();

            if ui.button("➕ Add Page Break (===)").clicked() {
                self.raw_content.push_str("\n===\n");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
                ui.close_menu();
            }

            if ui.button("➖ Add Divider (---)").clicked() {
                self.raw_content.push_str("\n---\n");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
                ui.close_menu();
            }
        });
    }

    fn render_find_replace_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("🔍 Find:");
            let find_changed = ui.text_edit_singleline(&mut self.find_text).changed();

            ui.label("→ Replace:");
            ui.text_edit_singleline(&mut self.replace_text);

            // Update match count whenever the find text changes
            if find_changed && !self.find_text.is_empty() {
                self.find_match_count = self.raw_content.matches(&self.find_text).count();
            } else if find_changed {
                self.find_match_count = 0;
            }

            // Single Replace All button
            let replace_btn = ui.add_enabled(
                !self.find_text.is_empty(),
                egui::Button::new("Replace All"),
            );
            if replace_btn.clicked() {
                let count = self.raw_content.matches(&self.find_text).count();
                self.raw_content = self.raw_content.replace(&self.find_text, &self.replace_text);
                self.document.content = parse_content(&self.raw_content);
                let (wc, cc) = count_words_chars(&self.raw_content);
                self.word_count = wc;
                self.char_count = cc;
                self.find_match_count = 0;
                self.pdf_needs_refresh = true;
                self.status_message = format!("Replaced {} occurrence(s)", count);
            }

            if !self.find_text.is_empty() {
                ui.label(format!("{} match(es)", self.find_match_count));
            }

            if ui.button("✕ Close").clicked() {
                self.show_find_replace = false;
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
            if ui.selectable_label(self.theme == AppTheme::Light, "☀️ Light").clicked() {
                self.theme = AppTheme::Light;
                self.status_message = "Theme changed to Light".to_string();
                ui.close_menu();
            }
            if ui.selectable_label(self.theme == AppTheme::Dark, "🌙 Dark").clicked() {
                self.theme = AppTheme::Dark;
                self.status_message = "Theme changed to Dark".to_string();
                ui.close_menu();
            }
            if ui.selectable_label(self.theme == AppTheme::Midnight, "🌌 Midnight").clicked() {
                self.theme = AppTheme::Midnight;
                self.status_message = "Theme changed to Midnight".to_string();
                ui.close_menu();
            }
            if ui.selectable_label(self.theme == AppTheme::Sepia, "📜 Sepia").clicked() {
                self.theme = AppTheme::Sepia;
                self.status_message = "Theme changed to Sepia".to_string();
                ui.close_menu();
            }
            if ui.selectable_label(self.theme == AppTheme::Comfort, "🌿 Comfort").clicked() {
                self.theme = AppTheme::Comfort;
                self.status_message = "Theme changed to Comfort (Eye-friendly for long sessions)".to_string();
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
            ui.label(format!("📝 {} words", self.word_count));
            ui.separator();
            ui.label(format!("🔤 {} chars", self.char_count));
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

    // ============================================================================
    // PDF Viewer Tab
    // ============================================================================

    fn render_pdf_viewer_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // Toolbar
        ui.horizontal(|ui| {
            ui.heading("📄 PDF Viewer");
            ui.separator();

            if ui.button("🔄 Refresh Preview").clicked() {
                self.pdf_needs_refresh = true;
            }

            if ui.button("💾 Export PDF...").clicked() {
                if export_pdf_file(&self.document).is_some() {
                    self.status_message = "PDF exported successfully".to_string();
                } else {
                    self.status_message = "PDF export failed".to_string();
                }
            }

            ui.separator();
            ui.label(RichText::new("ℹ️ Live PDF preview of your document").weak().italics());
        });
        ui.separator();

        // Generate PDF and display its content as a structured preview
        if self.pdf_needs_refresh {
            self.pdf_needs_refresh = false;

            // Count pages by checking page breaks
            let page_breaks = self.raw_content.matches("===").count();
            self.pdf_page_count = page_breaks + 1;

            self.status_message = format!(
                "PDF preview ready — {} page(s), {} words",
                self.pdf_page_count,
                self.word_count
            );
        }

        // PDF-style preview panel
        let pdf_bg = match self.theme {
            AppTheme::Dark | AppTheme::Midnight => egui::Color32::from_rgb(40, 40, 45),
            _ => egui::Color32::from_rgb(180, 180, 185),
        };

        let page_bg = egui::Color32::WHITE;
        let text_dark = egui::Color32::from_rgb(20, 20, 20);

        ScrollArea::vertical()
            .id_salt("pdf_viewer_scroll")
            .show(ui, |ui| {
                ui.painter().rect_filled(ui.clip_rect(), 0.0, pdf_bg);

                ui.add_space(16.0);

                // Simulate PDF pages
                let available_width = (ui.available_width() - 80.0).min(680.0);

                // Page info header
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - available_width) / 2.0);
                    ui.label(
                        RichText::new(format!(
                            "📄 Document: \"{}\" — {} page(s) | {} words | PDF Export Ready",
                            self.document.metadata.title,
                            self.pdf_page_count,
                            self.word_count
                        ))
                        .size(12.0)
                        .color(egui::Color32::from_rgb(100, 100, 110)),
                    );
                });
                ui.add_space(8.0);

                // Render the document as a "PDF page" styled frame
                egui::Frame::none()
                    .fill(page_bg)
                    .shadow(egui::epaint::Shadow {
                        offset: egui::Vec2::new(4.0, 4.0),
                        blur: 12.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(80),
                    })
                    .inner_margin(egui::Margin::symmetric(48.0, 40.0))
                    .show(ui, |ui| {
                        ui.set_min_width(available_width);
                        ui.set_max_width(available_width);

                        // Document title header
                        ui.label(
                            RichText::new(&self.document.metadata.title)
                                .size(26.0)
                                .color(text_dark)
                                .strong(),
                        );

                        if !self.document.metadata.author.is_empty() {
                            ui.label(
                                RichText::new(format!("By: {}", self.document.metadata.author))
                                    .size(13.0)
                                    .color(egui::Color32::from_rgb(90, 90, 100))
                                    .italics(),
                            );
                        }

                        ui.add_space(8.0);
                        ui.separator();
                        ui.add_space(12.0);

                        // Render document content in PDF style
                        render_node_pdf_style(
                            ui,
                            &self.document.content,
                            &self.document.styles,
                            self.zoom_level,
                            text_dark,
                            page_bg,
                            &self.loaded_images,
                        );

                        ui.add_space(40.0);

                        // Footer
                        ui.separator();
                        ui.label(
                            RichText::new(format!(
                                "Page 1 of {} | Generated by PDX Editor",
                                self.pdf_page_count
                            ))
                            .size(10.0)
                            .color(egui::Color32::from_rgb(150, 150, 160)),
                        );
                    });

                ui.add_space(24.0);
            });
    }

    fn render_editor_tab(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.load_images_from_content(ctx);

        // Formatting quick toolbar
        ui.horizontal(|ui| {
            ui.label("Format:");
            if ui.small_button("B").on_hover_text("Bold (**text**)").clicked() {
                self.raw_content.push_str("**bold**");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            if ui.small_button("I").on_hover_text("Italic (*text*)").clicked() {
                self.raw_content.push_str("*italic*");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            if ui.small_button("U").on_hover_text("Underline (__text__)").clicked() {
                self.raw_content.push_str("__underline__");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            ui.separator();
            if ui.small_button("H1").on_hover_text("Heading 1 (# text)").clicked() {
                self.raw_content.push_str("\n# Heading\n");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            if ui.small_button("H2").on_hover_text("Heading 2 (## text)").clicked() {
                self.raw_content.push_str("\n## Heading\n");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            if ui.small_button("H3").on_hover_text("Heading 3 (### text)").clicked() {
                self.raw_content.push_str("\n### Heading\n");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            ui.separator();
            if ui.small_button("• List").on_hover_text("Unordered list (- item)").clicked() {
                self.raw_content.push_str("\n- Item 1\n- Item 2\n- Item 3\n");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            if ui.small_button("---").on_hover_text("Horizontal divider").clicked() {
                self.raw_content.push_str("\n---\n");
                self.document.content = parse_content(&self.raw_content);
                self.pdf_needs_refresh = true;
            }
            if ui.small_button("🔍").on_hover_text("Find & Replace").clicked() {
                self.show_find_replace = !self.show_find_replace;
            }
        });
        ui.separator();

        match self.mode {
            EditorMode::Edit => {
                ui.heading("Editor");
                ui.add_space(4.0);
                let available_height = ui.available_height();
                ScrollArea::vertical()
                    .id_salt("edit_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        let editor = egui::TextEdit::multiline(&mut self.raw_content)
                            .desired_width(f32::INFINITY)
                            .min_size(egui::vec2(ui.available_width(), available_height))
                            .font(egui::TextStyle::Monospace);

                        if ui.add(editor).changed() {
                            self.document.content = parse_content(&self.raw_content);
                            let (wc, cc) = count_words_chars(&self.raw_content);
                            self.word_count = wc;
                            self.char_count = cc;
                            self.pdf_needs_refresh = true;
                        }
                    });
            }

            EditorMode::Preview => {
                ui.heading("Preview");
                ui.separator();
                ScrollArea::vertical()
                    .id_salt("preview_scroll")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        render_node(
                            ui,
                            &self.document.content,
                            &self.document.styles,
                            self.zoom_level,
                            &self.theme,
                            &self.loaded_images,
                        );
                        ui.add_space(20.0); // breathing room at the bottom
                    });
            }

            EditorMode::Split => {
                let available_height = ui.available_height();
                ui.columns(2, |cols| {
                    cols[0].heading("Editor");
                    cols[0].add_space(4.0);
                    ScrollArea::vertical()
                        .id_salt("split_edit_scroll")
                        .auto_shrink([false, false])
                        .show(&mut cols[0], |ui| {
                            let editor = egui::TextEdit::multiline(&mut self.raw_content)
                                .desired_width(f32::INFINITY)
                                .min_size(egui::vec2(ui.available_width(), available_height))
                                .font(egui::TextStyle::Monospace);

                            if ui.add(editor).changed() {
                                self.document.content = parse_content(&self.raw_content);
                                let (wc, cc) = count_words_chars(&self.raw_content);
                                self.word_count = wc;
                                self.char_count = cc;
                                self.pdf_needs_refresh = true;
                            }
                        });

                    cols[1].heading("Preview");
                    cols[1].separator();
                    ScrollArea::vertical()
                        .id_salt("split_preview_scroll")
                        .auto_shrink([false, false])
                        .show(&mut cols[1], |ui| {
                            render_node(
                                ui,
                                &self.document.content,
                                &self.document.styles,
                                self.zoom_level,
                                &self.theme,
                                &self.loaded_images,
                            );
                            ui.add_space(20.0); // breathing room at the bottom
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

                ui.label(format!("📝 Word Count: {}", self.word_count));
                ui.label(format!("🔤 Character Count: {}", self.char_count));
                ui.label(format!("📄 Estimated Pages: {}", self.pdf_page_count.max(1)));

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

                ui.collapsing("📖 Formatting Guide", |ui| {
                    ui.label("Use these in the editor:");
                    ui.separator();
                    ui.monospace("**text**  → Bold");
                    ui.monospace("*text*    → Italic");
                    ui.monospace("__text__  → Underline");
                    ui.monospace("# Title   → Heading 1");
                    ui.monospace("## Sub    → Heading 2");
                    ui.monospace("- item    → List");
                    ui.monospace("---       → Divider");
                    ui.monospace("===       → Page Break");
                    ui.monospace("![alt](path) → Image");
                    ui.monospace("```lang  → Code block");
                });

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

// ============================================================================
// PDF-Style Renderer (for the PDF Viewer tab)
// ============================================================================

fn render_node_pdf_style(
    ui: &mut egui::Ui,
    node: &Node,
    styles: &crate::data::StyleSheet,
    zoom: f32,
    text_color: egui::Color32,
    _bg_color: egui::Color32,
    images: &HashMap<String, egui::TextureHandle>,
) {
    use crate::data::Direction;
    use crate::pdx_text::pdx_text;

    match node {
        Node::Document { children } => {
            for child in children {
                render_node_pdf_style(ui, child, styles, zoom, text_color, _bg_color, images);
            }
        }

        Node::Heading { level, runs, style } => {
            let style_def = styles.styles.get(style).cloned().unwrap_or_default();
            let size = (style_def.font_size * zoom).min(32.0);

            ui.add_space(style_def.margin.top);

            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);
            if is_rtl {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    for run in runs.iter().rev() {
                        let mut rich = RichText::new(&pdx_text(&run.text))
                            .size(size)
                            .color(text_color)
                            .strong();
                        if run.italic { rich = rich.italics(); }
                        ui.label(rich);
                    }
                });
            } else {
                ui.horizontal_wrapped(|ui| {
                    for run in runs {
                        let mut rich = RichText::new(&pdx_text(&run.text))
                            .size(size)
                            .color(text_color)
                            .strong();
                        if run.italic { rich = rich.italics(); }
                        if run.underline { rich = rich.underline(); }
                        ui.label(rich);
                    }
                });
            }

            ui.add_space(style_def.margin.bottom);
        }

        Node::Paragraph { runs, style } => {
            let style_def = styles.styles.get(style).cloned().unwrap_or_default();
            let size = (style_def.font_size * zoom).min(18.0);

            ui.add_space(style_def.margin.top);

            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);
            if is_rtl {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for run in runs.iter().rev() {
                            let mut rich = RichText::new(&pdx_text(&run.text))
                                .size(size)
                                .color(text_color);
                            if run.bold { rich = rich.strong(); }
                            if run.italic { rich = rich.italics(); }
                            if run.underline { rich = rich.underline(); }
                            ui.label(rich);
                        }
                    });
                });
            } else {
                ui.horizontal_wrapped(|ui| {
                    for run in runs {
                        let mut rich = RichText::new(&pdx_text(&run.text))
                            .size(size)
                            .color(text_color);
                        if run.bold { rich = rich.strong(); }
                        if run.italic { rich = rich.italics(); }
                        if run.underline { rich = rich.underline(); }
                        ui.label(rich);
                    }
                });
            }

            ui.add_space(style_def.margin.bottom);
        }

        Node::List { ordered, items, .. } => {
            for (i, item) in items.iter().enumerate() {
                let is_rtl = item.content.iter().any(|r| r.direction == Direction::RTL);
                if is_rtl {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for run in item.content.iter().rev() {
                                let mut rich = RichText::new(&pdx_text(&run.text))
                                    .size(14.0 * zoom)
                                    .color(text_color);
                                if run.bold { rich = rich.strong(); }
                                if run.italic { rich = rich.italics(); }
                                ui.label(rich);
                            }
                            let marker = if *ordered { format!(".{}", i + 1) } else { "•".to_string() };
                            ui.label(RichText::new(marker).size(14.0 * zoom).color(text_color));
                        });
                    });
                } else {
                    ui.horizontal_wrapped(|ui| {
                        let marker = if *ordered { format!("{}.", i + 1) } else { "•".to_string() };
                        ui.label(RichText::new(marker).size(14.0 * zoom).color(text_color));
                        for run in &item.content {
                            let mut rich = RichText::new(&pdx_text(&run.text))
                                .size(14.0 * zoom)
                                .color(text_color);
                            if run.bold { rich = rich.strong(); }
                            if run.italic { rich = rich.italics(); }
                            if run.underline { rich = rich.underline(); }
                            ui.label(rich);
                        }
                    });
                }
            }
            ui.add_space(8.0);
        }

        Node::CodeBlock { language, code, .. } => {
            ui.add_space(8.0);
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(245, 245, 248))
                .inner_margin(egui::Margin::same(10.0))
                .show(ui, |ui| {
                    ui.label(RichText::new(language).size(10.0).italics().color(egui::Color32::from_rgb(100, 100, 120)));
                    ui.label(RichText::new(code).size(11.0).code().color(egui::Color32::from_rgb(30, 30, 50)));
                });
            ui.add_space(8.0);
        }

        Node::Image { path, alt_text, width, height } => {
            ui.add_space(8.0);
            if let Some(texture) = images.get(path) {
                let size = if let (Some(w), Some(h)) = (width, height) {
                    egui::vec2(*w, *h)
                } else {
                    let s = texture.size_vec2();
                    egui::vec2(s.x.min(500.0), s.y.min(400.0))
                };
                ui.image((texture.id(), size));
            } else {
                ui.label(RichText::new(format!("🖼️ [Image: {}]", alt_text)).italics().color(egui::Color32::from_rgb(120, 120, 140)));
            }
            ui.add_space(8.0);
        }

        Node::Divider => {
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
        }

        Node::PageBreak => {
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                ui.separator();
                ui.label(RichText::new("  Page Break  ").size(10.0).color(egui::Color32::from_rgb(150, 150, 160)));
                ui.separator();
            });
            ui.add_space(16.0);
        }
    }
}