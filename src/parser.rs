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
                .map(|r| serialize_run(r))
                .collect::<Vec<_>>()
                .join(" ");
            format!("{} {}", prefix, text)
        }

        Node::Paragraph { runs, .. } => runs
            .iter()
            .map(|r| serialize_run(r))
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
                    .map(|r| serialize_run(r))
                    .collect::<Vec<_>>()
                    .join("");
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

fn serialize_run(run: &TextRun) -> String {
    let mut text = run.text.clone();
    if run.underline {
        text = format!("__{text}__");
    }
    if run.italic {
        text = format!("*{text}*");
    }
    if run.bold {
        text = format!("**{text}**");
    }
    text
}

// ============================================================================
// Inline Markdown Parser: **bold**, *italic*, __underline__
// ============================================================================

fn parse_inline(text: &str, language: &str, style: &str) -> Vec<TextRun> {
    let mut runs = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;
    let mut current = String::new();

    while i < chars.len() {
        // Check for **bold**
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            if !current.is_empty() {
                let mut run = TextRun::new(&current, language, style);
                run.direction = detect_direction(language);
                runs.push(run);
                current = String::new();
            }
            i += 2;
            let mut inner = String::new();
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '*') {
                inner.push(chars[i]);
                i += 1;
            }
            if i + 1 < chars.len() {
                i += 2; // skip closing **
            }
            let mut run = TextRun::new(&inner, language, style);
            run.bold = true;
            run.direction = detect_direction(language);
            runs.push(run);
            continue;
        }

        // Check for __underline__
        if i + 1 < chars.len() && chars[i] == '_' && chars[i + 1] == '_' {
            if !current.is_empty() {
                let mut run = TextRun::new(&current, language, style);
                run.direction = detect_direction(language);
                runs.push(run);
                current = String::new();
            }
            i += 2;
            let mut inner = String::new();
            while i + 1 < chars.len() && !(chars[i] == '_' && chars[i + 1] == '_') {
                inner.push(chars[i]);
                i += 1;
            }
            if i + 1 < chars.len() {
                i += 2;
            }
            let mut run = TextRun::new(&inner, language, style);
            run.underline = true;
            run.direction = detect_direction(language);
            runs.push(run);
            continue;
        }

        // Check for *italic* (single star)
        if chars[i] == '*' {
            if !current.is_empty() {
                let mut run = TextRun::new(&current, language, style);
                run.direction = detect_direction(language);
                runs.push(run);
                current = String::new();
            }
            i += 1;
            let mut inner = String::new();
            while i < chars.len() && chars[i] != '*' {
                inner.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                i += 1; // skip closing *
            }
            let mut run = TextRun::new(&inner, language, style);
            run.italic = true;
            run.direction = detect_direction(language);
            runs.push(run);
            continue;
        }

        current.push(chars[i]);
        i += 1;
    }

    if !current.is_empty() {
        let mut run = TextRun::new(&current, language, style);
        run.direction = detect_direction(language);
        runs.push(run);
    }

    if runs.is_empty() {
        let mut run = TextRun::new(text, language, style);
        run.direction = detect_direction(language);
        runs.push(run);
    }

    runs
}

fn is_ordered_list_item(line: &str) -> bool {
    // Matches "1. " "12. " etc. at the start of a line
    let mut chars = line.chars();
    let first = chars.next();
    if !first.map(|c| c.is_ascii_digit()).unwrap_or(false) {
        return false;
    }
    for ch in chars {
        if ch == '.' {
            return true;
        }
        if !ch.is_ascii_digit() {
            return false;
        }
    }
    false
}

fn detect_direction(language: &str) -> Direction {
    if language == "ar" || language == "fa" || language == "ur" {
        Direction::RTL
    } else {
        Direction::LTR
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
                // Find the closing ')' AFTER the opening '(' to avoid matching wrong paren
                let search_from = close_bracket + 2;
                if let Some(rel_paren) = line[search_from..].find(')') {
                    let close_paren = search_from + rel_paren;
                    let alt_text = &line[2..close_bracket];
                    let path = &line[search_from..close_paren];

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
            let lang = if is_arabic { "ar" } else { "en" };
            let style_name = format!("heading{}", level);

            children.push(Node::Heading {
                level,
                runs: parse_inline(text, lang, &style_name),
                style: style_name,
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
        } else if line.starts_with('-') || line.starts_with('•') {
            let mut items = Vec::new();

            while i < lines.len() {
                let l = lines[i].trim();
                if l.starts_with('-') || l.starts_with('•') {
                    let text = l.trim_start_matches('-').trim_start_matches('•').trim();
                    let is_arabic = text.chars().any(|c| c >= '\u{0600}' && c <= '\u{06FF}');
                    let lang = if is_arabic { "ar" } else { "en" };
                    items.push(ListItem {
                        content: parse_inline(text, lang, "paragraph"),
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
        } else if is_ordered_list_item(line) {
            // Ordered list: lines starting with "1." "2." etc.
            let mut items = Vec::new();

            while i < lines.len() {
                let l = lines[i].trim();
                if is_ordered_list_item(l) {
                    // Strip leading "N." prefix
                    let text = l.splitn(2, '.').nth(1).unwrap_or("").trim();
                    let is_arabic = text.chars().any(|c| c >= '\u{0600}' && c <= '\u{06FF}');
                    let lang = if is_arabic { "ar" } else { "en" };
                    items.push(ListItem {
                        content: parse_inline(text, lang, "paragraph"),
                    });
                    i += 1;
                } else {
                    break;
                }
            }

            children.push(Node::List {
                ordered: true,
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
            let lang = if is_arabic { "ar" } else { "en" };
            let style = if is_arabic { "arabic" } else { "paragraph" };

            children.push(Node::Paragraph {
                runs: parse_inline(line, lang, style),
                style: style.to_string(),
            });
        }

        i += 1;
    }

    Node::Document { children }
}