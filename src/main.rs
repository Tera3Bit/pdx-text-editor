use eframe::egui;
use egui::{FontDefinitions, FontFamily, RichText, ScrollArea};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// ============================================================================
// Core Data Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdxDocument {
    pub version: u32,
    pub metadata: Metadata,
    pub styles: StyleSheet,
    pub content: Node,
    #[serde(skip)]
    pub resources: Resources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub title: String,
    pub author: String,
    pub language: String,
    pub created: String,
    pub modified: String,
    pub keywords: Vec<String>,
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: "Untitled Document".to_string(),
            author: String::new(),
            language: "ar".to_string(),
            created: chrono::Local::now().to_string(),
            modified: chrono::Local::now().to_string(),
            keywords: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleSheet {
    pub styles: HashMap<String, Style>,
    pub active_theme: String,
}

impl Default for StyleSheet {
    fn default() -> Self {
        let mut styles = HashMap::new();

        // Default styles
        styles.insert(
            "heading1".to_string(),
            Style {
                font_size: 24.0,
                font_weight: FontWeight::Bold,
                color: Color::rgb(0, 0, 0),
                text_align: TextAlign::Start,
                margin: EdgeInsets::new(0.0, 0.0, 16.0, 0.0),
                ..Default::default()
            },
        );

        styles.insert(
            "heading2".to_string(),
            Style {
                font_size: 20.0,
                font_weight: FontWeight::Bold,
                color: Color::rgb(40, 40, 40),
                text_align: TextAlign::Start,
                margin: EdgeInsets::new(0.0, 0.0, 12.0, 0.0),
                ..Default::default()
            },
        );

        styles.insert(
            "paragraph".to_string(),
            Style {
                font_size: 14.0,
                font_weight: FontWeight::Normal,
                color: Color::rgb(0, 0, 0),
                text_align: TextAlign::Start,
                line_height: 1.5,
                margin: EdgeInsets::new(0.0, 0.0, 8.0, 0.0),
                ..Default::default()
            },
        );

        styles.insert(
            "arabic".to_string(),
            Style {
                font_size: 16.0,
                font_weight: FontWeight::Normal,
                color: Color::rgb(0, 0, 0),
                text_align: TextAlign::Start,
                line_height: 1.8,
                direction: Direction::RTL,
                ..Default::default()
            },
        );

        Self {
            styles,
            active_theme: "default".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Style {
    #[serde(default)]
    pub font_size: f32,
    #[serde(default)]
    pub font_weight: FontWeight,
    #[serde(default)]
    pub color: Color,
    #[serde(default)]
    pub text_align: TextAlign,
    #[serde(default)]
    pub direction: Direction,
    #[serde(default)]
    pub line_height: f32,
    #[serde(default)]
    pub margin: EdgeInsets,
    #[serde(default)]
    pub padding: EdgeInsets,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FontWeight {
    Normal,
    Bold,
    Light,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TextAlign {
    Start,
    End,
    Center,
    Justify,
}

impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Start
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    LTR,
    RTL,
    Auto,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Auto
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_egui(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct EdgeInsets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl EdgeInsets {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    pub fn all(value: f32) -> Self {
        Self::new(value, value, value, value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Document {
        children: Vec<Node>,
    },
    Heading {
        level: u8,
        runs: Vec<TextRun>,
        style: String,
    },
    Paragraph {
        runs: Vec<TextRun>,
        style: String,
    },
    List {
        ordered: bool,
        items: Vec<ListItem>,
        style: String,
    },
    CodeBlock {
        language: String,
        code: String,
        style: String,
    },
    Divider,
    PageBreak,
}

impl Default for Node {
    fn default() -> Self {
        Node::Document {
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    pub text: String,
    pub language: String,
    pub direction: Direction,
    pub style: String,
}

impl TextRun {
    pub fn new(text: &str, language: &str, style: &str) -> Self {
        let direction = if language == "ar" {
            Direction::RTL
        } else {
            Direction::LTR
        };

        Self {
            text: text.to_string(),
            language: language.to_string(),
            direction,
            style: style.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    pub content: Vec<TextRun>,
}

#[derive(Debug, Clone, Default)]
pub struct Resources {
    // Font references, images, etc.
}

// ============================================================================
// Theme System
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppTheme {
    Light,
    Dark,
    Sepia,
}

impl AppTheme {
    fn text_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(20, 20, 20),
            AppTheme::Dark => egui::Color32::from_rgb(230, 230, 230),
            AppTheme::Sepia => egui::Color32::from_rgb(60, 50, 40),
        }
    }

    fn background_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(250, 250, 250),
            AppTheme::Dark => egui::Color32::from_rgb(30, 30, 35),
            AppTheme::Sepia => egui::Color32::from_rgb(245, 235, 215),
        }
    }

    fn panel_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(255, 255, 255),
            AppTheme::Dark => egui::Color32::from_rgb(40, 40, 45),
            AppTheme::Sepia => egui::Color32::from_rgb(255, 248, 235),
        }
    }

    fn apply(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::default();
        
        match self {
            AppTheme::Light => {
                visuals = egui::Visuals::light();
                visuals.override_text_color = Some(self.text_color());
            }
            AppTheme::Dark => {
                visuals = egui::Visuals::dark();
                visuals.override_text_color = Some(self.text_color());
            }
            AppTheme::Sepia => {
                visuals = egui::Visuals::light();
                visuals.override_text_color = Some(self.text_color());
                visuals.panel_fill = self.panel_color();
                visuals.window_fill = self.panel_color();
                visuals.extreme_bg_color = self.background_color();
            }
        }

        ctx.set_visuals(visuals);
    }

    fn name(&self) -> &str {
        match self {
            AppTheme::Light => "Light",
            AppTheme::Dark => "Dark",
            AppTheme::Sepia => "Sepia",
        }
    }
}

// ============================================================================
// Editor State
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum EditorMode {
    Edit,
    Preview,
    Split,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EditorTab {
    Editor,
    Metadata,
    Styles,
}

struct PdxApp {
    document: PdxDocument,
    path: Option<PathBuf>,
    mode: EditorMode,
    active_tab: EditorTab,
    theme: AppTheme,

    // Editor state
    raw_content: String,
    show_stats: bool,
    zoom_level: f32,

    // Status
    last_save: Option<String>,
    status_message: String,
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
            theme: AppTheme::Light,
            raw_content,
            show_stats: false,
            zoom_level: 1.0,
            last_save: None,
            status_message: "Ready".to_string(),
        }
    }
}

// ============================================================================
// Document Rendering
// ============================================================================

fn render_node(ui: &mut egui::Ui, node: &Node, styles: &StyleSheet, zoom: f32, theme: &AppTheme) {
    let text_color = theme.text_color();

    match node {
        Node::Document { children } => {
            for child in children {
                render_node(ui, child, styles, zoom, theme);
            }
        }

        Node::Heading { level, runs, style } => {
            let style_def = styles.styles.get(style).cloned().unwrap_or_default();
            let size = style_def.font_size * zoom;

            ui.add_space(style_def.margin.top * zoom);

            // Check if any run is RTL
            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);

            if is_rtl {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    for run in runs {
                        ui.label(
                            RichText::new(&run.text)
                                .size(size)
                                .color(text_color)
                                .strong(),
                        );
                    }
                });
            } else {
                for run in runs {
                    ui.label(
                        RichText::new(&run.text)
                            .size(size)
                            .color(text_color)
                            .strong(),
                    );
                }
            }

            ui.add_space(style_def.margin.bottom * zoom);
        }

        Node::Paragraph { runs, style } => {
            let style_def = styles.styles.get(style).cloned().unwrap_or_default();
            let size = style_def.font_size * zoom;

            ui.add_space(style_def.margin.top * zoom);

            // Check if any run is RTL
            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);

            if is_rtl {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for run in runs.iter().rev() {
                            ui.label(
                                RichText::new(&run.text)
                                    .size(size)
                                    .color(text_color),
                            );
                        }
                    });
                });
            } else {
                ui.horizontal_wrapped(|ui| {
                    for run in runs {
                        ui.label(
                            RichText::new(&run.text)
                                .size(size)
                                .color(text_color),
                        );
                    }
                });
            }

            ui.add_space(style_def.margin.bottom * zoom);
        }

        Node::List { ordered, items, .. } => {
            for (i, item) in items.iter().enumerate() {
                // Check if any run is RTL
                let is_rtl = item.content.iter().any(|r| r.direction == Direction::RTL);

                if is_rtl {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.horizontal(|ui| {
                            for run in item.content.iter().rev() {
                                ui.label(RichText::new(&run.text).size(14.0 * zoom).color(text_color));
                            }
                            
                            let marker = if *ordered {
                                format!(".{}", i + 1)
                            } else {
                                "•".to_string()
                            };
                            ui.label(RichText::new(marker).size(14.0 * zoom).color(text_color));
                        });
                    });
                } else {
                    ui.horizontal(|ui| {
                        let marker = if *ordered {
                            format!("{}.", i + 1)
                        } else {
                            "•".to_string()
                        };

                        ui.label(RichText::new(marker).size(14.0 * zoom).color(text_color));

                        for run in &item.content {
                            ui.label(RichText::new(&run.text).size(14.0 * zoom).color(text_color));
                        }
                    });
                }
            }
            ui.add_space(8.0 * zoom);
        }

        Node::CodeBlock { language, code, .. } => {
            ui.add_space(8.0);
            ui.group(|ui| {
                ui.label(RichText::new(language).size(10.0 * zoom).italics().color(text_color));
                ui.label(RichText::new(code).size(12.0 * zoom).code().color(text_color));
            });
            ui.add_space(8.0);
        }

        Node::Divider => {
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
        }

        Node::PageBreak => {
            ui.add_space(16.0);
            ui.separator();
            ui.label(RichText::new("— Page Break —").italics().weak());
            ui.separator();
            ui.add_space(16.0);
        }
    }
}

