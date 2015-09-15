PREFIX ?= /usr/local
FILENAME = $(PREFIX)/bin/lit

.PHONY: build
build:
	cargo build --release

.PHONY: clean
clean:
	cargo clean

.PHONY: install
install:
	cp target/release/lit "$(FILENAME)"
	chmod 4755 "$(FILENAME)"

.PHONY: uninstall
uninstall:
	rm "$(FILENAME)"
