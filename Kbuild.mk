
rust-target := lib$(RUST_LIB_NAME).a
rust-obj := lib$(RUST_LIB_NAME).o

c-objs := $(patsubst %.c, %.o, ${C_FILES})
c-objs := $(foreach filename, $(c-objs), $(shell realpath --relative-to $(M) $(BASE_DIR)/$(filename)))

obj-m := $(MODULE_NAME).o
$(MODULE_NAME)-objs := $(c-objs) $(rust-obj)


$(M)/${rust-obj}: $(RUST_TARGET_DIR)/${rust-target}
	ld -r -o $@ --whole-archive $^

$(RUST_TARGET_DIR)/${rust-target}: $(RUST_FILES)
	cd $(BASE_DIR) && \
	cargo +nightly build --target x86_64-linux-kernel -Zbuild-std=core