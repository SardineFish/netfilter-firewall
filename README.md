# Linux kernel module with Rust

A tiny example of building linux kernel modules with C & Rust.

Tested in ubuntu 20.04.1 LTS with linux kernel 5.4.0-29-generic.

## Build

```shell
$ make
```

The module will build to `target/kernel/kmod.ko`

## Install module

```shell
$ sudo make insmod
```

After installation, use following command to see the output from kernel module.

```shell
$ sudo dmesg
```

You will see kernel messages like this:
```
[40772.669088] Load kmod-test.
[40772.669089] gcd(48, 64) = 16, called from C & calculated from Rust.
```

## Uninstall module

```shell
$ sudo make rmmod
```