@echo off
REM PDX Editor - Arabic Font Installer for Windows

echo =========================================
echo PDX Editor - Arabic Font Installer
echo =========================================
echo.

REM Create fonts directory
echo Creating fonts directory...
if not exist "assets\fonts" mkdir "assets\fonts"

echo.
echo Downloading Noto Sans Arabic font...
echo.

REM Try to download using PowerShell
powershell -Command "& {Invoke-WebRequest -Uri 'https://github.com/google/fonts/raw/main/ofl/notosansarabic/static/NotoSansArabic-Regular.ttf' -OutFile 'assets\fonts\NotoSansArabic-Regular.ttf'}" 2>nul

if exist "assets\fonts\NotoSansArabic-Regular.ttf" (
    echo ✅ Font downloaded successfully!
    echo.
    echo Next steps:
    echo 1. Open main.rs
    echo 2. Find the setup_fonts^(^) function
    echo 3. Uncomment the Arabic font code ^(remove /* and */^)
    echo 4. Run: cargo build --release
    echo 5. Run: cargo run --release
    echo.
    echo Arabic support will be fully enabled!
) else (
    echo ❌ Download failed!
    echo.
    echo Manual download required:
    echo 1. Visit: https://fonts.google.com/noto/specimen/Noto+Sans+Arabic
    echo 2. Click 'Download family'
    echo 3. Extract NotoSansArabic-Regular.ttf
    echo 4. Place it in: assets\fonts\
    echo.
    echo Then run:
    echo   cargo build --release
    echo   cargo run --release
)

echo.
echo =========================================
pause