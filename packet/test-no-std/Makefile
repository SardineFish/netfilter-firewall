
all: src/main.o target/debug/libtest_no_std.a
	cargo build
	clang -o target/debug/bin $^

run:
	target/debug/bin