// ============================================================================
// Content Serialization
// ============================================================================

fn serialize_content(node: &Node) -> String {
    match node {
        Node::Document { children } => children
            .iter()
            .map(|c| serialize_content(c))
            .collect::<Vec<_>>()
            .join("\n\n"),

        Node::Heading { level, runs, .. } => {
            let prefix = "#".repeat(*level as usize);
            let text = runs
                .iter()
                .map(|r| r.text.clone())
                .collect::<Vec<_>>()
                .join("");
            format!("{} {}", prefix, text)
        }

        Node::Paragraph { runs, .. } => runs
            .iter()
            .map(|r| r.text.clone())
            .collect::<Vec<_>>()
            .join(""),

        Node::List { ordered, items, .. } => items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let marker = if *ordered {
                    format!("{}.", i + 1)
                } else {
                    "-".to_string()
                };
                let text = item
                    .content
                    .iter()
                    .map(|r| r.text.clone())
                    .collect::<Vec<_>>()
                    .join("");
                format!("{} {}", marker, text)
            })
            .collect::<Vec<_>>()
            .join("\n"),

        Node::CodeBlock { language, code, .. } => {
            format!("```{}\n{}\n```", language, code)
        }

        Node::Divider => "---".to_string(),

        Node::PageBreak => "===".to_string(),
    }
}

