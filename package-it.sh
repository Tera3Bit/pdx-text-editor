#!/bin/bash

clear
echo "=============================="
echo "       PDX BUILD TOOL"
echo "=============================="
echo
echo "Choose Target OS:"
echo
echo "1) Windows"
echo "2) Linux"
echo "3) macOS"
echo "4) Exit"
echo

read -p "Enter choice: " os

case $os in
    1)
        clear
        echo
        echo "Choose Windows Format:"
        echo
        echo "1) NSIS Installer"
        echo "2) Portable Binary"
        echo

        read -p "Enter choice: " format

        if [ "$format" == "1" ]; then
            echo "Building Windows NSIS Installer..."
            cargo packager --release --formats nsis

        elif [ "$format" == "2" ]; then
            echo "Building Portable Release..."
            cargo build --release
        else
            echo "Invalid option."
            exit 1
        fi
        ;;

    2)
        clear
        echo
        echo "Choose Linux Format:"
        echo
        echo "1) DEB"
        echo "2) AppImage"
        echo

        read -p "Enter choice: " format

        if [ "$format" == "1" ]; then
            cargo packager --release --formats deb

        elif [ "$format" == "2" ]; then
            cargo packager --release --formats appimage
        else
            echo "Invalid option."
            exit 1
        fi
        ;;

    3)
        clear
        echo
        echo "Building macOS DMG..."
        cargo packager --release --formats dmg
        ;;

    4)
        exit 0
        ;;

    *)
        echo "Invalid option."
        exit 1
        ;;
esac

echo
echo "=============================="
echo "       BUILD FINISHED"
echo "=============================="
read -p "Press Enter to continue..."
