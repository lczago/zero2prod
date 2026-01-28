# sudo apt update && sudo apt install -y clang lld

# Tools and components
cargo install cargo-binutils
rustup component add llvm-tools-preview

# Create the local config correctly
mkdir -p ".cargo"
cat <<EOF > .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
EOF

# Useful quality-of-life tools
cargo install cargo-watch
cargo install cargo-llvm-cov
cargo install cargo-audit
rustup component add clippy
rustup component add rustfmt