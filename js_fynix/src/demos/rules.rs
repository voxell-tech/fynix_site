//! Set rules, the call site, show rules, and scopes.

use fynix::{AnyView, Cx, ScopedExt};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::backend::Web;
use crate::elements::{Label, Tone, label};
use crate::tokens::Monokai;
use crate::ui::{Ui, delta};
use crate::views::{column, panel};

// snippet:start
fn rules(cx: &mut Cx<'_, Web, Monokai>) {
    cx.scope(|cx| {
        // Every label from here on is small and dim...
        cx.set::<Label>(|l, _| l.size(12.0).tone(Tone::Dim));
        cx.build(label("A set rule: small and dim"));
        // ...unless its call site says otherwise.
        cx.build(label("The call site wins: big").size(22.0));
        // A show rule wins over the call site.
        cx.show::<Label>(|l, _| l.tone(Tone::Accent).bold(true));
        cx.build(
            label("A show rule wins: accent").tone(Tone::Critical),
        );
    });
    // The scope ended, and its rules with it.
    cx.build(label("After the scope: plain again"));
}
// snippet:end

/// [`rules`], built in a card.
#[wasm_bindgen]
pub struct RulesDemo {
    ui: Ui<Monokai>,
}

#[wasm_bindgen]
impl RulesDemo {
    #[wasm_bindgen(constructor)]
    pub fn new(container: HtmlElement) -> RulesDemo {
        let mut ui = Ui::new(container, Monokai);
        ui.mount(AnyView::new(|cx: &mut Cx<'_, Web, Monokai>| {
            let root = cx.build(column(()).rules(panel));
            cx.under(root, rules);
            root
        }));
        RulesDemo { ui }
    }

    pub fn frame(&mut self, dt_ms: f64, reduced_motion: bool) {
        self.ui.frame(delta(dt_ms), reduced_motion);
    }
}
