//! The landing page hero: one card that restyles itself.
//!
//! The card and the controls under it are written once. Their theme
//! is [`Live`], whose tokens follow the [`Look`] the controls pick,
//! so switching a skin or a switch glides the same views into a new
//! look.

use core::time::Duration;
use std::sync::{Arc, Mutex};

use fynix::{
    Curve, Cx, Motion, MotionTokens, ScopedExt, Signal, View,
};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::backend::{Hovered, Pressed, Web, World, resource};
use crate::elements::{
    Align, Color, Frame, FrameProps, Justify, Pad, Tone, frame, label,
};
use crate::tokens::{
    Candy, Monokai, Paper, SpacingTokens, SurfaceTokens, TextTokens,
};
use crate::ui::{Ui, delta};
use crate::views::{Button, ClickExt, button, column, row};

/// One of the site's three themes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skin {
    Monokai,
    Paper,
    Candy,
}

impl Skin {
    const ALL: [Skin; 3] = [Skin::Monokai, Skin::Paper, Skin::Candy];

    fn name(self) -> &'static str {
        match self {
            Skin::Monokai => "Monokai",
            Skin::Paper => "Paper",
            Skin::Candy => "Candy",
        }
    }

    fn tokens(self) -> &'static dyn Base {
        match self {
            Skin::Monokai => &Monokai,
            Skin::Paper => &Paper,
            Skin::Candy => &Candy,
        }
    }
}

/// The token traits a skin has, as one object.
trait Base: TextTokens + SurfaceTokens + SpacingTokens {}

impl<T: TextTokens + SurfaceTokens + SpacingTokens> Base for T {}

/// What the controls pick: a skin, and two switches on top of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Look {
    pub skin: Skin,
    pub round: bool,
    pub roomy: bool,
}

impl Look {
    fn base(&self) -> &'static dyn Base {
        self.skin.tokens()
    }

    fn radius(&self) -> f32 {
        let r = self.base().radius();
        if self.round { r * 1.5 + 10.0 } else { r }
    }

    fn gap(&self) -> f32 {
        self.base().gap() * if self.roomy { 1.7 } else { 1.0 }
    }

    fn pad(&self) -> f32 {
        self.base().pad() * if self.roomy { 1.5 } else { 1.0 }
    }
}

/// A theme whose tokens are whatever [`Look`] it was last given.
#[derive(Clone)]
pub struct Live(Arc<Mutex<Look>>);

impl Live {
    fn look(&self) -> Look {
        *self.0.lock().expect("an unpoisoned look")
    }
}

impl TextTokens for Live {
    fn tone(&self, tone: Tone) -> Color {
        self.look().base().tone(tone)
    }

    fn body_size(&self) -> f32 {
        self.look().base().body_size()
    }
}

impl SurfaceTokens for Live {
    fn panel(&self) -> Color {
        self.look().base().panel()
    }

    fn fill(&self) -> Color {
        self.look().base().fill()
    }

    fn hover(&self) -> Color {
        self.look().base().hover()
    }

    fn pressed(&self) -> Color {
        self.look().base().pressed()
    }

    fn accent(&self) -> Color {
        self.look().base().accent()
    }

    fn critical(&self) -> Color {
        self.look().base().critical()
    }

    fn border(&self) -> Color {
        self.look().base().border()
    }
}

impl SpacingTokens for Live {
    fn gap(&self) -> f32 {
        self.look().gap()
    }

    fn radius(&self) -> f32 {
        self.look().radius()
    }

    fn pad(&self) -> f32 {
        self.look().pad()
    }
}

impl MotionTokens for Live {
    /// One unhurried glide for everything, so a whole restyle and a
    /// hover move alike.
    fn motion(&self, _: Motion) -> Curve {
        Curve {
            duration: Duration::from_millis(320),
            ease: |t| 1.0 - (1.0 - t).powi(3),
        }
    }
}

/// A prop that follows the [`Look`].
fn look<T>(
    read: impl Fn(&Look) -> T + Send + Sync + 'static,
) -> Signal<World, T> {
    resource::<Look, _>(read)
}

/// A small round mark in `color`.
fn dot(color: Color) -> impl View<Web, Live> {
    frame().padding(Pad::all(4.0)).radius(4.0).fill(color)
}

/// One passed check: a mark, its name and how long it took.
fn check(
    name: &'static str,
    took: &'static str,
) -> impl View<Web, Live> {
    row((
        row((dot(Color::hex(0xa9dc76)), label(name)))
            .align(Align::Center),
        label(took).tone(Tone::Dim).size(12.0),
    ))
    .align(Align::Center)
    .justify(Justify::Between)
    .padding(look(|l| Pad::new(l.pad() * 1.4, l.pad())))
    .radius(look(|l| l.radius()))
    .fill(look(|l| l.base().fill()))
}

/// The card: a deploy about to go out.
fn card() -> impl View<Web, Live> {
    column((
        row((
            row((
                dot(Color::hex(0xfc9867)),
                label("fynix.voxell.dev"),
            ))
            .align(Align::Center),
            label("Ready").tone(Tone::Accent).bold(true).size(12.0),
        ))
        .align(Align::Center)
        .justify(Justify::Between),
        column((
            label("Ship the new site").bold(true).size(20.0),
            label("Three checks passed.").tone(Tone::Dim),
        ))
        .gap(4.0),
        column((
            check("Build", "1.2s"),
            check("Tests", "4.8s"),
            check("Preview", "0.6s"),
        ))
        .gap(look(|l| l.gap() * 0.75)),
        row((
            live_button(
                button(label("Later")),
                |_| Color::NONE,
                |l| l.base().hover(),
            ),
            live_button(
                button(
                    label("Deploy").tone(Tone::OnAccent).bold(true),
                ),
                |l| l.base().accent(),
                |l| l.base().accent().lighten(0.18),
            ),
        ))
        .justify(Justify::End),
    ))
    .fill(look(|l| l.base().panel()))
    .radius(look(|l| l.radius() * 1.6))
    .padding(look(|l| Pad::all(l.gap() * 2.0)))
    .gap(look(|l| l.gap() * 1.5))
    .border(1.0)
    .border_color(look(|l| l.base().border()))
    .shadow(28.0)
}

