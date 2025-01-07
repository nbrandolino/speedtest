TARGET = target/release/speedtest
BIN = speedtest
DESTDIR = /usr/bin/

all:
	@echo "Makefile commands:"
	@echo "  make build      - Build the project"
	@echo "  make run        - Run the project"
	@echo "  make clean      - Clean the build artifacts"
	@echo "  make install    - Install the binary"
	@echo "  make uninstall  - Uninstall the binary"
build:
	@cargo build --release
run:
	$(TARGET)
clean:
	@cargo clean
	@ rm -rf Cargo.lock
install:
	@cp -p $(TARGET) $(DESTDIR)$(BIN)
uninstall:
	@rm -rf $(DESTDIR)$(TARGET)
