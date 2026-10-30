/*
* build.rs
*
* Gets run by:
*   - IDE on host; WRONG FEATURES!!
*   - 'cargo build' (CLI); correct features
*/
use std::{
    env,
};

/*
*/
fn main() {
    // Detect when IDE is running us:
    //  - Rust Rover:
    //      __CFBundleIdentifier=com.jetbrains.rustrover-EAP
    //  - Rust Rover (remote development):
    //      - REMOTE_DEV_SERVER_IS_NATIVE_LAUNCHER=1
    {
        if env::var("__CFBundleIdentifier").is_ok() ||
            env::var("REMOTE_DEV_SERVER_IS_NATIVE_LAUNCHER").is_ok() {
            panic!("IDE build cut short");   // try to avoid spending a _lot_ of time, building ESP-IDF on the IDE
            //return;  // skip the rest
        }
    }

    // DEBUG
    #[cfg(false)]
    {
        env::vars().for_each(|(a, b)| { eprintln!("{a}={b}"); });
        panic!();
    }

    // Ensure all clear for the build
    {
        // Ensure system-wide ESP-IDF install is not active - it would confuse the build.
        if env::var("IDF_PATH").is_ok() {
            panic!("❗️Please build with a shell that doesn't know of system-wide esp-idf. 'IDF_PATH' env.var. detected.");
        }

        // Ensure no 'sdkconfig' left-over.
        let fn_ = std::path::Path::new("sdkconfig");
        if fn_.exists() {
            panic!("❗️There's an 'sdkconfig' file. It may confuse the build. Please remove it and try again.");
        }
    }

    // Needed. This is the step that downloads and builds 'ESP-IDF' if not already there.
    embuild::espidf::sysenv::output();

    // ...place your own additions below...
}
