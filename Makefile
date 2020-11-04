export MODULE_NAME := kmod
export RUST_LIB_NAME := kmod_test

export KDIR ?= /lib/modules/$(shell uname -r)/build
export BASE_DIR := $(CURDIR)
export TARGET_DIR := $(CURDIR)/target/kernel

export RUST_TARGET_PLATFORM := x86_64-linux-kernel
export RUST_FLAGS := 
export RUST_MODE := debug
export RUST_TARGET_DIR := $(CURDIR)/target/$(RUST_TARGET_PLATFORM)/$(RUST_MODE)

export C_FILES := $(wildcard src/**/*.c)
export RUST_FILES := $(wildcard src/**/*.rs)


all:
	mkdir -p $(TARGET_DIR)
	cp Kbuild.mk $(TARGET_DIR)/Makefile
	make -C $(KDIR) M=$(TARGET_DIR) modules
	echo "\n\nBuild Complete!\n\n"

clean:
	make -C $(KDIR) M=$(TARGET_DIR) CC=$(CC) clean
	make -C $(KDIR) M=$(BASE_DIR)/src CC=$(CC) clean
	cargo clean

$(TARGET_DIR)/$(MODULE_NAME).ko: all

insmod: $(TARGET_DIR)/$(MODULE_NAME).ko
	insmod $(TARGET_DIR)/$(MODULE_NAME).ko

rmmod:
	rmmod $(MODULE_NAME)

install: insmod

uninstall: rmmod