.PHONY: setup install uninstall test

setup install:
	./scripts/install-dev.sh

uninstall:
	./scripts/uninstall-dev.sh

test:
	cargo test -p typevn-core
