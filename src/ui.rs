use crate::data::PdxDocument;
use crate::export::{export_as_html, export_as_pdf, export_as_png};
use eframe::egui::{self, FontDefinitions, FontFamily};
use std::fs;
use std::path::PathBuf;

// ============================================================================
// UI Setup
// ============================================================================

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "arabic".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansArabic-Regular.ttf"
        ))),
    );

    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "arabic".to_owned());

    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "arabic".to_owned());

    ctx.set_fonts(fonts);

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            egui::TextStyle::Heading,
            egui::FontId::new(26.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(18.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::new(15.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Small,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();
    ctx.set_style(style);
}

// ============================================================================
// File Operations
// ============================================================================

pub fn open_document() -> Option<(PdxDocument, PathBuf)> {
    let path = rfd::FileDialog::new()
        .add_filter("PDX Document", &["pdx", "json"])
        .pick_file()?;

    let data = fs::read_to_string(&path).ok()?;
    let document: PdxDocument = serde_json::from_str(&data).ok()?;

    Some((document, path))
}

pub fn save_document(document: &PdxDocument, path: Option<&PathBuf>) -> Option<PathBuf> {
    let path = match path {
        Some(p) => p.clone(),
        None => rfd::FileDialog::new()
            .add_filter("PDX Document", &["pdx"])
            .set_file_name("document.pdx")
            .save_file()?,
    };

    let json = serde_json::to_string_pretty(document).unwrap();
    fs::write(&path, json).ok()?;

    Some(path)
}

pub fn export_html(document: &PdxDocument) -> Option<()> {
    let path = rfd::FileDialog::new()
        .add_filter("HTML", &["html"])
        .set_file_name(&format!("{}.html", document.metadata.title))
        .save_file()?;

    let html = export_as_html(document);
    fs::write(path, html).ok()?;

    Some(())
}

pub fn export_pdf_file(document: &PdxDocument) -> Option<()> {
    let path = rfd::FileDialog::new()
        .add_filter("PDF", &["pdf"])
        .set_file_name(&format!("{}.pdf", document.metadata.title))
        .save_file()?;

    match export_as_pdf(document) {
        Ok(pdf_data) => {
            fs::write(path, pdf_data).ok()?;
            Some(())
        }
        Err(_) => None,
    }
}

pub fn export_png_file() -> Option<()> {
    let path = rfd::FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .set_file_name("document.png")
        .save_file()?;

    match export_as_png(1200, 1600) {
        Ok(png_data) => {
            fs::write(path, png_data).ok()?;
            Some(())
        }
        Err(_) => None,
    }
}

pub fn insert_image() -> Option<String> {
    let path = rfd::FileDialog::new()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp"])
        .pick_file()?;

    Some(path.to_string_lossy().to_string())
}