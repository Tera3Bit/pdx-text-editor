\# PDX Editor - Modular Code Structure

\## 📁 Project Structure

```

pdx-editor/

├── src/

│   ├── main.rs          # Entry point

│   ├── app.rs           # Main application logic \& UI rendering

│   ├── data.rs          # Data structures (Document, Node, Metadata, etc.)

│   ├── theme.rs         # Theme system (Light, Dark, Comfort, etc.)

│   ├── parser.rs        # Content parsing \& serialization

│   ├── renderer.rs      # Document rendering logic

│   ├── export.rs        # Export functions (HTML, PDF, PNG)

│   ├── ui.rs            # UI setup \& file operations

│   └── pdx\_text.rs      # Text processing utilities

├── assets/

│   └── fonts/

│       └── NotoSansArabic-Regular.ttf

├── Cargo.toml

└── README.md

```

---

\## 📦 Module Breakdown

\### 1. `main.rs` - Entry Point

\*\*Purpose:\*\* Application entry and initialization

\*\*Responsibilities:\*\*

\- Initialize eframe window

\- Setup fonts

\- Create PdxApp instance

\*\*Lines:\*\* ~30

---

\### 2. `data.rs` - Core Data Structures

\*\*Purpose:\*\* All document data models

\*\*Contains:\*\*

\- `PdxDocument` - Main document structure

\- `Node` - Content nodes (Heading, Paragraph, List, Image, etc.)

\- `Metadata` - Document metadata

\- `StyleSheet` - Styling information

\- `TextRun` - Text with direction/language info

\- `Resources` - Image resources

\- Helper enums: `Direction`, `FontWeight`, `TextAlign`, `Color`

\*\*Lines:\*\* ~350

\*\*Key Types:\*\*

```rust

pub struct PdxDocument { ... }

pub enum Node { ... }

pub struct TextRun { ... }

```

---

\### 3. `theme.rs` - Theme System

\*\*Purpose:\*\* Visual themes and color management

\*\*Contains:\*\*

\- `AppTheme` enum (Light, Dark, Midnight, Sepia, Comfort)

\- Color definitions for each theme

\- Theme application logic

\*\*Lines:\*\* ~120

\*\*Usage:\*\*

```rust

let theme = AppTheme::Comfort;

theme.apply(ctx);

```

---

\### 4. `parser.rs` - Content Parsing

\*\*Purpose:\*\* Convert between text and document structure

\*\*Functions:\*\*

\- `serialize\_content(node) -> String` - Node to Markdown

\- `parse\_content(text) -> Node` - Markdown to Node

\*\*Lines:\*\* ~180

\*\*Supports:\*\*

