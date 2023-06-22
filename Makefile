install:
	cargo build
	sudo mv ./target/debug/dot /usr/bin/dot
