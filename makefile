all: build
	cargo run
	magick display bob.ppm&

build:
	cargo build