use crate::data::{Direction, ListItem, Node, TextRun};

// ============================================================================
// Content Serialization
// ============================================================================

pub fn serialize_content(node: &Node) -> String {
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

        Node::Image { path, alt_text, .. } => {
            format!("![{}]({})", alt_text, path)
        }

        Node::Divider => "---".to_string(),
        Node::PageBreak => "===".to_string(),
    }
}

pub fn parse_content(text: &str) -> Node {
    let mut children = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        if line.is_empty() {
            i += 1;
            continue;
        }

        // Image syntax: ![alt text](path)
        if line.starts_with("![") {
            if let Some(close_bracket) = line.find("](") {
                if let Some(close_paren) = line.find(')') {
                    let alt_text = &line[2..close_bracket];
                    let path = &line[close_bracket + 2..close_paren];

                    children.push(Node::Image {
                        path: path.to_string(),
                        alt_text: alt_text.to_string(),
                        width: None,
                        height: None,
                    });
                    i += 1;
                    continue;
                }
            }
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