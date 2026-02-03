#!/bin/bash
# UEFI QEMU Debugging Script
#
# This script helps set up GDB debugging for UEFI applications running in QEMU.
#
# Usage:
#   1. Start QEMU with debug flag: ./run-qemu.sh --debug
#   2. In another terminal: ./debug-qemu.sh [path/to/your.efi]
#
# The script will:
#   - Connect GDB to QEMU's GDB server
#   - Load debug symbols if EFI file is provided
#   - Set up useful breakpoints

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EFI_FILE="${1:-}"

GDB="${GDB:-gdb}"
GDB_PORT="${GDB_PORT:-1234}"
GDB_HOST="${GDB_HOST:-localhost}"

# Check for GDB
if ! command -v "$GDB" &> /dev/null; then
    echo "Error: GDB not found"
    echo "Install with: sudo apt install gdb"
    exit 1
fi

# Create GDB commands file
GDB_COMMANDS=$(mktemp)
trap "rm -f $GDB_COMMANDS" EXIT

cat > "$GDB_COMMANDS" << 'EOF'
# UEFI GDB Setup

# Connect to QEMU
target remote localhost:1234

# Set architecture
set architecture i386:x86-64

# Useful display settings
set disassembly-flavor intel
set print pretty on

# Don't stop on signals commonly used by UEFI
handle SIGSEGV nostop noprint

# Info commands
define uefi-info
    echo \n=== UEFI Debug Info ===\n
    info registers rip rsp rbp
    echo \nStack:\n
    x/16xg $rsp
    echo \nCode:\n
    x/10i $rip
end

# Memory map helper
define uefi-mem
    echo \n=== Memory around address ===\n
    x/32xb $arg0
end

echo \n
echo UEFI GDB Session Started
echo ========================
echo Commands:
echo   c          - Continue execution
echo   si         - Step one instruction
echo   ni         - Next instruction (skip calls)
echo   bt         - Backtrace
echo   info reg   - Show registers
echo   uefi-info  - Show UEFI debug info
echo   uefi-mem   - Show memory at address
echo \n

EOF

# Add symbol loading if EFI file provided
if [ -n "$EFI_FILE" ] && [ -f "$EFI_FILE" ]; then
    echo "# Load symbols from $EFI_FILE" >> "$GDB_COMMANDS"
    echo "# Note: UEFI apps are loaded at runtime addresses" >> "$GDB_COMMANDS"
    echo "# You may need to use 'add-symbol-file' with correct load address" >> "$GDB_COMMANDS"
    echo "" >> "$GDB_COMMANDS"

    echo "Loading symbols from: $EFI_FILE"
fi

echo "Starting GDB session..."
echo "  Target: $GDB_HOST:$GDB_PORT"
echo ""
echo "Make sure QEMU is running with: ./run-qemu.sh --debug"
echo ""

# Start GDB
"$GDB" -x "$GDB_COMMANDS"
