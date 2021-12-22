ifndef BUILD_ENV
	BUILD_ENV = debug
endif

ifeq ($(BUILD_ENV), debug)
# ok
else
ifeq ($(BUILD_ENV), release)
# ok
else
$(error Unknown BUILD_ENV=$(BUILD_ENV). Known values: debug, release; default: debug)
endif
endif

ifeq ($(BUILD_LANG),Rust)
EXTERNAL_BINDINGS = false
else
ifeq ($(BUILD_LANG),C)
include c-bindings/build.mk
else
ifeq ($(BUILD_LANG),C++)
include cpp-bindings/build.mk
else
$(error Unknown BUILD_LANG=$(BUILD_LANG). Known values: C, C++)
endif
endif
endif

include rust-foo/build.mk

clean:
	rm -rf $(CLEAN)
