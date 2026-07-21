use crate::data::{Direction, Node, PdxDocument};
use crate::pdx_text::pdx_text;
use ::image::ImageFormat;
use ::image::{ImageBuffer, Rgba};
use printpdf::*;
use std::io::BufWriter;

// ============================================================================
// Export Functions
// ============================================================================

pub fn export_as_html(document: &PdxDocument) -> String {
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
        img { max-width: 100%; height: auto; margin: 10px 0; }
        .bold { font-weight: bold; }
        .italic { font-style: italic; }
        .underline { text-decoration: underline; }
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
                let text: String = runs.iter().map(|r| {
                    let mut s = r.text.clone();
                    if r.bold { s = format!("<strong>{}</strong>", s); }
                    if r.italic { s = format!("<em>{}</em>", s); }
                    if r.underline { s = format!("<u>{}</u>", s); }
                    s
                }).collect();
                format!(
                    "<h{} class=\"{}\">{}</h{}>\n",
                    level, dir_class, text, level
                )
            }
            Node::Paragraph { runs, .. } => {
                let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);
                let dir_class = if is_rtl { "rtl" } else { "ltr" };
                let text: String = runs.iter().map(|r| {
                    let mut s = r.text.clone();
                    if r.bold { s = format!("<strong>{}</strong>", s); }
                    if r.italic { s = format!("<em>{}</em>", s); }
                    if r.underline { s = format!("<u>{}</u>", s); }
                    s
                }).collect();
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
                format!(
                    "<pre><code class=\"language-{}\">{}</code></pre>\n",
                    language, code
                )
            }
            Node::Image { path, alt_text, .. } => {
                format!("<img src=\"{}\" alt=\"{}\" />\n", path, alt_text)
            }
            Node::Divider => "<hr/>\n".to_string(),
            Node::PageBreak => "<hr style=\"border-top: 3px double #ddd;\"/>\n".to_string(),
        }
    }

    html.push_str(&node_to_html(&document.content));
    html.push_str("</body>\n</html>");
    html
}

pub fn export_as_png(width: u32, height: u32) -> Result<Vec<u8>, String> {
    let img = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));

    let mut buffer = Vec::new();
    ::image::DynamicImage::ImageRgba8(img)
        .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(buffer)
}

// ============================================================================
// Improved PDF Export with proper multi-page support, line wrapping, margins
// ============================================================================

const PAGE_WIDTH_MM: f32 = 210.0;
const PAGE_HEIGHT_MM: f32 = 297.0;
const MARGIN_LEFT: f32 = 20.0;
const MARGIN_RIGHT: f32 = 20.0;
const MARGIN_TOP: f32 = 277.0; // from bottom in printpdf coords
const MARGIN_BOTTOM: f32 = 20.0;
const LINE_HEIGHT_BODY: f32 = 7.0;
const LINE_HEIGHT_H1: f32 = 14.0;
const LINE_HEIGHT_H2: f32 = 11.0;
const LINE_HEIGHT_H3: f32 = 9.0;
const CHARS_PER_LINE_BODY: usize = 85;
const CHARS_PER_LINE_HEADING: usize = 60;

struct PdfState<'a> {
    doc: &'a PdfDocumentReference,
    font: &'a IndirectFontRef,
    font_bold: &'a IndirectFontRef,
    current_layer: PdfLayerReference,
    y_position: f32,
    page_count: u32,
}

impl<'a> PdfState<'a> {
    fn new_page(&mut self) {
        self.page_count += 1;
        let (page, layer) = self.doc.add_page(
            Mm(PAGE_WIDTH_MM),
            Mm(PAGE_HEIGHT_MM),
            &format!("Layer {}", self.page_count),
        );
        self.current_layer = self.doc.get_page(page).get_layer(layer);
        self.y_position = MARGIN_TOP;
    }

    fn ensure_space(&mut self, needed: f32) {
        if self.y_position - needed < MARGIN_BOTTOM {
            self.new_page();
        }
    }

    fn write_line(&mut self, text: &str, font_size: f32, x: f32, use_bold: bool) {
        let font = if use_bold { self.font_bold } else { self.font };
        self.current_layer
            .use_text(text, font_size, Mm(x), Mm(self.y_position), font);
    }

    fn advance(&mut self, amount: f32) {
        self.y_position -= amount;
    }
}

