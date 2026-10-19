//! Buttons whose looks hold while hovered and pressed.

use fynix::{Cx, Motion, ScopedExt, View};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::backend::{Hovered, Pressed, Web};
use crate::elements::{Frame, FrameProps, Label, Tone, label};
use crate::tokens::{Monokai, SurfaceTokens};
use crate::ui::{Ui, delta};
use crate::views::{button, danger, ghost, primary, row};

// snippet:start
fn buttons() -> impl View<Web, Monokai> {
    row((
        button(label("Cancel")).rules(ghost),
        button(label("Save")).rules(primary),
        button(label("Delete")).rules(danger),
        // A look of its own, written in place.
        button(label("Hover me"))
            .when_in::<Hovered, _>(|cx: &mut Cx<Web, Monokai>| {
                cx.root(|cx| {
                    cx.set::<Frame>(|f, t| {
                        f.fill(t.accent()).scale(1.1)
                    })
                });
                cx.set::<Label>(|l, _| l.tone(Tone::OnAccent));
            })
            .when_in::<Pressed, _>(|cx: &mut Cx<Web, Monokai>| {
                cx.root(|cx| cx.set::<Frame>(|f, _| f.scale(0.9)));
            }),
    ))
    .gap(12.0)
    .transition(Motion::Interact)
}
// snippet:end

/// [`buttons`], to hover and press.
#[wasm_bindgen]
pub struct StatesDemo {
    ui: Ui<Monokai>,
}

#[wasm_bindgen]
impl StatesDemo {
    #[wasm_bindgen(constructor)]
    pub fn new(container: HtmlElement) -> StatesDemo {
        let mut ui = Ui::new(container, Monokai);
        ui.mount(buttons());
        StatesDemo { ui }
    }

    pub fn frame(&mut self, dt_ms: f64, reduced_motion: bool) {
        self.ui.frame(delta(dt_ms), reduced_motion);
    }
}
