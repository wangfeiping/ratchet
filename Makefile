.PHONY: tests

GIT_TAG := $(shell git describe --tags --candidates 1)

# Builds the Lighthouse binary in release (optimized).
#
# Binaries will most likely be found in `./target/release`
install:
	cargo install --path ratchet --force --locked

