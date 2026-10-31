# Report for ESP-IDF 6.1

All experiments carried out against `esp32c6` target.



## Networking

### Bluetooth (`BT_ENABLED`)

- ✅ disabled by default

### WiFi (`ESP_WIFI_ENABLED`)

- 🟨 Fixed:

	```
	default y if SOC_WIFI_SUPPORTED(=y)
	```

Not every project needs WiFi. Would be nice to be able to switch this off in `sdkconfig.defaults`.

### Remote WiFi (`ESP_WIFI_REMOTE_ENABLED`)

- ✅ Default on; can be disabled.


### OpenThread (`OPENTHREAD_ENABLED`)

- ✅ disabled by default


## TCP/IP

### IPv6 (`LWIP_IPV6`)

- ✅ Default on; can be disabled. <!--not tried-->

### WebSocket (`WS_TRANSPORT`)

- ✅ Default on; can be disabled. <!--not tried-->


## File systems

### FATfs (*no key*)

- 🟨 Not configurable.

	Q: Is there a reason why FAT is always-on, or is it just legacy?

	>In the Zigbee project that the author works on, a partition called `zb_fct` does have type `fat`, but it's unknown (need to ask!) whether it's even needed.

	It *feels* like FAT could be opt-in.


## Built-in hardware drivers

ESP-IDF (C) / `esp-idf` (Rust) take the approach that all hardware drivers where physical capability exists on a chip are enabled.

This is probably okay.

Where it matters is when building such drivers causes warnings that I -on application level- can mitigate, by certain kConfig keys.

```
CONFIG_I2C_SUPPRESS_DEPRECATE_WARN=y
CONFIG_TWAI_SUPPRESS_DEPRECATE_WARN=y
```

The project uses neither I2C not TWAI.

It would likely be too much work to make this part modular - Espressif folks are the best judge of it.

