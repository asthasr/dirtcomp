build:
    cargo build

release:
    cargo build --release

install PATH="~/.local/bin":  release
    cp target/release/dirtcomp {{ PATH }}
    
check: format
    cargo clippy

format:
    cargo fmt
