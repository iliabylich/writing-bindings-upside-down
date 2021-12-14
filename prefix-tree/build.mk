# RUSTFLAGS += -Clinker-plugin-lto

CARGOFLAGS += --features=external

ifeq ($(BUILD_ENV), release)
CARGOFLAGS += --release
endif

prefix-tree/libprefix-tree-rust.a: $(wildcard prefix-tree/src/*.rs) prefix-tree/Cargo.toml
	cd prefix-tree && RUSTFLAGS="$(RUSTFLAGS)" cargo build $(CARGOFLAGS)
	cp prefix-tree/target/$(BUILD_ENV)/libprefix_tree.a $@
