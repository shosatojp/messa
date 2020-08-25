SHELL=/bin/bash

build:
	time cargo run -- --home /home/sho --pwd /home/sho/repos/powerline-shell/ --error 2
	strip target/debug/pl
	ls -lh target/debug/pl

run:
	time target/debug/pl --home /home/sho --pwd /home/sho/repos/powerline-shell/