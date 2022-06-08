INSTALL_DIR=$(HOME)/.local/bin

target:
	cargo build --release

install: target
	mkdir -p $(INSTALL_DIR)
	mv target/release/basedname $(INSTALL_DIR)

link: target
	mkdir -p $(INSTALL_DIR)
	ln -vsf $(PWD)/target/release/basedname $(INSTALL_DIR)
