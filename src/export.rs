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
                format!(
                    "<h{} class=\"{}\">{}</h{}>\n",
                    level, dir_class, text, level
                )
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
    // Create a simple rendered version
    let img = ImageBuffer::from_pixel(width, height, Rgba([255, 255, 255, 255]));

    let mut buffer = Vec::new();
    ::image::DynamicImage::ImageRgba8(img)
        .write_to(&mut std::io::Cursor::new(&mut buffer), ImageFormat::Png)
        .map_err(|e| e.to_string())?;

    Ok(buffer)
}

pub fn export_as_pdf(document: &PdxDocument) -> Result<Vec<u8>, String> {
    let (doc, page1, layer1) =
        PdfDocument::new(&document.metadata.title, Mm(210.0), Mm(297.0), "Layer 1");

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load Arabic font
    let font_bytes = include_bytes!("../assets/fonts/NotoSansArabic-Regular.ttf");
    let font = doc
        .add_external_font(font_bytes.as_ref())
        .map_err(|e| format!("Font error: {:?}", e))?;

    let mut y_position = 270.0; // Start from top

    fn render_node_to_pdf(
        node: &Node,
        layer: &PdfLayerReference,
        font: &IndirectFontRef,
        y_pos: &mut f32,
        x_start: f32,
    ) {
        match node {
            Node::Document { children } => {
                for child in children {
                    render_node_to_pdf(child, layer, font, y_pos, x_start);
                }
            }
            Node::Heading { runs, level, .. } => {
                let font_size = match level {
                    1 => 24.0,
                    2 => 20.0,
                    _ => 16.0,
                };

                let text: String = runs.iter().map(|r| r.text.clone()).collect();
                let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);

                let x_pos = if is_rtl { 190.0 } else { x_start };

                layer.use_text(&pdx_text(&text), font_size, Mm(x_pos), Mm(*y_pos), font);

                *y_pos -= font_size * 0.5 + 10.0;
            }
            Node::Paragraph { runs, .. } => {
                let text: String = runs.iter().map(|r| r.text.clone()).collect();
                let is_rtl = runs.iter().any(|r| r.direction == Direction::RTL);

                let x_pos = if is_rtl { 190.0 } else { x_start };

                layer.use_text(&pdx_text(&text), 12.0, Mm(x_pos), Mm(*y_pos), font);

                *y_pos -= 20.0;
            }
            Node::List { items, ordered, .. } => {
                for (i, item) in items.iter().enumerate() {
                    let marker = if *ordered {
                        format!("{}.", i + 1)
                    } else {
                        "•".to_string()
                    };

                    let text: String = item.content.iter().map(|r| r.text.clone()).collect();
                    let full_text = format!("{} {}", marker, text);

                    layer.use_text(
                        &pdx_text(&full_text),
                        12.0,
                        Mm(x_start + 5.0),
                        Mm(*y_pos),
                        font,
                    );

                    *y_pos -= 15.0;
                }
                *y_pos -= 5.0;
            }
            Node::Divider => {
                *y_pos -= 20.0;
            }
            Node::PageBreak => {
                *y_pos = 270.0;
            }
            _ => {}
        }
    }

    render_node_to_pdf(
        &document.content,
        &current_layer,
        &font,
        &mut y_position,
        20.0,
    );

    let mut buffer = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buffer);
        doc.save(&mut writer)
            .map_err(|e| format!("PDF save error: {:?}", e))?;
    }

    Ok(buffer)
}