fn parse_content(text: &str) -> Node {
    let mut children = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Heading
        if line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count() as u8;
            let text = line.trim_start_matches('#').trim();

            children.push(Node::Heading {
                level,
                runs: vec![TextRun::new(text, "ar", "heading1")],
                style: format!("heading{}", level),
            });
        }
        // Code block
        else if line.starts_with("```") {
            let language = line.trim_start_matches('`').trim().to_string();
            let mut code_lines = Vec::new();
            i += 1;

            while i < lines.len() && !lines[i].trim().starts_with("```") {
                code_lines.push(lines[i]);
                i += 1;
            }

            children.push(Node::CodeBlock {
                language: if language.is_empty() {
                    "text".to_string()
                } else {
                    language
                },
                code: code_lines.join("\n"),
                style: "code".to_string(),
            });
        }
        // List item
        else if line.starts_with('-') || line.starts_with("•") {
            let mut items = Vec::new();

            while i < lines.len() {
                let line = lines[i].trim();
                if line.starts_with('-') || line.starts_with("•") {
                    let text = line.trim_start_matches('-').trim_start_matches("•").trim();
                    items.push(ListItem {
                        content: vec![TextRun::new(text, "ar", "paragraph")],
                    });
                    i += 1;
                } else {
                    break;
                }
            }

            children.push(Node::List {
                ordered: false,
                items,
                style: "list".to_string(),
            });
            i -= 1;
        }
        // Divider
        else if line == "---" {
            children.push(Node::Divider);
        }
        // Page break
        else if line == "===" {
            children.push(Node::PageBreak);
        }
        // Paragraph
        else {
            children.push(Node::Paragraph {
                runs: vec![TextRun::new(line, "ar", "paragraph")],
                style: "paragraph".to_string(),
            });
        }

        i += 1;
    }

    Node::Document { children }
}

