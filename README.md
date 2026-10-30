# esp-idf.embassy.min

A sample repo for:

- starting an ESP-IDF Rust project 
- ..that also uses Embassy `async`.

and/or

- Studying how the footprint of `esp-idf` (the Rust libraries) can be reduced, making components opt-in rather than opt-out.

	>The `.min` in the name reflects the latter aim. But you can use the repo as a simple getting-started template simply by commenting out `ESP_IDF_COMPONENTS` section in the `.cargo/config.toml`.


## Used by

Loosely connected repos:

- 🚨[SLED](https://github.com/finalyards/SLED)

	Use of ESP32-C6 devkit's LED as a status indicator.
	
- ⚡️[ezb-node](https://github.com/finalyards-org/ezb-node)

	Limited Zigbee support, in Rust, for ESP32-C6.


## Requirements

- ESP32-C6 devkit

- Ubuntu Linux, with:

	```
	$ sudo apt install libssl-dev pkg-config python3-venv
	```

- Rust with:

	```
	$ cargo install --locked ldproxy
	```

>Note: [Lima VM](https://lima-vm.io) and [EdgeVM](https://github.com/finalyards/EdgeVM) provide a virtualized, embedded-aligned development toolchain that can be used with this repo.


## Steps

```
$ cargo clean
```

```
$ cargo build --release -vv
```

### The output

```
$ file ~/target/riscv32imac-esp-espidf/release/min
/home/lima/target/riscv32imac-esp-espidf/release/min: ⏎
ELF 32-bit LSB executable, UCB RISC-V, RVC, soft-float ABI, version 1 (SYSV), statically linked, not stripped
```

>Note: Use either `target` or `~/target`, depending on your Rust setup.

<!--
```
$ ls -al ~/target/riscv32imac-esp-espidf/release/min
-rwxrwxr-x 2 lima lima 1076596 Oct 30 14:10 /home/lima/target/riscv32imac-esp-espidf/release/min
```

That gives the output a 1.03 MiB size.
-->

### Does it run?

For this repo, running is a sideline, but let's do it, anyways. 

```
$ cargo run --release
[...]
I (308) min: Merry
I (1308) min: go
I (2308) min: around
I (3308) min: ...
I (4308) min: Merry
I (5308) min: go
I (6308) min: around
I (7308) min: ...
I (8308) min: Merry
I (9308) min: go
I (10308) min: around
I (11308) min: ...
```

The lines should emerge one every 1s (at least the logging timestamps confirm that).

### Wipe out

By now, you have noticed that `~/.espressif` gains significant weight.

```
$ du -h -d1 ~/.espressif/
1.4G	/home/lima/.espressif/dist
208M	/home/lima/.espressif/python_env
9.4G	/home/lima/.espressif/tools
12K	/home/lima/.espressif/root_managed_components
1.5G	/home/lima/.espressif/esp-idf
13G	/home/lima/.espressif/
```

>*The author's folder, with two (5.5.6, 6.1) toolchains installed.*

You can *always* just remove `~/.espressif` completely. It is a cache and will be rebuilt by `esp-idf-sys`, next time you build.


## Advanced

<!--
### Skinning

See [`skinned/README.md`](skinned/README.md) for attempts to limit the number of ESP-IDF components involved in a project.
-->

### Working with ESP-IDF

#### Changing versions

Different ESP-IDF versions vacate different paths within the `~/.espressif` folder, and seem to be able to co-exist. 

To change versions, *normally* it's enough to:

- edit `.cargo/config.toml`
- do `cargo clean` 
- rebuild

If, however, you have cancelled a build midway, it might be that an ESP-IDF installation is corrupt. In such a case, just remove `~/.espressif` and try again.


#### `no_std`, anyone?

It *is* technically possible to build an `esp-idf` application, using `esp-idf-sys` only (`no_std` + `alloc`, for example). However, when placing ESP-IDF together with Embassy, one inevitably needs to bring in `esp-idf-svc`. 

This has lead the author to take the following stand:

- when building libraries, attempt to depend on `esp-idf-sys` only (and make them `no_std` + `alloc` if possible)
- when building *applications*, if Embassy is involved, take in full `esp-idf-svc` stack.

The purpose of this repo is actually to showcase just this: how to "marry" ESP-IDF and `async`, in practise.

EOF.
