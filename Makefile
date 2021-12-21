ifndef BUILD_ENV
	BUILD_ENV = debug
endif

ifeq ($(BUILD_ENV), debug)
# ok
else
ifeq ($(BUILD_ENV), release)
# ok
else
$(error Unknown BUILD_ENV=$(BUILD_ENV); known values: debug, release; default: debug)
endif
endif

include rust-foo/build.mk
include c-bindings/build.mk
include cpp-bindings/build.mk

clean:
	rm -rf $(CLEAN)


# EXTERNAL_LIB_PATH="../c-bindings" \
#   EXTERNAL_LIB_NAME="bindings" \
#   SIZES_FILEPATH="../c-bindings/sizes" \
#   CARGOFLAGS="--features=external" \
#   make rust-foo/test

# LINK_WITH_CXX_RUNTIME=1 \
#   EXTERNAL_LIB_PATH="../cpp-bindings" \
#   EXTERNAL_LIB_NAME="bindings" \
#   SIZES_FILEPATH="../cpp-bindings/sizes" \
#   CARGOFLAGS="--features=external" \
#   make rust-foo/test
