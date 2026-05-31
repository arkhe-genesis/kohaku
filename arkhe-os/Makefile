# ARKHE OS Build System
# Substrato 996: ARKHE-OS

BOOT_DIR = boot
KERNEL_DIR = kernel
SERVERS_DIR = servers
LIBS_DIR = libs
TOOLS_DIR = tools
BUILD_DIR = build

.PHONY: all clean kernel servers tools libs bootloader

all: $(BUILD_DIR) bootloader kernel servers libs tools

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)

bootloader:
	# Mock bootloader build step

kernel:
	cd $(KERNEL_DIR) && cargo check

servers:
	cargo check --manifest-path Cargo.toml -p arkhe-vfs -p arkhe-net -p arkhe-passport -p arkhe-orchestrator -p arkhe-bindu

libs:
	cargo check --manifest-path Cargo.toml -p arklib -p pqc -p nostr

tools:
	cargo check --manifest-path Cargo.toml -p arkhe-sh -p pkg -p checkpoint

clean:
	rm -rf $(BUILD_DIR)
	cargo clean --manifest-path Cargo.toml
