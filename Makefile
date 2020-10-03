SHELL=/bin/bash

build:
	time cargo build
	strip target/debug/fprompt
	ls -lh target/debug/fprompt

musl:
	sudo docker run --rm -v "$(shell pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release
	strip target/x86_64-unknown-linux-musl/release/fprompt
	ls -lh target/x86_64-unknown-linux-musl/release/fprompt
