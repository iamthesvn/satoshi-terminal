#!/usr/bin/env bash
# GitQuest installer — builds and installs the `gitquest` binary to ~/.cargo/bin/
# After running this, type: gitquest

set -e

echo ""
echo "  ██████╗ ██╗████████╗"
echo "  ██╔════╝ ██║╚══██╔══╝"
echo "  ██║  ███╗██║   ██║   "
echo "  ██║   ██║██║   ██║   "
echo "  ╚██████╔╝██║   ██║   "
echo "   ╚═════╝ ╚═╝   ╚═╝   "
echo ""
echo "  ██████╗ ██╗   ██╗███████╗███████╗████████╗"
echo " ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝"
echo " ██║   ██║██║   ██║█████╗  ███████╗   ██║   "
echo " ██║   ██║██║   ██║██╔══╝  ╚════██║   ██║   "
echo " ╚██████╔╝╚██████╔╝███████╗███████║   ██║   "
echo "  ╚═════╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝   "
echo ""
echo "  Installing GitQuest — The Codewright Chronicles"
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
echo "    gitquest"
echo ""
echo "  The binary is at: $(which gitquest 2>/dev/null || echo ~/.cargo/bin/gitquest)"
echo ""
echo "  If 'gitquest' is not found, add ~/.cargo/bin to your PATH:"
echo "    export PATH=\"\$HOME/.cargo/bin:\$PATH\""
echo ""
