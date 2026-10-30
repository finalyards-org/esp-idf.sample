# Skinning

In embedded development, we wish the target binaries to be slim. Being able to work only on the components your project actually uses..

- reduces build time
- reduces attach surfaces
- reduces overall complexity
- improves flashing speed
- allows more features to fit on the target device

Rust has been designed around these concepts. Libraries have features, dead code elimination removes code that seeped in but isn't actually needed in the eventual target.

But ESP-IDF is made in C. **Your application is a Rust/C hybrid.**


## State of ESP-IDF

The transition to ESP-IDF 6 has made the framework more modular. Components that used to be internal are now often external.

This is a good start, and deserves support! Especially we in the Rust community can push the project in a direction where the outcome is in line with what Rust `std` support really means.


## The aim

The author would like to, using this repo as a playground, 

a. observe the number of components that "seep in" - uninvited - to a project that does not need them.

- excluding those that provide meaningful functionality for Rust's `std` layer (e.g. general networking, file access, synchronization)

>For example, WiFi should not "seep in". It is not an essential component to do networking on ESP32 platforms.
>
>LWIP (TCP/IP), however, could be allowed. 


## The steps

1. Identify what components *currently* get built in, in this minimal application.

	- Focus on ESP-IDF 6.1 (not 5.5.x) since this is forward-looking work.

2. Highlight the ones regarded as "uninvited".

	- This should be a **discussion starter**. Espressif maintains the ESP-IDF (C side) repo, and is in no obligation to change anything, especially if PR's are not being made? But highlighting the components just **might help** them continue the modularization work started in ESP-IDF 6 transition, and bring the ESP-IDF world and the Rust world, closer together. 🌤️


## The playground

Perhaps a branch of this repo will function as a playing ground (`skinned`) for such work.

And a folder (`skinned/`) for extra information.

>This leaves the `main` for regular visitors to get a working, occasionally updated template for a general ESP-IDF + Embassy project.


## The diff

The skinning happens in two places:

- `ESP_IDF_COMPONENTS`

	This, within `.cargo/config.toml`, allows one to restrict the list of components brought in.
	
- `sdkconfig.defaults`

	This allows switching components off, using kConfig keys.
