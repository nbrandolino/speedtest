CARGO := cargo
TARGET := target/release/speedtest
DESTDIR = /usr/bin/

all: release
release:
	$(CARGO) build --release
run: release
	$(TARGET)
install: release
	install -Dm755 $(TARGET) $(DESTDIR)/speedtest
clean:
	$(CARGO) clean
format:
	$(CARGO) fmt
lint:
	$(CARGO) clippy -- -D warnings
test:
	$(CARGO) test

.PHONY: all release run install clean format lint test
