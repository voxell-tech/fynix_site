//! One view, three themes.

use fynix::{Motion, ScopedExt, View};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use super::div;
use crate::backend::Web;
use crate::elements::{FrameProps, Justify, Tone, label};
use crate::tokens::{Candy, Monokai, Paper, Theme};
use crate::ui::{Ui, delta};
use crate::views::{button, column, ghost, panel, primary, row};

// snippet:start
/// One card, written once. Its colours, sizes, corners and curves
/// all come from the theme it is built with.
fn card<T: Theme>() -> impl View<Web, T> {
    column((
        label("Deploy to production").bold(true),
        label("All checks passed.").tone(Tone::Dim),
        row((
            button(label("Later")).rules(ghost),
            button(label("Deploy")).rules(primary),
        ))
        .justify(Justify::End),
    ))
    .rules(panel)
    .transition(Motion::Interact)
}

/// The same card under three themes.
fn mount(
    dark: &mut Ui<Monokai>,
    light: &mut Ui<Paper>,
    sweet: &mut Ui<Candy>,
) {
    dark.mount(card());
    light.mount(card());
    sweet.mount(card());
}
// snippet:end

/// [`card`] under [`Monokai`], [`Paper`] and [`Candy`], side by side.
#[wasm_bindgen]
pub struct ThemesDemo {
    dark: Ui<Monokai>,
    light: Ui<Paper>,
    sweet: Ui<Candy>,
}

#[wasm_bindgen]
impl ThemesDemo {
    #[wasm_bindgen(constructor)]
    pub fn new(container: HtmlElement) -> ThemesDemo {
        let pane = |name: &str, class: &str| {
            let pane = div(&container, &format!("fx-pane {class}"));
            div(&pane, "fx-caption").set_text_content(Some(name));
            div(&pane, "fx-stage")
        };
        let mut dark = Ui::new(pane("Monokai", "fx-dark"), Monokai);
        let mut light = Ui::new(pane("Paper", "fx-light"), Paper);
        let mut sweet = Ui::new(pane("Candy", "fx-candy"), Candy);
        mount(&mut dark, &mut light, &mut sweet);
        ThemesDemo { dark, light, sweet }
    }

    pub fn frame(&mut self, dt_ms: f64, reduced_motion: bool) {
        let delta = delta(dt_ms);
        self.dark.frame(delta, reduced_motion);
        self.light.frame(delta, reduced_motion);
        self.sweet.frame(delta, reduced_motion);
    }
}
