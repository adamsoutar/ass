# Only run ass without then invoking gcc
# Better for parser debugging
RUST_BACKTRACE=1 cargo run input.c
