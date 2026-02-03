#!/bin/bash
# Create a bootable UEFI disk image
#
# This script creates a FAT32 disk image with an EFI System Partition
# suitable for testing UEFI applications.
#
# Prerequisites:
#   - mtools (for mcopy, mformat)
#   - dosfstools (for mkfs.fat)
#
# Usage: ./create-disk.sh [output.img] [efi_file]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

OUTPUT="${1:-$PROJECT_DIR/uefi-disk.img}"
EFI_FILE="${2:-}"

DISK_SIZE="64M"

echo "Creating UEFI bootable disk image..."
echo "  Output: $OUTPUT"
echo "  Size: $DISK_SIZE"

# Create empty disk image
dd if=/dev/zero of="$OUTPUT" bs=1M count=64 status=progress

# Create GPT partition table and EFI System Partition
# Using parted for GPT support
if command -v parted &> /dev/null; then
    parted -s "$OUTPUT" mklabel gpt
    parted -s "$OUTPUT" mkpart ESP fat32 1MiB 100%
    parted -s "$OUTPUT" set 1 esp on
    echo "Created GPT partition table with ESP"
else
    echo "Warning: parted not found, creating simple FAT image"
fi

# Format as FAT32 using mtools
# First, create the FAT filesystem
if command -v mformat &> /dev/null; then
    # mtools approach (doesn't require root)
    mformat -i "$OUTPUT" -F ::

    # Create EFI directory structure
    mmd -i "$OUTPUT" ::EFI
    mmd -i "$OUTPUT" ::EFI/BOOT

    echo "Created FAT32 filesystem with EFI structure"

    # Copy EFI file if provided
    if [ -n "$EFI_FILE" ] && [ -f "$EFI_FILE" ]; then
        mcopy -i "$OUTPUT" "$EFI_FILE" ::EFI/BOOT/BOOTX64.EFI
        echo "Copied $EFI_FILE to EFI/BOOT/BOOTX64.EFI"
    fi
else
    echo "Warning: mtools not found"
    echo "Install with: sudo apt install mtools"
fi

echo ""
echo "Disk image created: $OUTPUT"
echo ""
echo "To add your UEFI application:"
echo "  mcopy -i $OUTPUT your_app.efi ::EFI/BOOT/BOOTX64.EFI"
echo ""
echo "To run with QEMU:"
echo "  qemu-system-x86_64 -bios /usr/share/OVMF/OVMF.fd -drive format=raw,file=$OUTPUT"
