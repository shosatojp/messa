SHELL=/bin/bash

build:
	time cargo build
	strip target/debug/pl
	ls -lh target/debug/pl

musl:
	sudo docker run --rm -it -v "$(shell pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release
	strip target/x86_64-unknown-linux-musl/release/pl
	ls -lh target/x86_64-unknown-linux-musl/release/pl

run:
	target/debug/pl --home /home/sho --pwd /home/sho/repos/powerline-shell/