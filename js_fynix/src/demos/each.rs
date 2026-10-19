//! A list whose rows come and go.

use fynix::{Motion, ScopedExt, View, ViewExt, each};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::backend::{Web, resource};
use crate::elements::{
    Align, Color, FrameProps, Justify, Pad, Tone, frame, label,
};
use crate::tokens::Monokai;
use crate::ui::{Ui, delta};
use crate::views::{
    AppearExt, ClickExt, button, column, hidden, primary, row,
};

/// The rows' ids, and the next id to hand out.
pub struct Rows {
    ids: Vec<u32>,
    next: u32,
}

// snippet:start
fn list() -> impl View<Web, Monokai> {
    column((
        row((
            button(label("Add")).rules(primary).on_click(|world| {
                let rows = world.get_mut::<Rows>();
                rows.ids.push(rows.next);
                rows.next += 1;
            }),
            button(label("Remove")).on_click(|world| {
                let rows = world.get_mut::<Rows>();
                if !rows.ids.is_empty() {
                    rows.ids.remove(0);
                }
            }),
            button(label("Reverse")).on_click(|world| {
                world.get_mut::<Rows>().ids.reverse()
            }),
        )),
        each(
            resource::<Rows, _>(|rows| rows.ids.clone()),
            |id| *id,
            |id| {
                item(*id)
                    .appear(hidden)
                    .transition(Motion::Expand)
                    .boxed()
            },
        )
        .within(column(()).gap(6.0)),
    ))
    .gap(14.0)
    .transition(Motion::Interact)
}
// snippet:end

const DOTS: [Color; 5] = [
    Color::hex(0xfc9867),
    Color::hex(0xffd866),
    Color::hex(0xa9dc76),
    Color::hex(0x78dce8),
    Color::hex(0xab9df2),
];

/// One row of the list: a dot in its own colour, and its name.
fn item(id: u32) -> impl View<Web, Monokai> {
    row((
        row((
            frame()
                .padding(Pad::all(5.0))
                .radius(5.0)
                .fill(DOTS[id as usize % DOTS.len()]),
            label(format!("Row {id}")),
        ))
        .align(Align::Center)
        .gap(10.0),
        label(format!("#{id:03}")).tone(Tone::Dim).size(12.0),
    ))
    .align(Align::Center)
    .justify(Justify::Between)
    .padding(Pad::new(14.0, 10.0))
    .radius(8.0)
    .fill(Color::hex(0x2d2a2e))
    .border(1.0)
    .border_color(Color::hex(0xfcfcfa).alpha(0.08))
}

/// [`list`], starting with three rows.
#[wasm_bindgen]
pub struct EachDemo {
    ui: Ui<Monokai>,
}

#[wasm_bindgen]
impl EachDemo {
    #[wasm_bindgen(constructor)]
    pub fn new(container: HtmlElement) -> EachDemo {
        let mut ui = Ui::new(container, Monokai);
        ui.world.insert(Rows {
            ids: vec![1, 2, 3],
            next: 4,
        });
        ui.mount(list());
        EachDemo { ui }
    }

    pub fn frame(&mut self, dt_ms: f64, reduced_motion: bool) {
        self.ui.frame(delta(dt_ms), reduced_motion);
    }
}
