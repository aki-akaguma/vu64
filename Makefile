
all: readme

clean:
	@cargo clean
	@rm -f z.*

clippy:
	cargo clippy --offline --tests --workspace

cov:
	cargo llvm-cov
	cargo llvm-cov --output-dir target/llvm-cov --text report
	cargo llvm-cov --output-dir target/llvm-cov --html report

doc:
	cargo doc

fmt:
	cargo fmt

test:
	cargo test --offline

test-no-default-features:
	cargo test --offline --no-default-features

miri:
	cargo +nightly miri test --offline

tarpaulin:
	cargo tarpaulin --offline --engine llvm --out html --output-dir ./target

readme: README.md

README.md: README.tpl src/lib.rs
	cargo readme > $@


rustc_vers = 1.56.1 1.57.0 1.58.1 1.59.0 1.60.0 1.61.0 1.62.1 1.63.0 \
	1.64.0 1.65.0 1.66.1
target_base = x86_64-unknown-linux-gnu i586-unknown-linux-gnu

define test-rustc-templ =
target/stamp/stamp.test-rustc.$(1).$(2):
	@mkdir -p target/stamp
	cargo +$(1) test --target=$(2)
	@touch target/stamp/stamp.test-rustc.$(1).$(2)
endef

test-all-version: $(foreach ver,$(rustc_vers),$(foreach tb,$(target_base),target/stamp/stamp.test-rustc.$(ver).$(tb)))

test-clean:
	@rm -fr target/stamp

$(foreach ver,$(rustc_vers),$(eval $(foreach tb,$(target_base),$(eval $(call test-rustc-templ,$(ver),$(tb))))))
