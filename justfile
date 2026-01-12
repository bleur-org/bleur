build:
    cargo build --release

format:
    cargo fmt

lint:
    cargo clippy

try:
    rm -rf ./test
    mkdir -p ./test
    cd test && cargo run -- new

clean:
    rm -rf ./.direnv
    rm -rf ./target
    rm -rf ./result
    rm -rf ./test
