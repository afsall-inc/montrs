use std::{collections::VecDeque, sync::Mutex};

/// A simple frame loop scheduler.
///
/// On WASM, uses `requestAnimationFrame`. On native, uses a timer-based loop.
/// Inspired by Motion's frameloop system.
static FRAME_LOOP: std::sync::LazyLock<Mutex<FrameLoopState>> =
    std::sync::LazyLock::new(|| {
        Mutex::new(FrameLoopState {
            callbacks: VecDeque::new(),
            start_time: 0.0,
            running: false,
        })
    });

struct FrameLoopState {
    callbacks: VecDeque<Box<dyn FnMut() -> bool + Send>>,
    start_time: f64,
    running: bool,
}

pub struct FrameLoop;

impl FrameLoop {
    /// Get current time in seconds (high-resolution).
    pub fn now() -> f64 {
        #[cfg(target_arch = "wasm32")]
        {
            web_sys::window()
                .and_then(|w| w.performance())
                .map(|p| p.now() / 1000.0)
                .unwrap_or(0.0)
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs_f64()
        }
    }

    /// Register a callback for the next frame. Return `true` to continue, `false` to stop.
    pub fn on_frame(callback: impl FnMut() -> bool + Send + 'static) {
        let mut state = FRAME_LOOP.lock().unwrap();
        state.callbacks.push_back(Box::new(callback));
        if !state.running {
            state.running = true;
            state.start_time = Self::now();
            Self::schedule_next();
        }
    }

    fn schedule_next() {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            let closure = wasm_bindgen::prelude::Closure::wrap(Box::new(|| {
                Self::tick();
            }) as Box<dyn FnMut()>);
            web_sys::window().and_then(|w| {
                w.request_animation_frame(closure.as_ref().unchecked_ref())
                    .ok()
            });
            closure.forget();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs_f64(
                    1.0 / 60.0,
                ));
                Self::tick();
            });
        }
    }

    fn tick() {
        let mut state = FRAME_LOOP.lock().unwrap();
        let mut remaining = VecDeque::new();
        while let Some(mut cb) = state.callbacks.pop_front() {
            if cb() {
                remaining.push_back(cb);
            }
        }
        state.callbacks = remaining;
        if !state.callbacks.is_empty() {
            drop(state);
            Self::schedule_next();
        } else {
            state.running = false;
        }
    }
}
