PROJECT := rshell
PROJECT_DIR := $(shell pwd)
BIN_DIR := bin
SRC_DIR := src

SRCS := $(wildcard $(SRC_DIR)/*.rs)

$(PROJECT): $(SRCS)
	cargo build --release
	mkdir -p $(PROJECT_DIR)/$(BIN_DIR)
	cp $(PROJECT_DIR)/target/release/$(PROJECT) $(PROJECT_DIR)/$(BIN_DIR)/$(PROJECT)

clean:
	rm -Rf $(PROJECT_DIR)/$(BIN_DIR)
	cargo clean

.PHONY: clean
