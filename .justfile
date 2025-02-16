setup:
    cd ./commit-msg && chmod +x setup_hooks.sh
    cd ./commit-msg && ./setup_hooks.sh

build:
    cargo build --release

clean:
    cargo clean

run:
    cargo run --example simple

actix:
    cargo run --example actix

check:
    cargo clippy

verify:
    cargo publish --dry-run
