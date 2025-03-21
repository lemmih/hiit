use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{WakeLockSentinel, WakeLockType};

/// Component that acquires a screen wake lock when mounted and releases it when unmounted.
/// This prevents the device screen from turning off while the component is active.
#[component]
pub fn ScreenWakeLock() -> impl IntoView {
    // Store the wake lock sentinel in a LocalResource
    let wake_lock_resource = LocalResource::new(|| async { acquire_wake_lock().await });

    // Function to acquire the wake lock
    async fn acquire_wake_lock() -> Option<WakeLockSentinel> {
        // Check if the browser supports the Wake Lock API
        if let Some(window) = web_sys::window() {
            let navigator = window.navigator();
            // Request a screen wake lock
            let wake_lock = navigator.wake_lock();
            match JsFuture::from(wake_lock.request(WakeLockType::Screen)).await {
                Ok(sentinel) => {
                    if let Ok(sentinel) = sentinel.dyn_into::<WakeLockSentinel>() {
                        log::info!("Screen wake lock acquired");
                        return Some(sentinel);
                    }
                }
                Err(err) => {
                    log::warn!("Failed to acquire wake lock: {:?}", err);
                }
            }
        }

        log::warn!("Wake Lock API not supported by this browser");
        None
    }

    // Release the wake lock when the component is unmounted
    on_cleanup(move || {
        // Try to get the wake lock from resource
        if let Some(Some(sentinel)) = wake_lock_resource.get().as_deref() {
            let _ = sentinel.release();
            log::info!("Screen wake lock released");
        }
    });

    // The component doesn't render anything visible
    view! { <div class="hidden"></div> }
}
