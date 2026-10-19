//! Fynix in the browser: a backend whose nodes are `<div>`s, and the
//! live demos on fynix.voxell.dev, written against it.
//!
//! Each demo is a `#[wasm_bindgen]` struct that builds into the
//! element it is given and is stepped by `frame` from a
//! `requestAnimationFrame` loop. The code between a demo file's
//! `// snippet:start` and `// snippet:end` is what its page shows.

mod backend;
mod demos;
mod elements;
mod tokens;
mod ui;
mod views;

pub use demos::{
    EachDemo, HeroDemo, RulesDemo, StatesDemo, ThemesDemo,
};
