test:
	cargo test --lib

doc:
	cargo doc --open --no-deps

.PHONEY: test doc