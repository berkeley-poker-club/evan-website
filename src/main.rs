mod app;
mod components;
mod pages;

use app::*;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    web_sys::console::log_1(&"WASM main() starting...".into());
    mount_to_body(|| {
        web_sys::console::log_1(&"Mounting app...".into());
        view! {
            <App/>
        }
    });
    web_sys::console::log_1(&"WASM main() completed".into());
}
