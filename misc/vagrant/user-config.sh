#!/usr/bin/env sh

set -eu

export CARGO_HOME="${CARGO_HOME:-${HOME}/.cargo}"
export PATH="${PATH}:${CARGO_HOME}/bin"

command -v rustup || curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
rustup component add rustfmt
rustup component add clippy
rustup install beta
rustup install nightly
rustup update

cat << 'EOF' > "${HOME}/do"
set -eu

SRC_DEST="${HOME}/src"

cargo install cargo-vendor || true
sudo pbuilder update
rm -rf "$SRC_DEST" || true
mkdir -p "$SRC_DEST"
cp -r /src/ "$SRC_DEST"
command -v cargo-vendor || cargo install cargo-vendor
cd "$SRC_DEST"
pdebuild -us -uc

echo "Done!"
EOF

chmod +x "${HOME}/do"
