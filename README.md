# PDX Editor Enhanced - New Features Documentation

## Overview

PDX Editor Enhanced is an upgraded version with professional export capabilities, image support, and an eye-friendly theme designed for extended writing sessions.

---

## 🆕 NEW FEATURES

### 1. Real PDF Export with Arabic Support ✅

**What changed:** Previously, PDF export was just a browser print suggestion. Now it's a real PDF generator.

**Features:**

- ✅ **Native PDF generation** using `printpdf` library
- ✅ **Arabic font embedding** - Noto Sans Arabic is embedded in the PDF
- ✅ **RTL (Right-to-Left) support** - Arabic text renders correctly
- ✅ **Proper text positioning** - Headings, paragraphs, and lists are properly laid out
- ✅ **Multi-page support** - Automatic page breaks when content is too long

**How to use:**

1. Go to `File` → `Export as...` → `PDF (Real Export)`
2. Choose save location
3. PDF is generated with full Arabic support

**Technical details:**

```rust
// Embeds Arabic font directly into PDF
let font_bytes = include_bytes!("../assets/fonts/NotoSansArabic-Regular.ttf");
let font = doc.add_external_font(font_bytes.as_ref())?;

// Supports RTL text positioning
let x_pos = if is_rtl { 190.0 } else { x_start };
```

---

### 2. PNG Image Export ✅

**What changed:** Added ability to export entire document as a PNG image.

**Features:**

- ✅ Export document as high-resolution PNG
- ✅ Configurable width/height (default 1200x1600)
- ✅ Perfect for sharing on social media or embedding in presentations

**How to use:**

1. Go to `File` → `Export as...` → `PNG Image (Real Export)`
2. Choose save location
3. PNG file is created

**Use cases:**

- Share document previews on social media
- Create thumbnails
- Embed in presentations
- Print as image

---

### 3. Image Embedding in Documents ✅

**What changed:** Documents can now contain embedded images, not just text.

**Features:**

- ✅ **Insert images** from local files
- ✅ **Multiple image formats** supported: PNG, JPG, JPEG, GIF, WebP
- ✅ **Automatic rendering** in preview mode
- ✅ **Markdown syntax**: `![alt text](path/to/image.png)`
- ✅ **Images saved in document** structure

**How to use:**

**Method 1 - Using Menu:**

1. Go to `File` → `Insert Image...`
2. Select image file
3. Image markup is inserted at cursor position

**Method 2 - Markdown Syntax:**
Type directly in editor:

```markdown
![My Image](path/to/image.png)
```

**Example document with image:**

```markdown
# My Document

This is a paragraph.

![Screenshot](screenshots/demo.png)

Another paragraph after the image.
```

**New Node Type:**

```rust
Node::Image {
    path: String,        // File path
    alt_text: String,    // Alternative text
    width: Option<f32>,  // Optional width
    height: Option<f32>, // Optional height
}
```

---

### 4. Comfort Theme (Eye-Friendly) 🌿✅

**What changed:** New theme specifically designed for long writing sessions.

**Features:**

- ✅ **Reduced eye strain** - Soft green-tinted background
- ✅ **Lower contrast** - Gentler on eyes than pure black/white
- ✅ **Warm color palette** - Based on research on comfortable reading
- ✅ **Perfect for 2+ hour sessions**

**Color Science:**

```rust
AppTheme::Comfort => {
    text_color: rgb(45, 55, 65),      // Soft blue-gray (not harsh black)
    background: rgb(248, 250, 245),   // Very soft green tint
    panel: rgb(252, 253, 250),        // Warm white with green tint
}
```

**Why these colors?**

- Green tint reduces blue light exposure
- Lower contrast reduces eye fatigue
- Warm tones are more comfortable for extended reading
- Based on studies showing green light is less straining

**How to activate:**

1. Go to `Theme` menu
2. Select `🌿 Comfort`

**Comparison:**

- **Light theme:** High contrast, pure white - good for short sessions
- **Dark theme:** Pure black - good for low-light environments
- **Comfort theme:** Balanced, green-tinted - optimal for 2+ hour writing

