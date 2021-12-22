# RUSTFLAGS += -Clinker-plugin-lto

ifeq ($(BUILD_ENV), release)
CARGOFLAGS += --release
endif

ifeq ($(EXTERNAL_BINDINGS),true)
BUILD_DEPS += $(BINDINGS_DIR)/sizes $(BINDINGS_DIR)/libbindings.a
endif

rust-foo/librust-foo-rust.a: $(wildcard rust-foo/src/*.rs) rust-foo/Cargo.toml
	cd rust-foo && RUSTFLAGS="$(RUSTFLAGS)" cargo build $(CARGOFLAGS)
	cp rust-foo/target/$(BUILD_ENV)/librust_foo.a $@
CLEAN += rust-foo/librust-foo-rust.a


rust-foo/test: $(BUILD_DEPS)
	cd rust-foo && \
		EXTERNAL_LIB_PATH="../$(BINDINGS_DIR)" \
		EXTERNAL_LIB_NAME=bindings \
		SIZES_FILEPATH="../$(BINDINGS_DIR)/sizes" \
		cargo test $(CARGOFLAGS) -vv

CLEAN += rust-foo/target
