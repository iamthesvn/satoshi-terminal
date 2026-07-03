#!/usr/bin/env bash
# Satoshi's Terminal installer — builds and installs the `satoshi-terminal` binary to ~/.cargo/bin/
# After running this, type: satoshi-terminal

set -e

echo ""
echo "  ███████╗ █████╗ ████████╗ ██████╗ ███████╗██╗  ██╗██╗      ██████╗ ███████╗"
echo "  ██╔════╝██╔══██╗╚══██╔══╝██╔═══██╗██╔════╝██║  ██║██║     ██╔═══██╗██╔════╝"
echo "  ███████╗███████║   ██║   ██║   ██║███████╗███████║██║     ██║   ██║█████╗  "
echo "  ╚════██║██╔══██║   ██║   ██║   ██║╚════██║██╔══██║██║     ██║   ██║██╔══╝  "
echo "  ███████║██║  ██║   ██║   ╚██████╔╝███████║██║  ██║███████╗╚██████╔╝██║     "
echo "  ╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═╝     "
echo "                                  ████████╗███████╗██████╗ ███╗   ███╗██╗███╗   ██║"
echo "                                  ╚══██╔══╝██╔════╝██╔══██╗████╗ ████║██║████╗  ██║"
echo "                                     ██║   █████╗  ██████╔╝██╔████╔██║██║██╔██╗ ██║"
echo "                                     ██║   ██╔══╝  ██╔══██╗██║╚██╔╝██║██║██║╚██╗██║"
echo "                                     ██║   ███████╗██║  ██║██║ ╚═╝ ██║██║██║ ╚████║"
echo "                                     ╚═╝   ╚══════╝╚═╝  ╚═╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝"
echo ""
echo "  Installing Satoshi's Terminal"
echo ""

# Check for Rust/Cargo
if ! command -v cargo &>/dev/null; then
    echo "  ERROR: Rust/Cargo not found."
    echo "  Install from: https://rustup.rs"
    exit 1
fi

echo "  Building release binary..."
cargo install --path . --quiet

echo ""
echo "  ✓ Installation complete!"
echo ""
echo "  Run the game with:"
echo ""
echo "    satoshi-terminal"
echo ""
echo "  The binary is at: $(which satoshi-terminal 2>/dev/null || echo ~/.cargo/bin/satoshi-terminal)"
echo ""
echo "  If 'satoshi-terminal' is not found, add ~/.cargo/bin to your PATH:"
echo "    export PATH=\"\$HOME/.cargo/bin:\$PATH\""
echo ""
