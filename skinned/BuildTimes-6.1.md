# Build times for ESP-IDF 6.1

All experiments carried out against `esp32c6` target.

Build environment:

- Lima VM with 4 cores

>The build environment does not matter, as long as it's the same between builds.


## Regular build

- `EXPERIMENTS`-marked section in `sdkconfig.defaults` commented out

```
$ cargo clean
$ time cargo build --release -vv
[...]
real	0m47.498s
user	1m54.016s
sys		0m14.465s
```


## Skinned build

- `EXPERIMENTS` in `sdkconfig.defaults`:

	```
	## EXPERIMENTS
	#
	#CONFIG_ESP_WIFI_ENABLED=n
    # does not cut it ⛑️
	CONFIG_ESP_WIFI_REMOTE_ENABLED=n
    # works 👍
	```

```
$ cargo clean
$ time cargo build --release -vv
[...]
real	0m46.819s
user	1m53.910s
sys		0m14.459s
```

### Analysis

- No change of build speed, but that's to be expected: only one component disabled.

	>Also, we don't even know if enabling/disabling components affects build speed (the component might still be built). However, *steering* what features get into the binary is more important / a good start.
