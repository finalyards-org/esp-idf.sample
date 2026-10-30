
use embassy_executor::Spawner;

use esp_idf_svc::{
    sys::link_patches,
};

/// Asynchronous example.
///
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    let msg = ["Merry", "go", "around", "..."];

    loop {
        for s in &msg {
            log::info!("{}", s);
            embassy_time::Timer::after_secs(1).await;
        }
    }
}
