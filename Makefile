RUSTC ?= rustc

all:
	$(RUSTC) deadlock.rs
clean:
	rm -rf deadlock
