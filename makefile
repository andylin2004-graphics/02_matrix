all: build
	cargo run
	magick display bob.ppm&
	magick display rubensim.ppm&

build:
	cargo build