# Stateful Firewall with Netfilter

A stateful firewall with Netfilter by rust.

Including a linux kernel module and a client.

Currently avalable on ubuntu 20.04.1 LTS with linux kernel 5.4.0-29-generic.

Not fully tested for other linux systems.

Programming task for HUST *Cyber Security Course Project*.

## Features

Base on stateful-inspection, filter IPv4 packets by fast connection state check (hash map).

Manipulate the firewall rules by a user-space cli program: 
- Manipulate firewall rules separately for TCP, UDP and ICMP Echo.
- Allow / Deny network communication from / to specific address & port.
- Set default rule for each protocol.
- List all current active rules.

### Supported Protocol
- TCP
- UDP
- ICMP Echo / Reply (ping)



## Kernel Module

### Build Module

```shell
$ make
```

The module will build to `target/kernel/kmod.ko`

### Install Module

```shell
$ sudo make insmod
```

The firewall will start working immediately after installation. Use following command to view the log.

```shell
$ sudo dmesg
```

### Uninstall Module

```shell
$ sudo make rmmod
```

## Client

### Build Client
```shell
$ cd client
$ cargo build --release
```

### Run Client

#### Examples
```shell
# Allow UDP packet send to 8.8.8.8:53
$ ./target/release/client allow UDP 0.0.0.0 8.8.8.8:53

# Allow TCP connection into 80 port from subnet 192.168.1.0/24
$ ./target/release/client allow TCP 192.168.1.0/24 0.0.0.0:80

# Deny all UDP packet by default
$ ./target/release/client deny UDP default

# List all currently active rules
$ ./target/release/client list

# Delete the rule at index of 7
$ ./target/release/client delete 7

```