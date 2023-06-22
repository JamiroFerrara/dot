install:
	cargo build
	sudo mv ./target/debug/dot /usr/local/bin/dot

reset:
	sudo rm -dfr /home/stiwie/.dotfiles/

view:
	sudo cat /home/stiwie/.dotfiles/config.json
