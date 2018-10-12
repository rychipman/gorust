
rust:
	cd util && cargo build

build: rust
	go build .

run: build
	./gorust

clean:
	rm -f gorust; \
	rm -rf util/target
