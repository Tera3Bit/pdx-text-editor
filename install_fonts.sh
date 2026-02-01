#!/bin/bash
echo "========================================="
echo "PDX Editor - Arabic Font Installer"
echo "========================================="
echo ""

# Create fonts directory
echo "📁 Creating fonts directory..."
mkdir -p assets/fonts

# Download Noto Sans Arabic
echo "⬇️  Downloading Noto Sans Arabic font..."

# Try multiple sources
if command -v curl &> /dev/null; then
    # Try Google Fonts API
    curl -L "https://github.com/google/fonts/raw/main/ofl/notosansarabic/NotoSansArabic%5Bwdth%2Cwght%5D.ttf" \
         -o assets/fonts/NotoSansArabic-Regular.ttf 2>/dev/null || \
    # Fallback to direct download
    curl -L "https://noto-website-2.storage.googleapis.com/pkgs/NotoSansArabic-unhinted.zip" \
         -o /tmp/arabic-font.zip 2>/dev/null && \
    unzip -q /tmp/arabic-font.zip "NotoSansArabic-Regular.ttf" -d assets/fonts/ && \
    rm /tmp/arabic-font.zip
elif command -v wget &> /dev/null; then
    wget -q "https://github.com/google/fonts/raw/main/ofl/notosansarabic/NotoSansArabic%5Bwdth%2Cwght%5D.ttf" \
         -O assets/fonts/NotoSansArabic-Regular.ttf 2>/dev/null || \
    wget -q "https://noto-website-2.storage.googleapis.com/pkgs/NotoSansArabic-unhinted.zip" \
         -O /tmp/arabic-font.zip && \
    unzip -q /tmp/arabic-font.zip "NotoSansArabic-Regular.ttf" -d assets/fonts/ && \
    rm /tmp/arabic-font.zip
else
    echo "❌ Error: Neither curl nor wget found!"
    echo "Please install curl or wget and try again."
    echo ""
    echo "Or download manually from:"
    echo "https://fonts.google.com/noto/specimen/Noto+Sans+Arabic"
    exit 1
fi

# Check if download succeeded
if [ -f "assets/fonts/NotoSansArabic-Regular.ttf" ]; then
    echo "✅ Font downloaded successfully!"
    echo ""
    echo "📝 Next steps:"
    echo "1. Open main.rs"
    echo "2. Find the setup_fonts() function"
    echo "3. Uncomment the Arabic font code (remove /* and */)"
    echo "4. Run: cargo build --release"
    echo "5. Run: cargo run --release"
    echo ""
    echo "🎉 Arabic support will be fully enabled!"
else
    echo "❌ Download failed!"
    echo ""
    echo "📥 Manual download required:"
    echo "1. Visit: https://fonts.google.com/noto/specimen/Noto+Sans+Arabic"
    echo "2. Click 'Download family'"
    echo "3. Extract NotoSansArabic-Regular.ttf"
    echo "4. Place it in: assets/fonts/"
    echo ""
    echo "Then run:"
    echo "  cargo build --release"
    echo "  cargo run --release"
fi

echo ""
echo "========================================="