use eframe::egui;
use egui::{FontDefinitions, FontFamily, RichText, ScrollArea};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::pdx_text::pdx_text;

mod pdx_text;

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
            language: "en".to_string(),
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

        styles.insert(
            "heading1".to_string(),
            Style {
                font_size: 28.0,
                font_weight: FontWeight::Bold,
                color: Color::rgb(0, 0, 0),
                text_align: TextAlign::Start,
                margin: EdgeInsets::new(12.0, 0.0, 16.0, 0.0),
                ..Default::default()
            },
        );

        styles.insert(
            "heading2".to_string(),
            Style {
                font_size: 22.0,
                font_weight: FontWeight::Bold,
                color: Color::rgb(40, 40, 40),
                text_align: TextAlign::Start,
                margin: EdgeInsets::new(10.0, 0.0, 12.0, 0.0),
                ..Default::default()
            },
        );

        styles.insert(
            "paragraph".to_string(),
            Style {
                font_size: 16.0,
                font_weight: FontWeight::Normal,
                color: Color::rgb(0, 0, 0),
                text_align: TextAlign::Start,
                line_height: 1.8,
                margin: EdgeInsets::new(0.0, 0.0, 10.0, 0.0),
                ..Default::default()
            },
        );

        styles.insert(
            "arabic".to_string(),
            Style {
                font_size: 18.0,
                font_weight: FontWeight::Normal,
                color: Color::rgb(0, 0, 0),
                text_align: TextAlign::Start,
                line_height: 2.0,
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
        let direction = if language == "ar" || language == "fa" || language == "ur" {
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
pub struct Resources {}

// ============================================================================
// Theme System
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum AppTheme {
    Dark,
    Light,
    Sepia,
    Midnight,
}

impl AppTheme {
    fn text_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(20, 20, 20),
            AppTheme::Dark => egui::Color32::from_rgb(230, 230, 230),
            AppTheme::Sepia => egui::Color32::from_rgb(60, 50, 40),
            AppTheme::Midnight => egui::Color32::from_rgb(200, 210, 230),
        }
    }

    fn background_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(250, 250, 250),
            AppTheme::Dark => egui::Color32::from_rgb(30, 30, 35),
            AppTheme::Sepia => egui::Color32::from_rgb(245, 235, 215),
            AppTheme::Midnight => egui::Color32::from_rgb(15, 20, 35),
        }
    }

    fn panel_color(&self) -> egui::Color32 {
        match self {
            AppTheme::Light => egui::Color32::from_rgb(255, 255, 255),
            AppTheme::Dark => egui::Color32::from_rgb(40, 40, 45),
            AppTheme::Sepia => egui::Color32::from_rgb(255, 248, 235),
            AppTheme::Midnight => egui::Color32::from_rgb(25, 30, 50),
        }
    }

    fn apply(&self, ctx: &egui::Context) {
        let mut visuals = match self {
            AppTheme::Light => egui::Visuals::light(),
            AppTheme::Dark => egui::Visuals::dark(),
            AppTheme::Sepia => egui::Visuals::light(),
            AppTheme::Midnight => egui::Visuals::dark(),
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

        ctx.set_visuals(visuals);
    }

    fn name(&self) -> &str {
        match self {
            AppTheme::Dark => "Dark",
            AppTheme::Light => "Light",
            AppTheme::Sepia => "Sepia",
            AppTheme::Midnight => "Midnight",
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
    raw_content: String,
    show_stats: bool,
    zoom_level: f32,
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

            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);

            if is_rtl {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for run in runs.iter().rev() {
                            ui.label(
                                RichText::new(&pdx_text(&run.text))
                                    .size(size)
                                    .color(text_color)
                                    .strong(),
                            );
                        }
                    });
                });
            } else {
                ui.horizontal_wrapped(|ui| {
                    for run in runs {
                        ui.label(
                            RichText::new(&pdx_text(&run.text))
                                .size(size)
                                .color(text_color)
                                .strong(),
                        );
                    }
                });
            }

            ui.add_space(style_def.margin.bottom * zoom);
        }

        Node::Paragraph { runs, style } => {
            let style_def = styles.styles.get(style).cloned().unwrap_or_default();
            let size = style_def.font_size * zoom;

            ui.add_space(style_def.margin.top * zoom);

            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);

            if is_rtl {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for run in runs.iter().rev() {
                            ui.label(RichText::new(&pdx_text(&run.text)).size(size).color(text_color));
                        }
                    });
                });
            } else {
                ui.horizontal_wrapped(|ui| {
                    for run in runs {
                        ui.label(RichText::new(&pdx_text(&run.text)).size(size).color(text_color));
                    }
                });
            }

            ui.add_space(style_def.margin.bottom * zoom);
        }

        Node::List { ordered, items, .. } => {
            for (i, item) in items.iter().enumerate() {
                let is_rtl = item.content.iter().any(|r| r.direction == Direction::RTL);

                if is_rtl {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.horizontal_wrapped(|ui| {
                            for run in item.content.iter().rev() {
                                ui.label(
                                    RichText::new(&pdx_text(&run.text))
                                        .size(16.0 * zoom)
                                        .color(text_color),
                                );
                            }

                            let marker = if *ordered {
                                format!(".{}", i + 1)
                            } else {
                                "•".to_string()
                            };
                            ui.label(RichText::new(marker).size(16.0 * zoom).color(text_color));
                        });
                    });
                } else {
                    ui.horizontal_wrapped(|ui| {
                        let marker = if *ordered {
                            format!("{}.", i + 1)
                        } else {
                            "•".to_string()
                        };

                        ui.label(RichText::new(marker).size(16.0 * zoom).color(text_color));

                        for run in &item.content {
                            ui.label(
                                RichText::new(&pdx_text(&run.text))
                                    .size(16.0 * zoom)
                                    .color(text_color),
                            );
                        }
                    });
                }
            }
            ui.add_space(10.0 * zoom);
        }

        Node::CodeBlock { language, code, .. } => {
            ui.add_space(10.0);
            ui.group(|ui| {
                ui.label(
                    RichText::new(language)
                        .size(11.0 * zoom)
                        .italics()
                        .color(text_color),
                );
                ui.label(
                    RichText::new(code)
                        .size(13.0 * zoom)
                        .code()
                        .color(text_color),
                );
            });
            ui.add_space(10.0);
        }

        Node::Divider => {
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
        }

        Node::PageBreak => {
            ui.add_space(20.0);
            ui.separator();
            ui.label(RichText::new("— Page Break —").italics().weak());
            ui.separator();
            ui.add_space(20.0);
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
                .join(" ");
            format!("{} {}", prefix, text)
        }

        Node::Paragraph { runs, .. } => runs
            .iter()
            .map(|r| r.text.clone())
            .collect::<Vec<_>>()
            .join(" "),

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
                    .join(" ");
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

        if line.starts_with('#') {
            let level = line.chars().take_while(|&c| c == '#').count() as u8;
            let text = line.trim_start_matches('#').trim();
            let is_arabic = text.chars().any(|c| c >= '\u{0600}' && c <= '\u{06FF}');

            children.push(Node::Heading {
                level,
                runs: vec![TextRun::new(
                    text,
                    if is_arabic { "ar" } else { "en" },
                    &format!("heading{}", level),
                )],
                style: format!("heading{}", level),
            });
        } else if line.starts_with("```") {
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
        } else if line.starts_with('-') || line.starts_with("•") {
            let mut items = Vec::new();

            while i < lines.len() {
                let line = lines[i].trim();
                if line.starts_with('-') || line.starts_with("•") {
                    let text = line.trim_start_matches('-').trim_start_matches("•").trim();
                    let is_arabic = text.chars().any(|c| c >= '\u{0600}' && c <= '\u{06FF}');

                    items.push(ListItem {
                        content: vec![TextRun::new(
                            text,
                            if is_arabic { "ar" } else { "en" },
                            "paragraph",
                        )],
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
        } else if line == "---" {
            children.push(Node::Divider);
        } else if line == "===" {
            children.push(Node::PageBreak);
        } else {
            let is_arabic = line.chars().any(|c| c >= '\u{0600}' && c <= '\u{06FF}');

            children.push(Node::Paragraph {
                runs: vec![TextRun::new(
                    line,
                    if is_arabic { "ar" } else { "en" },
                    if is_arabic { "arabic" } else { "paragraph" },
                )],
                style: if is_arabic { "arabic" } else { "paragraph" }.to_string(),
            });
        }

        i += 1;
    }

    Node::Document { children }
}

// ============================================================================
// Export Functions
// ============================================================================

fn export_as_html(document: &PdxDocument) -> String {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html dir="auto">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>"#,
    );
    html.push_str(&document.metadata.title);
    html.push_str(
        r#"</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif, 'Noto Sans Arabic';
            max-width: 800px;
            margin: 40px auto;
            padding: 20px;
            line-height: 1.8;
            direction: auto;
        }
        .rtl { direction: rtl; text-align: right; }
        .ltr { direction: ltr; text-align: left; }
        h1 { font-size: 28px; margin: 12px 0 16px; }
        h2 { font-size: 22px; margin: 10px 0 12px; }
        p { margin: 10px 0; font-size: 16px; }
        code { background: #f4f4f4; padding: 2px 6px; border-radius: 3px; }
        pre { background: #f4f4f4; padding: 15px; border-radius: 5px; overflow-x: auto; }
        hr { margin: 20px 0; border: none; border-top: 1px solid #ddd; }
    </style>
</head>
<body>
"#,
    );

    fn node_to_html(node: &Node) -> String {
        match node {
            Node::Document { children } => children.iter().map(node_to_html).collect(),
            Node::Heading { level, runs, .. } => {
                let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);
                let dir_class = if is_rtl { "rtl" } else { "ltr" };
                let text: String = runs.iter().map(|r| r.text.clone()).collect();
                format!("<h{} class=\"{}\">{}</h{}>\n", level, dir_class, text, level)
            }
            Node::Paragraph { runs, .. } => {
                let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);
                let dir_class = if is_rtl { "rtl" } else { "ltr" };
                let text: String = runs.iter().map(|r| r.text.clone()).collect();
                format!("<p class=\"{}\">{}</p>\n", dir_class, text)
            }
            Node::List { ordered, items, .. } => {
                let tag = if *ordered { "ol" } else { "ul" };
                let items_html: String = items
                    .iter()
                    .map(|item| {
                        let is_rtl = item.content.iter().any(|r| r.direction == Direction::RTL);
                        let dir_class = if is_rtl { "rtl" } else { "ltr" };
                        let text: String = item.content.iter().map(|r| r.text.clone()).collect();
                        format!("<li class=\"{}\">{}</li>", dir_class, text)
                    })
                    .collect();
                format!("<{0}>{1}</{0}>\n", tag, items_html)
            }
            Node::CodeBlock { language, code, .. } => {
                format!("<pre><code class=\"language-{}\">{}</code></pre>\n", language, code)
            }
            Node::Divider => "<hr/>\n".to_string(),
            Node::PageBreak => "<hr style=\"border-top: 3px double #ddd;\"/>\n".to_string(),
        }
    }

    html.push_str(&node_to_html(&document.content));
    html.push_str("</body>\n</html>");
    html
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
            keywords: vec!["pdx".to_string(), "document".to_string(), "مستند".to_string()],
        },
        styles: StyleSheet::default(),
        content: Node::Document {
            children: vec![
                Node::Heading {
                    level: 1,
                    runs: vec![TextRun::new("Welcome to PDX Editor", "en", "heading1")],
                    style: "heading1".to_string(),
                },
                Node::Paragraph {
                    runs: vec![TextRun::new(
                        "PDX is a modern document format that combines the power of PDF with the ease of editing text documents. It features full Arabic and RTL language support.",
                        "en",
                        "paragraph",
                    )],
                    style: "paragraph".to_string(),
                },
                Node::Divider,
                Node::Heading {
                    level: 2,
                    runs: vec![TextRun::new("مرحباً بك في محرر PDX", "ar", "heading2")],
                    style: "heading2".to_string(),
                },
                Node::Paragraph {
                    runs: vec![TextRun::new(
                        "هذا المحرر يدعم اللغة العربية بشكل كامل مع الكتابة من اليمين إلى اليسار. يمكنك كتابة المستندات بالعربية بسهولة تامة.",
                        "ar",
                        "arabic",
                    )],
                    style: "arabic".to_string(),
                },
                Node::Divider,
                Node::Heading {
                    level: 2,
                    runs: vec![TextRun::new("Key Features", "en", "heading2")],
                    style: "heading2".to_string(),
                },
                Node::List {
                    ordered: false,
                    items: vec![
                        ListItem {
                            content: vec![TextRun::new(
                                "Full support for Arabic and RTL text",
                                "en",
                                "paragraph",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "Four beautiful themes: Light, Dark, Midnight, and Sepia",
                                "en",
                                "paragraph",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "Export to HTML, PDF, and images",
                                "en",
                                "paragraph",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "Real-time preview with split mode",
                                "en",
                                "paragraph",
                            )],
                        },
                    ],
                    style: "list".to_string(),
                },
                Node::Divider,
                Node::Heading {
                    level: 2,
                    runs: vec![TextRun::new("المميزات الرئيسية", "ar", "heading2")],
                    style: "heading2".to_string(),
                },
                Node::List {
                    ordered: false,
                    items: vec![
                        ListItem {
                            content: vec![TextRun::new(
                                "دعم كامل للغة العربية والكتابة من اليمين لليسار",
                                "ar",
                                "arabic",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "أربعة ثيمات جميلة مع ثيم منتصف الليل الجديد",
                                "ar",
                                "arabic",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "تصدير لصيغ متعددة: HTML، PDF، صور",
                                "ar",
                                "arabic",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "معاينة مباشرة مع الوضع المقسم",
                                "ar",
                                "arabic",
                            )],
                        },
                    ],
                    style: "list".to_string(),
                },
                Node::Divider,
                Node::Heading {
                    level: 2,
                    runs: vec![TextRun::new("Code Example - مثال الكود", "en", "heading2")],
                    style: "heading2".to_string(),
                },
                Node::CodeBlock {
                    language: "rust".to_string(),
                    code: r#"fn main() {
    // English
    println!("Hello, World!");
    
    // العربية  
    println!("مرحباً بالعالم!");
}"#
                    .to_string(),
                    style: "code".to_string(),
                },
                Node::Paragraph {
                    runs: vec![TextRun::new(
                        "You can edit this document using the editor, and the preview will update automatically.",
                        "en",
                        "paragraph",
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

fn export_html(document: &PdxDocument) -> Option<()> {
    let path = rfd::FileDialog::new()
        .add_filter("HTML", &["html"])
        .set_file_name(&format!("{}.html", document.metadata.title))
        .save_file()?;

    let html = export_as_html(document);
    fs::write(path, html).ok()?;

    Some(())
}

// ============================================================================
// UI Setup
// ============================================================================

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "arabic".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoSansArabic-Regular.ttf"
        ))),
    );

    fonts.font_data.insert(
        "default".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/fonts/NotoColorEmoji.ttf"
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
// Main Application
// ============================================================================

impl eframe::App for PdxApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme.apply(ctx);

        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.heading("📄 PDX Editor");
                ui.separator();

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

                    ui.separator();

                    ui.menu_button("📤 Export as...", |ui| {
                        if ui.button("🌐 HTML").clicked() {
                            if export_html(&self.document).is_some() {
                                self.status_message = "Exported as HTML".to_string();
                            }
                            ui.close_menu();
                        }

                        if ui.button("📄 PDF").clicked() {
                            self.status_message =
                                "PDF export - Use Print (Ctrl+P) to save as PDF".to_string();
                            ui.close_menu();
                        }

                        if ui.button("🖼️ Image (PNG)").clicked() {
                            self.status_message = "Image export - Coming soon".to_string();
                            ui.close_menu();
                        }
                    });

                    ui.separator();

                    if ui.button("🖨️ Print").clicked() {
                        self.status_message =
                            "Use Ctrl+P or browser menu to print".to_string();
                        ui.close_menu();
                    }
                });

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
                });

                ui.separator();

                ui.selectable_value(&mut self.active_tab, EditorTab::Editor, "✏️ Editor");
                ui.selectable_value(
                    &mut self.active_tab,
                    EditorTab::Metadata,
                    "ℹ️ Metadata",
                );
                ui.selectable_value(&mut self.active_tab, EditorTab::Styles, "🎨 Styles");
            });
        });

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
                            );
                        });
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

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 800.0])
            .with_title("PDX Editor - Modern Document Format with Arabic Support"),
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