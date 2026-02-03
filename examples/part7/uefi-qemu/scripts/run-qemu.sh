#!/bin/bash
# UEFI QEMU Launch Script
#
# This script launches QEMU with OVMF firmware for UEFI application testing.
#
# Prerequisites:
#   - qemu-system-x86_64
#   - OVMF firmware (usually in /usr/share/OVMF/)
#
# Usage: ./run-qemu.sh [options]
#   Options:
#     -d, --debug     Enable GDB debugging (pause at start)
#     -n, --nographic No graphical output (serial only)
#     -v, --verbose   Verbose QEMU output

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ESP_DIR="$PROJECT_DIR/esp"

# Default OVMF paths (adjust for your system)
OVMF_CODE="${OVMF_CODE:-/usr/share/OVMF/OVMF_CODE.fd}"
OVMF_VARS="${OVMF_VARS:-/usr/share/OVMF/OVMF_VARS.fd}"

# Alternative paths for different distributions
if [ ! -f "$OVMF_CODE" ]; then
    # Try edk2-ovmf path (Fedora/Arch)
    OVMF_CODE="/usr/share/edk2-ovmf/x64/OVMF_CODE.fd"
    OVMF_VARS="/usr/share/edk2-ovmf/x64/OVMF_VARS.fd"
fi

if [ ! -f "$OVMF_CODE" ]; then
    # Try another common path
    OVMF_CODE="/usr/share/qemu/OVMF.fd"
    OVMF_VARS=""
fi

# Parse arguments
DEBUG=""
NOGRAPHIC=""
VERBOSE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -d|--debug)
            DEBUG="-s -S"
            echo "Debug mode: QEMU will wait for GDB connection on localhost:1234"
            shift
            ;;
        -n|--nographic)
            NOGRAPHIC="-nographic"
            shift
            ;;
        -v|--verbose)
            VERBOSE="-d int,cpu_reset"
            shift
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Check for OVMF
if [ ! -f "$OVMF_CODE" ]; then
    echo "Error: OVMF firmware not found!"
    echo "Install with: sudo apt install ovmf"
    echo "Or: sudo dnf install edk2-ovmf"
    exit 1
fi

# Check for ESP directory
if [ ! -d "$ESP_DIR" ]; then
    echo "Error: ESP directory not found at $ESP_DIR"
    exit 1
fi

echo "Starting QEMU with UEFI..."
echo "  OVMF: $OVMF_CODE"
echo "  ESP:  $ESP_DIR"
echo ""

# Build QEMU command
QEMU_CMD="qemu-system-x86_64"

# OVMF firmware
if [ -n "$OVMF_VARS" ] && [ -f "$OVMF_VARS" ]; then
    # Separate CODE and VARS (recommended)
    QEMU_CMD="$QEMU_CMD \
        -drive if=pflash,format=raw,readonly=on,file=$OVMF_CODE \
        -drive if=pflash,format=raw,file=$OVMF_VARS"
else
    # Single OVMF file
    QEMU_CMD="$QEMU_CMD -bios $OVMF_CODE"
fi

# ESP as FAT drive
QEMU_CMD="$QEMU_CMD \
    -drive format=raw,file=fat:rw:$ESP_DIR"

# Machine configuration
QEMU_CMD="$QEMU_CMD \
    -machine q35 \
    -m 256M \
    -cpu qemu64"

# Serial output to stdio
QEMU_CMD="$QEMU_CMD \
    -serial stdio"

# Optional flags
QEMU_CMD="$QEMU_CMD $DEBUG $NOGRAPHIC $VERBOSE"

# Network (disabled for simplicity)
QEMU_CMD="$QEMU_CMD -net none"

echo "Command: $QEMU_CMD"
echo ""
echo "Press Ctrl+A, X to exit QEMU"
echo "---"

# Run QEMU
exec $QEMU_CMD
