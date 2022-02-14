run:
	cargo run --release

clean:
	cargo clean
	rm *.ppm
	rm *.png

.PHONY: build run clean