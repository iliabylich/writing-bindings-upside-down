# RUSTFLAGS += -Clinker-plugin-lto

ifeq ($(BUILD_ENV), release)
CARGOFLAGS += --release
endif

rust-foo/librust-foo-rust.a: $(wildcard rust-foo/src/*.rs) rust-foo/Cargo.toml
	cd rust-foo && RUSTFLAGS="$(RUSTFLAGS)" cargo build $(CARGOFLAGS)
	cp rust-foo/target/$(BUILD_ENV)/librust_foo.a $@
CLEAN += rust-foo/librust-foo-rust.a

rust-foo/test:
	cd rust-foo && cargo test $(CARGOFLAGS) -vv

CLEAN += rust-foo/target
