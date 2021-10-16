TARGET:=x86_64-unknown-linux-musl
BIN:=fprompt
PROFILE:=release
BIN_PATH:=target/$(TARGET)/$(PROFILE)/$(BIN)
RUST_MUSL_BUILDER:=docker run -v $(shell pwd):/home/rust/src ekidd/rust-musl-builder
VERSION=$(shell cat Cargo.toml | grep ^version | sed -E 's/^version = "([0-9.]+)"/\1/')
ARTIFACTS_DIR:=artifacts

build:
	$(RUST_MUSL_BUILDER) cargo build --target $(TARGET) --$(PROFILE)
	strip $(BIN_PATH)
	-ldd $(BIN_PATH)
	ls -lh $(BIN_PATH)

test:
	$(RUST_MUSL_BUILDER) cargo test

release:
	gh release create "v$(VERSION)" $(BIN_PATH)