---

## 🔧 TECHNICAL IMPROVEMENTS

### PDF Generation Architecture

**Problem:** The old version just suggested browser print, which:

- Didn't embed fonts properly
- Broke Arabic text rendering
- Couldn't customize layout
- Not reliable across browsers

**Solution:**

```rust
use printpdf::*;

fn export_as_pdf(document: &PdxDocument) -> Result<Vec<u8>, String> {
    // 1. Create PDF document
    let (doc, page1, layer1) = PdfDocument::new(
        &document.metadata.title,
        Mm(210.0), Mm(297.0),  // A4 size
        "Layer 1"
    );

    // 2. Embed Arabic font
    let font = doc.add_external_font(font_bytes)?;

    // 3. Render nodes with proper positioning
    render_node_to_pdf(&document.content, &layer, &font, ...);

    // 4. Return as bytes
    doc.save(&mut buffer)?;
    Ok(buffer)
}
```

**Benefits:**
✅ Fonts are embedded
✅ Arabic renders perfectly
✅ Consistent across all platforms
✅ Full control over layout

---

### Image Loading System

**New resource management:**

```rust
struct PdxApp {
    loaded_images: HashMap<String, egui::TextureHandle>,
    // ...
}

fn load_images_from_content(&mut self, ctx: &egui::Context) {
    // 1. Collect all image paths from document
    let mut image_paths = Vec::new();
    collect_image_paths(&self.document.content, &mut image_paths);

    // 2. Load each image as texture (cached)
    for path in image_paths {
        if !self.loaded_images.contains_key(&path) {
            let img = image::open(&path)?;
            let texture = ctx.load_texture(&path, color_image, ...);
            self.loaded_images.insert(path, texture);
        }
    }
}
```

**Features:**

- Images loaded on-demand
- Cached for performance
- Converted to egui textures for rendering
- Supports RGBA with transparency

---

## 📖 USAGE EXAMPLES

### Example 1: Document with Images

```markdown
# Travel Blog

## Day 1: Cairo

We visited the pyramids today!

![Pyramids](photos/pyramids.jpg)

The pyramids were amazing. Here's a close-up:

![Sphinx](photos/sphinx.jpg)

## Day 2: Alexandria

The Mediterranean coast is beautiful.

![Beach](photos/alex-beach.jpg)
```

### Example 2: Bilingual Document with Images

```markdown
# Product Manual - دليل المنتج

## English Section

![Product Photo](images/product.png)

This is the product description.

---

## القسم العربي

![صورة المنتج](images/product.png)

هذا هو وصف المنتج باللغة العربية.
```

### Example 3: Research Paper

```markdown
# Research: Effect of Green Light on Eye Strain

## Abstract

This study examines...

## Methodology

![Experimental Setup](figures/setup.png)

We used the following equipment...

## Results

![Graph 1](figures/results-graph.png)

As shown in Figure 1, the green-tinted display reduced...

## Arabic Summary - ملخص عربي

![الرسم البياني](figures/results-graph.png)

أظهرت النتائج أن...
```

---

## 🎨 THEME COMPARISON

| Theme       | Best For                        | Background                   | Text Color               | Eye Strain   |
| ----------- | ------------------------------- | ---------------------------- | ------------------------ | ------------ |
| Light       | Short tasks, high ambient light | Pure white (250,250,250)     | Near black (20,20,20)    | Medium       |
| Dark        | Low light, night work           | Dark gray (30,30,35)         | Light gray (230,230,230) | Low-Medium   |
| Midnight    | Very low light, night           | Deep blue (15,20,35)         | Blue-gray (200,210,230)  | Low          |
| Sepia       | Reading, warm tone lovers       | Sepia (245,235,215)          | Dark brown (60,50,40)    | Low          |
| **Comfort** | **Long sessions (2+ hours)**    | **Soft green (248,250,245)** | **Blue-gray (45,55,65)** | **Very Low** |

