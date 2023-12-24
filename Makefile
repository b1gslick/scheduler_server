export RUST_LOG=debug

default:
	cargo test && \
	cargo run
