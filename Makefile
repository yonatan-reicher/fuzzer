build:
	@echo "Building..."
	cargo build --release
	@echo "Done."
	cp target/release/fuzzer .
