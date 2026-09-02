use leptos::prelude::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

/// Site-wide custom cursor: hides the native pointer and renders a small gold
/// spade that tracks the mouse exactly, plus a larger hollow gold ring that
/// eases toward the mouse with a trailing lag.
#[component]
pub fn CustomCursor() -> impl IntoView {
    let spade_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let ring_ref: NodeRef<leptos::html::Div> = NodeRef::new();

    Effect::new(move |_| {
        let (Some(spade_el), Some(ring_el)) = (spade_ref.get(), ring_ref.get()) else {
            return;
        };

        let Some(window) = web_sys::window() else {
            return;
        };

        // Latest raw mouse position, updated on every mousemove.
        let target = Rc::new(Cell::new((0.0_f64, 0.0_f64)));

        {
            let target = target.clone();
            let mousemove = Closure::<dyn FnMut(MouseEvent)>::new(move |e: MouseEvent| {
                target.set((e.client_x() as f64, e.client_y() as f64));
            });
            let _ = window
                .add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref());
            // Lives for the lifetime of the page, so it's fine to leak.
            mousemove.forget();
        }

        // Eased position for the trailing ring.
        let ring_pos = Rc::new(Cell::new((0.0_f64, 0.0_f64)));

        let tick: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let tick_setup = tick.clone();

        *tick_setup.borrow_mut() = Some(Closure::new(move || {
            let (tx, ty) = target.get();

            let _ = spade_el.set_attribute(
                "style",
                &format!(
                    "position: fixed; top: 0; left: 0; z-index: 99999; pointer-events: none; \
                     transform: translate({tx}px, {ty}px) translate(-50%, -55%); \
                     color: #F5C842; font-size: 23px; line-height: 1; user-select: none;"
                ),
            );

            let (rx, ry) = ring_pos.get();
            let nrx = rx + (tx - rx) * 0.15;
            let nry = ry + (ty - ry) * 0.15;
            ring_pos.set((nrx, nry));

            let _ = ring_el.set_attribute(
                "style",
                &format!(
                    "position: fixed; top: 0; left: 0; z-index: 99999; pointer-events: none; \
                     transform: translate({nrx}px, {nry}px) translate(-50%, -50%); \
                     width: 40px; height: 40px; border-radius: 9999px; \
                     border: 2px solid #F5C842; background: transparent;"
                ),
            );

            if let Some(win) = web_sys::window() {
                let cb_ref = tick.borrow();
                if let Some(cb) = cb_ref.as_ref() {
                    let _ = win.request_animation_frame(cb.as_ref().unchecked_ref());
                }
            }
        }));

        let cb_ref = tick_setup.borrow();
        if let Some(cb) = cb_ref.as_ref() {
            let _ = window.request_animation_frame(cb.as_ref().unchecked_ref());
        }
    });

    view! {
        <style>
            "* { cursor: none !important; }"
        </style>
        <div node_ref=spade_ref style="position: fixed; top: 0; left: 0; z-index: 99999; pointer-events: none;">
            "♠"
        </div>
        <div node_ref=ring_ref style="position: fixed; top: 0; left: 0; z-index: 99999; pointer-events: none;"></div>
    }
}