// ============================================================================
// Sample Document
// ============================================================================

fn create_sample_document() -> PdxDocument {
    PdxDocument {
        version: 1,
        metadata: Metadata {
            title: "PDX Demo Document".to_string(),
            author: "PDX Editor".to_string(),
            language: "en".to_string(),
            created: chrono::Local::now().to_string(),
            modified: chrono::Local::now().to_string(),
            keywords: vec!["pdx".to_string(), "مستند".to_string()],        
        },
        styles: StyleSheet::default(),
        content: Node::Document {
            children: vec![
                Node::Heading {
                    level: 1,
                    runs: vec![TextRun::new("Welcome to PDX", "en", "heading1")],
                    style: "heading1".to_string(),
                },
                Node::Paragraph {
                    runs: vec![TextRun::new(
                        "PDX is a modern document standard that combines the power of PDF with the ease of editing text documents.",
                        "en",
                        "paragraph"
                    )],
                    style: "paragraph".to_string(),
                },
                Node::Divider,
                Node::Heading {
                    level: 2,
                    runs: vec![TextRun::new("مرحباً بك في PDX", "ar", "heading2")],
                    style: "heading2".to_string(),
                },
                Node::Paragraph {
                    runs: vec![TextRun::new(
                        "هذا المحرر يدعم اللغة العربية بشكل كامل مع دعم الكتابة من اليمين إلى اليسار.",
                        "ar",
                        "arabic"
                    )],
                    style: "arabic".to_string(),
                },
                Node::Divider,
                Node::Heading {
                    level: 2,
                    runs: vec![TextRun::new("Basic Features", "en", "heading2")],
                    style: "heading2".to_string(),
                },
                Node::List {
                    ordered: false,
                    items: vec![
                        ListItem {
                            content: vec![TextRun::new("Full support for Arabic and RTL text", "en", "paragraph")],
                        },
                        ListItem { 
                            content: vec![TextRun::new("Editable semantic structure", "en", "paragraph")], 
                        }, 
                        ListItem { 
                            content: vec![TextRun::new("Advanced style system", "en", "paragraph")], 
                        }, 
                        ListItem { 
                            content: vec![TextRun::new("Small file size and high performance", "en", "paragraph")], 
                        },
                        ListItem {
                            content: vec![TextRun::new("دعم كامل للغة العربية", "ar", "arabic")],
                        },
                    ],
                    style: "list".to_string(),
                },
                Node::Divider,
                Node::Heading {
                    level: 2,
                    runs: vec![TextRun::new("Code Example", "en", "heading2")],
                    style: "heading2".to_string(),
                },
                Node::CodeBlock {
                    language: "rust".to_string(),
                    code: r#"fn main() {
    println!("Hello, PDX!");
    println!("مرحباً!");
}"#.to_string(),
                    style: "code".to_string(),
                },
                Node::Paragraph {
                    runs: vec![TextRun::new(
                        "You can edit this document using the editor, and the preview will update automatically.",
                        "en",
                        "paragraph"
                    )],
                    style: "paragraph".to_string(),
                },
            ],
        },
        resources: Resources::default(),
    }
}

// ============================================================================
// File Operations
// ============================================================================