/// Simple word-wrap: splits text into lines that fit within max_chars
fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return lines;
    }

    let mut current_line = String::new();
    for word in &words {
        if current_line.is_empty() {
            current_line.push_str(word);
        } else if current_line.chars().count() + 1 + word.chars().count() <= max_chars {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line.clone());
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

fn render_node_to_pdf(node: &Node, state: &mut PdfState) {
    match node {
        Node::Document { children } => {
            for child in children {
                render_node_to_pdf(child, state);
            }
        }

        Node::Heading { runs, level, .. } => {
            let (font_size, line_height, spacing_before, spacing_after) = match level {
                1 => (22.0_f32, LINE_HEIGHT_H1, 8.0_f32, 5.0_f32),
                2 => (17.0_f32, LINE_HEIGHT_H2, 6.0_f32, 4.0_f32),
                _ => (14.0_f32, LINE_HEIGHT_H3, 5.0_f32, 3.0_f32),
            };

            let text: String = runs.iter().map(|r| r.text.clone()).collect();
            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);
            let processed = pdx_text(&text);
            let wrapped = wrap_text(&processed, CHARS_PER_LINE_HEADING);

            state.ensure_space(spacing_before + (wrapped.len() as f32 * line_height) + spacing_after);
            state.advance(spacing_before);

            let x_pos = if is_rtl { PAGE_WIDTH_MM - MARGIN_RIGHT - 10.0 } else { MARGIN_LEFT };

            for line in &wrapped {
                state.ensure_space(line_height + 2.0);
                state.write_line(line, font_size, x_pos, true);
                state.advance(line_height);
            }
            state.advance(spacing_after);
        }

        Node::Paragraph { runs, .. } => {
            let text: String = runs.iter().map(|r| r.text.clone()).collect();
            if text.trim().is_empty() {
                return;
            }
            let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);
            let processed = pdx_text(&text);
            let wrapped = wrap_text(&processed, CHARS_PER_LINE_BODY);

            state.ensure_space((wrapped.len() as f32 * LINE_HEIGHT_BODY) + 4.0);

            let x_pos = if is_rtl { PAGE_WIDTH_MM - MARGIN_RIGHT - 10.0 } else { MARGIN_LEFT };

            let use_bold = runs.iter().any(|r| r.bold);
            for line in &wrapped {
                state.ensure_space(LINE_HEIGHT_BODY + 1.0);
                state.write_line(line, 11.0, x_pos, use_bold);
                state.advance(LINE_HEIGHT_BODY);
            }
            state.advance(4.0);
        }

        Node::List { items, ordered, .. } => {
            state.ensure_space(5.0);
            state.advance(3.0);
            for (i, item) in items.iter().enumerate() {
                let text: String = item.content.iter().map(|r| r.text.clone()).collect();
                let is_rtl = item.content.iter().any(|r| r.direction == Direction::RTL);
                let marker = if *ordered {
                    format!("{}. ", i + 1)
                } else {
                    "• ".to_string()
                };
                let full_text = format!("{}{}", marker, text);
                let processed = pdx_text(&full_text);
                let wrapped = wrap_text(&processed, CHARS_PER_LINE_BODY - 4);

                let x_pos = if is_rtl {
                    PAGE_WIDTH_MM - MARGIN_RIGHT - 15.0
                } else {
                    MARGIN_LEFT + 8.0
                };

                for (j, line) in wrapped.iter().enumerate() {
                    state.ensure_space(LINE_HEIGHT_BODY + 1.0);
                    let x = if j == 0 { x_pos } else { x_pos + 8.0 };
                    state.write_line(line, 11.0, x, false);
                    state.advance(LINE_HEIGHT_BODY);
                }
            }
            state.advance(5.0);
        }

        Node::CodeBlock { language, code, .. } => {
            state.ensure_space(8.0);
            state.advance(4.0);

            // Language label
            let lang_label = format!("[{}]", language);
            state.write_line(&lang_label, 9.0, MARGIN_LEFT, false);
            state.advance(6.0);

            // Code lines
            for code_line in code.lines() {
                let trimmed = if code_line.len() > CHARS_PER_LINE_BODY {
                    &code_line[..CHARS_PER_LINE_BODY]
                } else {
                    code_line
                };
                state.ensure_space(LINE_HEIGHT_BODY);
                state.write_line(trimmed, 9.0, MARGIN_LEFT + 4.0, false);
                state.advance(LINE_HEIGHT_BODY - 1.0);
            }
            state.advance(6.0);
        }

        Node::Divider => {
            state.ensure_space(8.0);
            state.advance(4.0);
            // Draw a horizontal line
            let line = Line {
                points: vec![
                    (Point::new(Mm(MARGIN_LEFT), Mm(state.y_position)), false),
                    (Point::new(Mm(PAGE_WIDTH_MM - MARGIN_RIGHT), Mm(state.y_position)), false),
                ],
                is_closed: false,
            };
            let outline_color = Color::Rgb(Rgb::new(0.7, 0.7, 0.7, None));
            state.current_layer.set_outline_color(outline_color);
            state.current_layer.set_outline_thickness(0.5);
            state.current_layer.add_line(line);
            state.advance(6.0);
        }

        Node::PageBreak => {
            state.new_page();
        }

        Node::Image { alt_text, .. } => {
            // Just show a placeholder text for images in PDF
            let placeholder = format!("[Image: {}]", alt_text);
            state.ensure_space(LINE_HEIGHT_BODY + 2.0);
            state.write_line(&placeholder, 10.0, MARGIN_LEFT, false);
            state.advance(LINE_HEIGHT_BODY + 4.0);
        }
    }
}

pub fn export_as_pdf(document: &PdxDocument) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) = PdfDocument::new(
        &document.metadata.title,
        Mm(PAGE_WIDTH_MM),
        Mm(PAGE_HEIGHT_MM),
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load both regular and bold Arabic fonts
    let font_bytes = include_bytes!("../assets/fonts/NotoSansArabic-Regular.ttf");
    let font = doc
        .add_external_font(font_bytes.as_ref())
        .map_err(|e| format!("Font error: {:?}", e))?;

    // Use same font for bold (fallback since we only have one Arabic font)
    let font_bold = doc
        .add_external_font(font_bytes.as_ref())
        .map_err(|e| format!("Font error: {:?}", e))?;

    // Write document title at the top
    current_layer.use_text(
        &document.metadata.title,
        24.0,
        Mm(MARGIN_LEFT),
        Mm(MARGIN_TOP + 8.0),
        &font,
    );

    // Author & subtitle line
    if !document.metadata.author.is_empty() {
        current_layer.use_text(
            &format!("By: {}", document.metadata.author),
            11.0,
            Mm(MARGIN_LEFT),
            Mm(MARGIN_TOP),
            &font,
        );
    }

    let mut state = PdfState {
        doc: &doc,
        font: &font,
        font_bold: &font_bold,
        current_layer,
        y_position: MARGIN_TOP - 14.0,
        page_count: 1,
    };

    render_node_to_pdf(&document.content, &mut state);

    let mut buffer = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        doc.save(&mut writer)
            .map_err(|e| format!("PDF save error: {:?}", e))?;
    }

    Ok(buffer)
}
