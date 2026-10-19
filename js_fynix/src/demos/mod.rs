//! The site's live demos. Each file's `// snippet:start` to
//! `// snippet:end` is the code its page shows, so the page and what
//! runs cannot drift apart.

mod each;
mod rules;
mod states;
mod themes;

pub use each::EachDemo;
pub use rules::RulesDemo;
pub use states::StatesDemo;
pub use themes::ThemesDemo;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// A new `<div>` with `class`, appended to `parent`.
fn div(parent: &HtmlElement, class: &str) -> HtmlElement {
    let doc = parent.owner_document().expect("a document");
    let el = doc
        .create_element("div")
        .expect("a div")
        .unchecked_into::<HtmlElement>();
    el.set_class_name(class);
    let _ = parent.append_child(&el);
    el
}
