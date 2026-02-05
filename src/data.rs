use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    pub fn to_egui(&self) -> eframe::egui::Color32 {
        eframe::egui::Color32::from_rgb(self.r, self.g, self.b)
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
    Image {
        path: String,
        alt_text: String,
        width: Option<f32>,
        height: Option<f32>,
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
pub struct Resources {
    pub images: HashMap<String, DynamicImage>,
}

// ============================================================================
// Sample Document
// ============================================================================

pub fn create_sample_document() -> PdxDocument {
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
                        "PDX is a modern document format with full Arabic support, real PDF/PNG export, and a comfortable theme for long writing sessions.",
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
                    runs: vec![TextRun::new(
                        "New Features - المميزات الجديدة",
                        "en",
                        "heading2",
                    )],
                    style: "heading2".to_string(),
                },
                Node::List {
                    ordered: false,
                    items: vec![
                        ListItem {
                            content: vec![TextRun::new(
                                "Real PDF export with Arabic font embedding",
                                "en",
                                "paragraph",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "PNG image export for sharing",
                                "en",
                                "paragraph",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "Image embedding support in documents",
                                "en",
                                "paragraph",
                            )],
                        },
                        ListItem {
                            content: vec![TextRun::new(
                                "Comfort theme - optimized for long writing sessions",
                                "en",
                                "paragraph",
                            )],
                        },
                    ],
                    style: "list".to_string(),
                },
            ],
        },
        resources: Resources::default(),
    }
}