@echo off
title PDX Editor Builder

:menu
cls
echo ==============================
echo        PDX BUILD TOOL
echo ==============================
echo.
echo Choose Target OS:
echo.
echo 1^) Windows
echo 2^) Linux
echo 3^) macOS
echo 4^) Exit
echo.

set /p os=Enter choice:

if "%os%"=="1" goto windows
if "%os%"=="2" goto linux
if "%os%"=="3" goto mac
if "%os%"=="4" exit

goto menu

:: ---------------- WINDOWS ----------------

:windows
cls
echo.
echo Choose Windows Format:
echo.
echo 1^) NSIS Installer
echo 2^) Portable Binary
echo.

set /p format=Enter choice:

if "%format%"=="1" (
echo Building Windows NSIS Installer...
cargo packager --release --formats nsis
goto done
)

if "%format%"=="2" (
echo Building Portable Release...
cargo build --release
goto done
)

goto windows

:: ---------------- LINUX ----------------

:linux
cls
echo.
echo Choose Linux Format:
echo.
echo 1^) DEB
echo 2^) AppImage
echo.

set /p format=Enter choice:

if "%format%"=="1" (
cargo packager --release --formats deb
goto done
)

if "%format%"=="2" (
cargo packager --release --formats appimage
goto done
)

goto linux

:: ---------------- MAC ----------------

:mac
cls
echo.
echo Building macOS DMG...
cargo packager --release --formats dmg
goto done

:done
echo.
echo ==============================
echo        BUILD FINISHED
echo ==============================
pause