**Recommendation:** Use Comfort theme for writing sessions longer than 1 hour.

---

## 🚀 PERFORMANCE

### Memory Management

- Images are cached in HashMap
- Textures loaded once, reused
- Old textures automatically freed by egui

### PDF Generation

- Streaming to buffer (no temp files)
- Font embedded once per document
- Efficient text positioning

### Image Loading

- Lazy loading (only when needed)
- Format auto-detection
- RGBA conversion for transparency

---

## 📋 DEPENDENCIES

```toml
[dependencies]
eframe = "0.30"
egui = "0.30"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rfd = "0.15"
image = { version = "0.25", features = ["png", "jpeg"] }
printpdf = { version = "0.7", features = ["embedded_images"] }
chrono = "0.4"
unicode-bidi = "0.3" 
arabic_reshaper = "0.4"
```

---

## 🐛 KNOWN ISSUES & SOLUTIONS

### Issue 1: Arabic text in PDF appears disconnected

**Solution:** ✅ Fixed! Font embedding now includes proper Arabic ligatures.

### Issue 2: Images not showing in preview

**Solution:** Check that image paths are correct and files exist. Use absolute paths or paths relative to document.

### Issue 3: PDF export fails

**Cause:** Usually missing font file or printpdf dependency
**Solution:** Ensure `NotoSansArabic-Regular.ttf` is in `assets/fonts/`

### Issue 4: Large images slow down editor

**Solution:** Resize images before inserting, or use thumbnails

---

## 🔮 FUTURE ENHANCEMENTS

### Potential additions:

1. **Image resizing in editor** - Drag handles to resize
2. **Image compression** - Automatic optimization
3. **Cloud storage** - Google Drive / Dropbox integration
4. **Collaborative editing** - Real-time multi-user
5. **More themes** - Nord, Gruvbox, Solarized
6. **LaTeX support** - Math equations
7. **Tables** - Markdown-style tables
8. **Footnotes** - Academic writing support

---

## 📝 MIGRATION GUIDE

### From old PDX Editor to Enhanced:

**Breaking changes:**

- None! Fully backward compatible

**New features to try:**

1. Switch to Comfort theme
2. Insert an image
3. Export as real PDF
4. Export as PNG

**Recommended workflow:**

1. Write in Comfort theme (reduced eye strain)
2. Insert images with `![](path)`
3. Export to PDF for sharing
4. Export to PNG for social media

---

## 💡 TIPS & TRICKS

### Tip 1: Image Organization

Create an `images/` folder next to your .pdx file:

```
my-pdx-project/
  ├── document.pdx
  └── images/
      ├── photo1.jpg
      ├── photo2.png
      └── diagram.png
```

### Tip 2: Comfort Theme for Coding

Even for code blocks, Comfort theme reduces strain:

```rust
// This code is easier to read in Comfort theme
fn main() {
    println!("Hello, World!");
}
```

### Tip 3: Batch Image Insert

Prepare markdown first:

```markdown
![Image 1](img/1.png)
![Image 2](img/2.png)
![Image 3](img/3.png)
```

Then paste all at once.

### Tip 4: PDF Preview Before Export

Use Preview mode to see how PDF will look.

---

## ✅ SUMMARY

**What's new:**

1. ✅ Real PDF export with Arabic font embedding
2. ✅ PNG image export
3. ✅ Image embedding in documents
4. ✅ Comfort theme for long writing sessions

**What's improved:**

1. ✅ Better RTL text handling
2. ✅ Performance optimizations
3. ✅ More professional exports
4. ✅ Enhanced user experience

**What's the same:**

1. ✅ All original features still work
2. ✅ Same file format (.pdx)
3. ✅ Backward compatible
4. ✅ Same editor interface

---

## 🤝 CONTRIBUTING

Found a bug? Want a feature?

1. Open an issue
2. Submit a pull request
3. Join discussions

---

## 📄 LICENSE

GNU3 GENERAL PUBLIC LICENSE

---

**Made with ❤️ for writers who care about their eyes and their documents.**
