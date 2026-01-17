use crate::Route;
use dioxus::prelude::*;
use gloo_events::EventListener;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

const SLIDE_ORDER: &[&str] = &[
    "/",
    "/motivation",
    "/background",
    "/rationale",
    "/methods",
    "/results",
    "/discussion",
    "/future",
    "/video",
];

fn normalize_path(path: &str) -> String {
    let path = path.trim();
    if path == "/" {
        return "/".to_string();
    }
    let path = if path.ends_with('/') {
        &path[..path.len() - 1]
    } else {
        path
    };
    if !path.starts_with('/') {
        format!("/{}", path)
    } else {
        path.to_string()
    }
}

pub fn get_next_route(current_path: &str) -> Option<&'static str> {
    let normalized = normalize_path(current_path);
    if let Some(pos) = SLIDE_ORDER.iter().position(|&r| r == normalized) {
        if pos + 1 < SLIDE_ORDER.len() {
            return Some(SLIDE_ORDER[pos + 1]);
        }
    }
    None
}

pub fn get_prev_route(current_path: &str) -> Option<&'static str> {
    let normalized = normalize_path(current_path);
    if let Some(pos) = SLIDE_ORDER.iter().position(|&r| r == normalized) {
        if pos > 0 {
            return Some(SLIDE_ORDER[pos - 1]);
        }
    }
    None
}

#[component]
pub fn NavControls() -> Element {
    let nav = use_navigator();
    let route = use_route::<Route>();

    // 1. We use a raw Rc<RefCell> to share state with the event listener safely.
    // This avoids Dioxus Signal runtime borrowing conflicts.
    // Rc is Clone, so use_hook is happy.
    let nav_state = use_hook(|| Rc::new(RefCell::new((None::<String>, None::<String>))));

    // 2. Calculate routes efficiently on every render
    let current_path = route.to_string();
    let next_str = get_next_route(&current_path).map(|s| s.to_string());
    let prev_str = get_prev_route(&current_path).map(|s| s.to_string());

    // 3. Update the shared state using use_effect to avoid RefCell borrow failures during render.
    // We clone the values to move them into the effect closure.
    let nav_state_update = nav_state.clone();
    let p_clone = prev_str.clone();
    let n_clone = next_str.clone();

    use_effect(move || {
        if let Ok(mut state) = nav_state_update.try_borrow_mut() {
            *state = (p_clone.clone(), n_clone.clone());
        } else {
            web_sys::console::warn_1(&"NavControls: Failed to borrow nav_state for update".into());
        }
    });

    // 4. Create the EventListener ONCE and keep it alive in a use_signal (or just use_hook resource).
    // The previous implementation used use_signal to hold the listener, which is fine.
    // We clone the Rc for the closure.
    let nav_state_for_listener = nav_state.clone();
    let nav_for_listener = nav.clone();

    // Store listener in a signal to keep it alive for the component's lifetime
    use_hook(move || {
        let window = web_sys::window().unwrap();

        let listener = EventListener::new(&window, "keydown", move |event| {
            let event = event.dyn_ref::<KeyboardEvent>().unwrap();
            let key = event.key();
            // Ignore modifiers
            if event.alt_key() || event.ctrl_key() || event.meta_key() || event.shift_key() {
                return;
            }

            match key.as_str() {
                "ArrowRight" | " " => {
                    // Safe borrow from RefCell
                    if let Ok(state) = nav_state_for_listener.try_borrow() {
                        let target = state.1.clone();
                        if let Some(t) = target {
                            web_sys::console::log_1(&format!("Key Nav -> {}", t).into());
                            nav_for_listener.push(t);
                        }
                    }
                }
                "ArrowLeft" => {
                    if let Ok(state) = nav_state_for_listener.try_borrow() {
                        let target = state.0.clone();
                        if let Some(t) = target {
                            web_sys::console::log_1(&format!("Key Nav -> {}", t).into());
                            nav_for_listener.push(t);
                        }
                    }
                }
                _ => {}
            }
        });

        // Return boxed listener or just the listener.
        // use_hook stores the return value.
        // We need to keep this object alive.
        Rc::new(listener)
    });

    rsx! {
        div {
            class: "fixed bottom-8 right-8 flex gap-4 z-50",

            if let Some(p) = prev_str {
                button {
                    class: "p-3 rounded-full bg-brand-dark hover:bg-zinc-700 text-brand-orange shadow-lg border border-brand-orange transition-all active:scale-95 cursor-pointer opacity-100",
                    onclick: move |_| {
                        nav.push(p.clone());
                    },
                    svg {
                        class: "w-6 h-6",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M15 18l-6-6 6-6" }
                    }
                }
            }
            if let Some(n) = next_str {
                button {
                    class: "p-3 rounded-full bg-brand-dark hover:bg-zinc-700 text-brand-orange shadow-lg border border-brand-orange transition-all active:scale-95 cursor-pointer opacity-100",
                    onclick: move |_| {
                        nav.push(n.clone());
                    },
                    svg {
                        class: "w-6 h-6",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path { d: "M9 18l6-6-6-6" }
                    }
                }
            }
        }
    }
}