fn open_document() -> Option<(PdxDocument, PathBuf)> {
    let path = rfd::FileDialog::new()
        .add_filter("PDX Document", &["pdx", "json"])
        .pick_file()?;

    let data = fs::read_to_string(&path).ok()?;
    let document: PdxDocument = serde_json::from_str(&data).ok()?;

    Some((document, path))
}

fn save_document(document: &PdxDocument, path: Option<&PathBuf>) -> Option<PathBuf> {
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

// ============================================================================
// UI Setup
// ============================================================================

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Add Arabic font support
    fonts.font_data.insert(
        "arabic".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(
            include_bytes!("../assets/fonts/NotoSansArabic-Regular.ttf")
        )),
    );

    // Add Arabic font to proportional family (for general text)
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "arabic".to_owned());

    // Add Arabic font to monospace family (for code)
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .insert(0, "arabic".to_owned());

    ctx.set_fonts(fonts);

    // Adjust text sizes
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (
            egui::TextStyle::Heading,
            egui::FontId::new(24.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Body,
            egui::FontId::new(16.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Monospace,
            egui::FontId::new(14.0, egui::FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Button,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        ),
        (
            egui::TextStyle::Small,
            egui::FontId::new(12.0, egui::FontFamily::Proportional),
        ),
    ]
    .into();
    ctx.set_style(style);
}

// ============================================================================
// Main Application
// ============================================================================

impl eframe::App for PdxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.theme.apply(ctx);

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("📄 PDX Editor");
                ui.separator();

                // File menu
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
                            self.last_save =
                                Some(chrono::Local::now().format("%H:%M:%S").to_string());
                            self.status_message = format!("Saved: {}", path.display());
                        }
                        ui.close_menu();
                    }

                    if ui.button("💾 Save As...").clicked() {
                        if let Some(path) = save_document(&self.document, None) {
                            self.path = Some(path.clone());
                            self.last_save =
                                Some(chrono::Local::now().format("%H:%M:%S").to_string());
                            self.status_message = format!("Saved as: {}", path.display());
                        }
                        ui.close_menu();
                    }
                });

                // View menu
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
                        self.zoom_level = (self.zoom_level + 0.1).min(2.0);
                    }
                    if ui.button("🔍- Zoom Out").clicked() {
                        self.zoom_level = (self.zoom_level - 0.1).max(0.5);
                    }
                    if ui.button("🔍 Reset").clicked() {
                        self.zoom_level = 1.0;
                    }
                });

                // Theme menu
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
                    if ui.selectable_label(self.theme == AppTheme::Sepia, "📜 Sepia").clicked() {
                        self.theme = AppTheme::Sepia;
                        self.status_message = "Theme changed to Sepia".to_string();
                        ui.close_menu();
                    }
                });

                ui.separator();

                // Tabs
                ui.selectable_value(&mut self.active_tab, EditorTab::Editor, "✏️ Editor");
                ui.selectable_value(&mut self.active_tab, EditorTab::Metadata, "ℹ️ Metadata");
                ui.selectable_value(&mut self.active_tab, EditorTab::Styles, "🎨 Styles");
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| match self.active_tab {
            EditorTab::Editor => {
                self.render_editor_tab(ui);
            }
            EditorTab::Metadata => {
                self.render_metadata_tab(ui);
            }
            EditorTab::Styles => {
                self.render_styles_tab(ui);
            }
        });

        // Status bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
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
        });
    }
}

impl PdxApp {
    fn render_editor_tab(&mut self, ui: &mut egui::Ui) {
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
                        );
                    });
            }

            EditorMode::Split => {
                ui.columns(2, |cols| {
                    // Editor
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

                    // Preview
                    ScrollArea::vertical().id_salt("split_preview_scroll").show(
                        &mut cols[1],
                        |ui| {
                            ui.heading("Preview");
                            ui.separator();
                            render_node(
                                ui,
                                &self.document.content,
                                &self.document.styles,
                                self.zoom_level,
                                &self.theme,
                            );
                        },
                    );
                });
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

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 800.0])
            .with_title("PDX Editor - Next Generation Document Format"),
        ..Default::default()
    };

    eframe::run_native(
        "PDX Editor",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::new(PdxApp::default()))
        }),
    )
}