run:
	cargo run --release $(ARGS)

unopt:
	cargo run $(ARGS)
	
clean:
	-cargo clean
	-rm *.ppm
	-rm *.png

.PHONY: build run clean