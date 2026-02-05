use crate::data::{Direction, Node, StyleSheet};
use crate::pdx_text::pdx_text;
use crate::theme::AppTheme;
use eframe::egui::{self, RichText};
use std::collections::HashMap;

// ============================================================================
// Document Rendering
// ============================================================================

pub fn render_node(
    ui: &mut egui::Ui,
    node: &Node,
    styles: &StyleSheet,
    zoom: f32,
    theme: &AppTheme,
    images: &HashMap<String, egui::TextureHandle>,
) {
    let text_color = theme.text_color();

    match node {
        Node::Document { children } => {
            for child in children {
                render_node(ui, child, styles, zoom, theme, images);
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
                            ui.label(
                                RichText::new(&pdx_text(&run.text))
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
                            RichText::new(&pdx_text(&run.text))
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

        Node::Image {
            path,
            alt_text,
            width,
            height,
        } => {
            ui.add_space(10.0);
            if let Some(texture) = images.get(path) {
                let size = if let (Some(w), Some(h)) = (width, height) {
                    egui::vec2(*w * zoom, *h * zoom)
                } else {
                    let size = texture.size_vec2();
                    egui::vec2(size.x * zoom, size.y * zoom)
                };
                ui.image((texture.id(), size));
            } else {
                ui.label(RichText::new(format!("🖼️ [Image: {}]", alt_text)).italics());
            }
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