test:
	cargo test --lib

doc:
	cargo doc --open --no-deps

doc-build:
	cargo doc --no-deps
	rm -rf ./docs
	echo "<meta http-equiv=\"refresh\" content=\"0; url=algorithm/index.html\">" > target/doc/index.html
	cp -r target/doc ./docs
.PHONEY: test doc