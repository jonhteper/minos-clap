all: prepare build

prepare: 
	mkdir package || true
	cp -r example-policies package/.policies


build: 
	cargo build --release
	cp ../target/release/minos-cli-test package/minos-test