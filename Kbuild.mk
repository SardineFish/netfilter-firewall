
rust-target := lib$(RUST_LIB_NAME).a
rust-obj := lib$(RUST_LIB_NAME).o

c-objs := $(foreach filename, $(C_FILES), $(shell realpath --relative-to $(M) $(filename)))
c-objs := $(patsubst %.c, %.o, ${c-objs})

obj-m := $(MODULE_NAME).o
$(MODULE_NAME)-objs := $(c-objs) $(rust-obj)

$(M)/${rust-obj}: $(RUST_TARGET_DIR)/${rust-target}
	echo $(c-objs)
	ld -r -o $@ --whole-archive $^

$(RUST_TARGET_DIR)/${rust-target}: ${RUST_FILES} $(RURST_GEN_FILES)
	cd $(BASE_DIR) && \
	cargo +nightly build --target x86_64-linux-kernel -Zbuild-std=core