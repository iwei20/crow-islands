run:
	cargo run --release
	convert result.ppm result.png
	rm result.ppm
	display result.png

clean:
	cargo clean
	rm *.ppm
	rm *.png

.PHONY: build run clean