
all: readme

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@

test:
	cargo test

test-no_std:
	cargo test --no-default-features

clean:
	@cargo clean
	@rm -f z.*