/// `button`, with its fill at rest and under the pointer, corners
/// and padding all following the look. A button's own hover and press
/// looks are worked out once, in the skin it was built in, so a
/// restyle would leave them behind.
fn live_button<C: View<Web, Live>>(
    button: Button<C>,
    rest: impl Fn(&Look) -> Color + Copy + Send + Sync + 'static,
    hover: impl Fn(&Look) -> Color + Copy + Send + Sync + 'static,
) -> impl View<Web, Live> {
    button
        .fill(look(rest))
        .radius(look(|l| l.radius()))
        .padding(look(|l| Pad::new(l.pad() * 1.6, l.pad())))
        .when_in::<Hovered, _>(move |cx: &mut Cx<'_, Web, Live>| {
            cx.root(|cx| {
                cx.set::<Frame>(move |f, _| f.fill(look(hover)))
            });
        })
        .when_in::<Pressed, _>(move |cx: &mut Cx<'_, Web, Live>| {
            cx.root(|cx| {
                cx.set::<Frame>(move |f, _| {
                    f.fill(look(rest)).scale(0.96)
                })
            });
        })
}

/// One skin's button: filled with the accent while it is picked.
fn skin_button(skin: Skin) -> impl View<Web, Live> {
    live_button(
        button(label(skin.name()).size(12.0).bold(true).tone(look(
            move |l| {
                if l.skin == skin {
                    Tone::OnAccent
                } else {
                    Tone::Body
                }
            },
        ))),
        move |l| {
            if l.skin == skin {
                l.base().accent()
            } else {
                l.base().fill()
            }
        },
        move |l| {
            if l.skin == skin {
                l.base().accent().lighten(0.18)
            } else {
                l.base().hover()
            }
        },
    )
    .on_click(move |world| world.get_mut::<Look>().skin = skin)
}

/// A switch's button: outlined in the accent while it is on.
fn switch(
    name: &'static str,
    on: fn(&Look) -> bool,
    flip: fn(&mut Look),
) -> impl View<Web, Live> {
    live_button(
        button(label(name).size(12.0).bold(true).tone(look(
            move |l| {
                if on(l) { Tone::Accent } else { Tone::Dim }
            },
        )))
        .border(1.5)
        .border_color(look(move |l| {
            if on(l) {
                l.base().accent()
            } else {
                Color::NONE
            }
        })),
        |l| l.base().fill(),
        |l| l.base().hover(),
    )
    .on_click(move |world| flip(world.get_mut::<Look>()))
}

/// The controls: the three skins, then the two switches.
fn controls() -> impl View<Web, Live> {
    row((
        row((
            skin_button(Skin::Monokai),
            skin_button(Skin::Paper),
            skin_button(Skin::Candy),
        ))
        .gap(4.0),
        row((
            switch("Round", |l| l.round, |l| l.round = !l.round),
            switch("Roomy", |l| l.roomy, |l| l.roomy = !l.roomy),
        ))
        .gap(4.0),
    ))
    .justify(Justify::Between)
    .align(Align::Center)
    .padding(look(|l| Pad::all(l.gap())))
    .radius(look(|l| l.radius() * 1.6))
    .fill(look(|l| l.base().panel()))
    .border(1.0)
    .border_color(look(|l| l.base().border()))
}

/// [`card`] over [`controls`], under [`Live`].
#[wasm_bindgen]
pub struct HeroDemo {
    ui: Ui<Live>,
    live: Live,
    /// The look [`Live`] holds, to tell when the controls change it.
    shown: Look,
}

#[wasm_bindgen]
impl HeroDemo {
    #[wasm_bindgen(constructor)]
    pub fn new(container: HtmlElement) -> HeroDemo {
        let start = Look {
            skin: Skin::Monokai,
            round: false,
            roomy: false,
        };
        let live = Live(Arc::new(Mutex::new(start)));
        let mut ui = Ui::new(container, live.clone());
        ui.world.insert(start);
        ui.mount(
            column((card(), controls()))
                .gap(14.0)
                .transition(Motion::Interact),
        );
        HeroDemo {
            ui,
            live,
            shown: start,
        }
    }

    pub fn frame(&mut self, dt_ms: f64, reduced_motion: bool) {
        let look = *self.ui.world.get::<Look>();
        if look != self.shown {
            *self.live.0.lock().expect("an unpoisoned look") = look;
            self.shown = look;
            self.ui.restyle();
        }
        self.ui.frame(delta(dt_ms), reduced_motion);
    }

    /// Moves to the next skin, for the page to show them off.
    pub fn next_skin(&mut self) {
        let look = self.ui.world.get_mut::<Look>();
        let at = Skin::ALL.iter().position(|s| *s == look.skin);
        look.skin =
            Skin::ALL[(at.unwrap_or(0) + 1) % Skin::ALL.len()];
    }

    /// The skin showing, by name, for the page's own colours.
    pub fn skin(&self) -> String {
        self.shown.skin.name().to_lowercase()
    }
}
