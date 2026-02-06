#!/bin/bash
# Build all Rust examples using Docker
# Usage: ./build-all.sh [--local] [--warnings]
#   --local:    Build without Docker (uses local Rust installation)
#   --warnings: Also report warnings (not just errors)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Parse arguments
USE_DOCKER=true
SHOW_WARNINGS=false
for arg in "$@"; do
    case $arg in
        --local)
            USE_DOCKER=false
            ;;
        --warnings)
            SHOW_WARNINGS=true
            ;;
    esac
done

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

if $USE_DOCKER; then
    echo "Running with Docker (single container session)..."

    # Check if Docker is available
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}Error: Docker is not installed or not in PATH${NC}"
        echo "Install Docker or run with --local flag to use local Rust"
        exit 1
    fi

    # Build all examples in a single Docker container
    docker run --rm -v "$SCRIPT_DIR:/workspace" -w /workspace rust:latest bash -c "
        echo '========================================'
        echo 'Building all Rust examples'
        echo '========================================'
        echo ''

        PASSED=0
        FAILED=0
        FAILED_LIST=''
        WARN_LIST=''

        for dir in \$(find . -name 'Cargo.toml' -not -path './target/*' -exec dirname {} \; | sort); do
            if [ \"\$dir\" == '.' ]; then continue; fi

            echo -n \"Building \$dir... \"

            OUTPUT=\$(cargo build --release --manifest-path \"\$dir/Cargo.toml\" 2>&1)
            EXIT_CODE=\$?

            if [ \$EXIT_CODE -eq 0 ]; then
                if echo \"\$OUTPUT\" | grep -q 'warning:'; then
                    echo -e '\033[0;32mOK\033[0m \033[1;33m(warnings)\033[0m'
                    WARN_LIST=\"\$WARN_LIST \$dir\"
                else
                    echo -e '\033[0;32mOK\033[0m'
                fi
                PASSED=\$((PASSED + 1))
            else
                echo -e '\033[0;31mFAILED\033[0m'
                echo \"\$OUTPUT\" | head -30
                FAILED=\$((FAILED + 1))
                FAILED_LIST=\"\$FAILED_LIST \$dir\"
                echo ''
            fi
        done

        echo ''
        echo '========================================'
        echo 'Build Summary'
        echo '========================================'
        echo -e \"\033[0;32mPassed: \$PASSED\033[0m\"
        echo -e \"\033[0;31mFailed: \$FAILED\033[0m\"

        if [ -n \"\$FAILED_LIST\" ]; then
            echo ''
            echo 'Failed examples:'
            for name in \$FAILED_LIST; do
                echo -e \"  \033[0;31m- \$name\033[0m\"
            done
            exit 1
        else
            echo ''
            echo -e '\033[0;32mAll examples built successfully!\033[0m'

            if [ -n \"\$WARN_LIST\" ] && [ '$SHOW_WARNINGS' == 'true' ]; then
                echo ''
                echo -e '\033[1;33mExamples with warnings:\033[0m'
                for name in \$WARN_LIST; do
                    echo \"  - \$name\"
                done
            fi
        fi
    "
else
    echo "Running with local Rust installation..."

    # Check if cargo is available
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo is not installed or not in PATH${NC}"
        echo "Install Rust or remove --local flag to use Docker"
        exit 1
    fi

    echo "========================================"
    echo "Building all Rust examples"
    echo "========================================"
    echo ""

    PASSED=0
    FAILED=0
    FAILED_LIST=""

    for cargo_file in $(find . -name "Cargo.toml" -not -path "./target/*" | sort); do
        dir=$(dirname "$cargo_file")

        if [[ "$dir" == "." ]]; then
            continue
        fi

        echo -n "Building $dir... "

        if OUTPUT=$(cargo build --release --manifest-path "$dir/Cargo.toml" 2>&1); then
            if echo "$OUTPUT" | grep -q "warning:"; then
                echo -e "${GREEN}OK${NC} ${YELLOW}(warnings)${NC}"
            else
                echo -e "${GREEN}OK${NC}"
            fi
            PASSED=$((PASSED + 1))
        else
            echo -e "${RED}FAILED${NC}"
            echo "$OUTPUT" | head -30
            FAILED=$((FAILED + 1))
            FAILED_LIST="$FAILED_LIST $dir"
            echo ""
        fi
    done

    echo ""
    echo "========================================"
    echo "Build Summary"
    echo "========================================"
    echo -e "${GREEN}Passed: $PASSED${NC}"
    echo -e "${RED}Failed: $FAILED${NC}"

    if [[ -n "$FAILED_LIST" ]]; then
        echo ""
        echo "Failed examples:"
        for name in $FAILED_LIST; do
            echo -e "  ${RED}- $name${NC}"
        done
        exit 1
    else
        echo ""
        echo -e "${GREEN}All examples built successfully!${NC}"
    fi
fi