\- Headings (#, ##, ###)

\- Paragraphs

\- Lists (-, •)

\- Code blocks (```)

\- Images (!\[](path))

\- Dividers (---)

\- Page breaks (===)

---

\### 5. `renderer.rs` - Document Rendering

\*\*Purpose:\*\* Render document to screen

\*\*Main Function:\*\*

```rust

pub fn render\_node(

&nbsp;   ui: \&mut egui::Ui,

&nbsp;   node: \&Node,

&nbsp;   styles: \&StyleSheet,

&nbsp;   zoom: f32,

&nbsp;   theme: \&AppTheme,

&nbsp;   images: \&HashMap<String, egui::TextureHandle>,

)

```

\*\*Lines:\*\* ~200

\*\*Features:\*\*

\- RTL text support

\- Image rendering

\- Style application

\- Zoom support

---

\### 6. `export.rs` - Export Functions

\*\*Purpose:\*\* Export documents to different formats

\*\*Functions:\*\*

\- `export\_as\_html(document) -> String`

\- `export\_as\_pdf(document) -> Result<Vec<u8>, String>`

\- `export\_as\_png(width, height) -> Result<Vec<u8>, String>`

\*\*Lines:\*\* ~230

\*\*Formats:\*\*

\- HTML with CSS

\- PDF with Arabic font embedding

\- PNG images

---

\### 7. `ui.rs` - UI \& File Operations

\*\*Purpose:\*\* UI setup and file I/O

\*\*Functions:\*\*

\- `setup\_fonts(ctx)` - Load Arabic fonts

\- `open\_document() -> Option<(PdxDocument, PathBuf)>`

\- `save\_document(...) -> Option<PathBuf>`

\- `export\_html(...) -> Option<()>`

\- `export\_pdf\_file(...) -> Option<()>`

\- `export\_png\_file() -> Option<()>`

\- `insert\_image() -> Option<String>`

\*\*Lines:\*\* ~130

---

\### 8. `app.rs` - Application Logic

\*\*Purpose:\*\* Main app state and UI rendering

\*\*Contains:\*\*

\- `PdxApp` struct - App state

\- Menu rendering

\- Editor tab rendering

\- Image loading

\- Event handling

\*\*Lines:\*\* ~400

\*\*State:\*\*

```rust

pub struct PdxApp {

&nbsp;   document: PdxDocument,

&nbsp;   path: Option<PathBuf>,

&nbsp;   mode: EditorMode,

&nbsp;   theme: AppTheme,

&nbsp;   // ...

}

```

---

\### 9. `pdx\_text.rs` - Text Processing

\*\*Purpose:\*\* Text utilities and processing

\*\*Current:\*\*

\- Simple pass-through function

\*\*Future Extensions:\*\*

\- Unicode normalization

\- RTL text shaping

\- Font fallback logic

\- Text analysis

\*\*Lines:\*\* ~5

---

\## 🔧 Adding New Features

\### Example 1: Add New Theme

\*\*File:\*\* `src/theme.rs`

```rust

\#\[derive(Debug, Clone, Copy, PartialEq)]

pub enum AppTheme {

&nbsp;   // ... existing themes

&nbsp;   Ocean,  // NEW

}



impl AppTheme {

&nbsp;   pub fn text\_color(\&self) -> egui::Color32 {

&nbsp;       match self {

&nbsp;           // ... existing

&nbsp;           AppTheme::Ocean => egui::Color32::from\_rgb(20, 40, 60),

&nbsp;       }

&nbsp;   }

&nbsp;   // ... add to other methods

}

```

---

\### Example 2: Add New Node Type

\*\*File:\*\* `src/data.rs`

```rust

pub enum Node {

&nbsp;   // ... existing

&nbsp;   Table {

&nbsp;       headers: Vec<String>,

&nbsp;       rows: Vec<Vec<String>>,

&nbsp;   },

}

```

\*\*File:\*\* `src/parser.rs`

```rust

// Add parsing logic for tables

```

\*\*File:\*\* `src/renderer.rs`

```rust

// Add rendering logic for tables

```

---

\### Example 3: Add New Export Format

\*\*File:\*\* `src/export.rs`

```rust

pub fn export\_as\_markdown(document: \&PdxDocument) -> String {

&nbsp;   // Implementation

}

```

\*\*File:\*\* `src/ui.rs`

```rust

pub fn export\_markdown\_file(document: \&PdxDocument) -> Option<()> {

&nbsp;   // File dialog + save

}

```

\*\*File:\*\* `src/app.rs`

```rust

// Add menu item in render\_file\_menu

if ui.button("📝 Markdown").clicked() {

&nbsp;   if export\_markdown\_file(\&self.document).is\_some() {

&nbsp;       self.status\_message = "Exported as Markdown".to\_string();

&nbsp;   }

}

```

---

\## 🧪 Testing Strategy

\### Unit Tests

\*\*File:\*\* `src/parser.rs`

```rust

\#\[cfg(test)]

mod tests {

&nbsp;   use super::\*;

&nbsp;

&nbsp;   #\[test]

&nbsp;   fn test\_parse\_heading() {

&nbsp;       let content = parse\_content("# Hello");

&nbsp;       // assertions

&nbsp;   }

}

```

\### Integration Tests

\*\*File:\*\* `tests/integration\_test.rs`

```rust

\#\[test]

fn test\_full\_workflow() {

&nbsp;   // Create document

&nbsp;   // Parse content

&nbsp;   // Render

&nbsp;   // Export

}

```

---

\## 📊 Complexity Metrics

| Module | Lines | Complexity | Purpose |

|--------|-------|------------|---------|

| data.rs | ~350 | Low | Data structures |

| app.rs | ~400 | Medium | UI logic |

| renderer.rs | ~200 | Medium | Rendering |

| export.rs | ~230 | High | Format conversion |

| parser.rs | ~180 | Medium | Text parsing |

| theme.rs | ~120 | Low | Theming |

| ui.rs | ~130 | Low | File I/O |

| main.rs | ~30 | Low | Entry point |

| pdx_text.rs | ~5 | Low | Text utils |

---

\## 🚀 Build \& Run

```bash

\# Development

cargo run



\# Release (optimized)

cargo build --release

./target/release/pdx-editor-enhanced



\# Run tests

cargo test



\# Format code

cargo fmt



\# Check for issues

cargo clippy

```

---

\## 📝 Code Style Guidelines

1\. \*\*Modules:\*\* One primary purpose per file

2\. \*\*Functions:\*\* Small, focused, well-named

3\. \*\*Comments:\*\* Explain "why", not "what"

4\. \*\*Errors:\*\* Use `Result` for recoverable errors

5\. \*\*Public API:\*\* Minimal, well-documented

---

\## 🔮 Future Refactoring Ideas

\### 1. Split `app.rs` Further

```

src/

&nbsp; app/

&nbsp;   mod.rs          # App struct

&nbsp;   menu.rs         # Menu rendering

&nbsp;   editor.rs       # Editor tab

&nbsp;   metadata.rs     # Metadata tab

&nbsp;   styles.rs       # Styles tab

```

\### 2. Separate Export Backends

```

src/

&nbsp; export/

&nbsp;   mod.rs

&nbsp;   html.rs

&nbsp;   pdf.rs

&nbsp;   png.rs

```

\### 3. Plugin System

```

src/

&nbsp; plugins/

&nbsp;   mod.rs

&nbsp;   trait.rs      # Plugin trait

&nbsp;   loader.rs     # Plugin loader

```

---

\## 💡 Benefits of This Structure

✅ \*\*Maintainability\*\* - Easy to find and fix bugs

✅ \*\*Testability\*\* - Each module can be tested independently

✅ \*\*Extensibility\*\* - Add features without touching core logic

✅ \*\*Readability\*\* - Clear separation of concerns

✅ \*\*Reusability\*\* - Modules can be reused in other projects

✅ \*\*Collaboration\*\* - Multiple developers can work simultaneously

---

\## 🤝 Contributing

When adding features:

1\. \*\*Identify the right module\*\* - Where does this belong?

2\. \*\*Add public API\*\* - What functions/types are needed?

3\. \*\*Implement\*\* - Write the code

4\. \*\*Test\*\* - Add unit/integration tests

5\. \*\*Document\*\* - Update README if needed

6\. \*\*Update dependents\*\* - Modify calling code

---

\## 📚 Dependencies by Module

| Module | Dependencies |

|--------|-------------|

| main.rs | eframe, app, ui |

| app.rs | data, parser, renderer, theme, ui, egui, image |

| data.rs | serde, chrono, image |

| theme.rs | egui |

| parser.rs | data |

| renderer.rs | data, theme, pdx_text, egui |

| export.rs | data, pdx_text, printpdf, image |

| ui.rs | data, export, egui, std::fs, rfd |

| pdx_text.rs | (none) |

---

\*\*Happy Coding! 🎉\*\*
