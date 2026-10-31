# Build size for ESP-IDF 6.1

All experiments carried out against `esp32c6` target.


## Tool requirements

```
$ cargo install cargo-binutils
```

```
$ rustup component add llvm-tools
```

>Note: We use `cargo size --release` because the author's remote `espflash` setup does not allow `espflash save-image`. If you can, use it instead.

## Regular build

- `EXPERIMENTS`-marked section in `sdkconfig.defaults` commented out

```
$ cargo clean
$ cargo build --release -vv
```

```
$ cargo size --release
    Finished `release` profile [optimized] target(s) in 0.10s
   text	   data	    bss	    dec	    hex	filename
 405416	   9564	 299556	 714536	  ae728	min
```

- `text` (flash): 405416
- `data` (RAM): 9564


## Skinned build

- `EXPERIMENTS` in `sdkconfig.defaults`:

	```
	## EXPERIMENTS
	#
	#CONFIG_ESP_WIFI_ENABLED=n
    	# does not cut it ⛑️
	CONFIG_ESP_WIFI_REMOTE_ENABLED=n
    	# works 👍
	CONFIG_LWIP_IPV6=n
    	# works 👍
	CONFIG_WS_TRANSPORT=n
    	# works 👍
	```

```
$ cargo clean
$ cargo run --release -vv
```

```
$ cargo size --release
[...]
   text	   data	    bss	    dec	    hex	filename
 405416	   9564	 299556	 714536	  ae728	min
```

The results are **absolutely same** as above, in a non-restricted build.

- `text` (flash): 405416
- `data` (RAM): 9564


### Analysis

Disabling the three ESP-IDF keys has **absolutely no** effect on the binary size. This is weird, and points out the author does not understand *how* the ESP-IDF flag system actually works.

The expectation is that the "skinned" build would have been at least a fraction smaller.

*tbd. ask for advice, `#help`*

