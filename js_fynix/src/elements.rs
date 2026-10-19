//! The two elements every view here is made of: `Label`, a run of
//! text, and `Frame`, a flex box with a fill and a border.

use fynix::{Patch, Styled, element};

use crate::backend::{Node, Web, World};
use crate::tokens::{SpacingTokens, TextTokens};

/// A prop of a view built into [`Web`].
pub type Prop<T> = fynix::Prop<World, T>;

/// A colour in sRGB, with alpha.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const NONE: Self = Self::rgba(0, 0, 0, 0.0);

    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a,
        }
    }

    /// `0xRRGGBB`, opaque.
    pub const fn hex(hex: u32) -> Self {
        Self::rgba(
            (hex >> 16) as u8,
            (hex >> 8) as u8,
            hex as u8,
            1.0,
        )
    }

    pub const fn alpha(self, a: f32) -> Self {
        Self { a, ..self }
    }

    /// `amount` of the way to white.
    pub fn lighten(self, amount: f32) -> Self {
        blend_color(&self, &Self::hex(0xffffff).alpha(self.a), amount)
    }

    fn css(&self) -> String {
        let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round();
        format!(
            "rgba({},{},{},{})",
            byte(self.r),
            byte(self.g),
            byte(self.b),
            self.a.clamp(0.0, 1.0)
        )
    }
}

/// How two numbers blend, for `#[elem(blend = ..)]`.
pub fn blend_f32(from: &f32, to: &f32, t: f32) -> f32 {
    from + (to - from) * t
}

/// How two colours blend, channel by channel.
pub fn blend_color(from: &Color, to: &Color, t: f32) -> Color {
    Color {
        r: blend_f32(&from.r, &to.r, t),
        g: blend_f32(&from.g, &to.g, t),
        b: blend_f32(&from.b, &to.b, t),
        a: blend_f32(&from.a, &to.a, t),
    }
}

/// A colour by its role, which the theme turns into a colour.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Body,
    Dim,
    Accent,
    OnAccent,
    Critical,
}

/// Which way a frame lays its children out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    Row,
    #[default]
    Column,
}

/// Where children sit across the frame's direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    Center,
    #[default]
    Stretch,
}

/// Where children sit along the frame's direction.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Justify {
    #[default]
    Start,
    Center,
    End,
    Between,
}

/// Padding across (`x`) and down (`y`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Pad {
    pub x: f32,
    pub y: f32,
}

impl Pad {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn all(all: f32) -> Self {
        Self { x: all, y: all }
    }
}

fn blend_pad(from: &Pad, to: &Pad, t: f32) -> Pad {
    Pad {
        x: blend_f32(&from.x, &to.x, t),
        y: blend_f32(&from.y, &to.y, t),
    }
}

/// A run of text.
#[element(backend = Web, theme = TextTokens, prepare = prepare_label)]
pub struct Label {
    #[elem(patch = PatchText)]
    pub text: Prop<String>,
    /// The theme's body size when unset.
    #[elem(
        default = theme.body_size(),
        patch = PatchTextSize,
        blend = blend_f32
    )]
    pub size: Prop<f32>,
    /// The colour, by role. Body when unset.
    #[elem(
        shown = Color,
        with = |tone, theme| theme.tone(tone),
        patch = PatchInk,
        blend = blend_color
    )]
    pub tone: Prop<Tone>,
    #[elem(patch = PatchBold)]
    pub bold: Prop<bool>,
    /// How opaque it is, 1.0 when unset.
    #[elem(default = 1.0, patch = PatchOpacity, blend = blend_f32)]
    pub opacity: Prop<f32>,
    /// The factor it is scaled by around its centre, 1.0 when unset.
    #[elem(default = 1.0, patch = PatchScale, blend = blend_f32)]
    pub scale: Prop<f32>,
}

pub fn label(text: impl Into<Prop<String>>) -> Label {
    Label {
        text: text.into(),
        ..Label::unset()
    }
}

fn prepare_label(world: &mut World, node: Node) {
    if let Some(el) = world.el(node) {
        let _ = el.class_list().add_1("fx-label");
    }
}

/// A flex box with a fill, a border and a shadow.
#[element(backend = Web, theme = SpacingTokens, prepare = prepare_frame)]
pub struct Frame {
    #[elem(patch = PatchDirection)]
    pub direction: Prop<Direction>,
    /// Between children. The theme's gap when unset.
    #[elem(default = theme.gap(), patch = PatchGap, blend = blend_f32)]
    pub gap: Prop<f32>,
    #[elem(patch = PatchPadding, blend = blend_pad)]
    pub padding: Prop<Pad>,
    #[elem(patch = PatchAlign)]
    pub align: Prop<Align>,
    #[elem(patch = PatchJustify)]
    pub justify: Prop<Justify>,
    /// How much of the free space it takes, none when unset.
    #[elem(patch = PatchGrow)]
    pub grow: Prop<f32>,
    /// Transparent when unset.
    #[elem(default = Color::NONE, patch = PatchFill, blend = blend_color)]
    pub fill: Prop<Color>,
    #[elem(patch = PatchRadius, blend = blend_f32)]
    pub radius: Prop<f32>,
    /// The width of a border on every edge, with `border_color`.
    #[elem(patch = PatchBorder, blend = blend_f32)]
    pub border: Prop<f32>,
    #[elem(
        default = Color::NONE,
        patch = PatchBorderColor,
        blend = blend_color
    )]
    pub border_color: Prop<Color>,
    /// How far it seems to float: the blur of its shadow.
    #[elem(patch = PatchShadow, blend = blend_f32)]
    pub shadow: Prop<f32>,
    /// How opaque it is, 1.0 when unset.
    #[elem(default = 1.0, patch = PatchOpacity, blend = blend_f32)]
    pub opacity: Prop<f32>,
    /// The factor it is scaled by around its centre, 1.0 when unset.
    /// Its children are scaled with it.
    #[elem(default = 1.0, patch = PatchScale, blend = blend_f32)]
    pub scale: Prop<f32>,
}

pub fn frame() -> Frame {
    Frame::unset()
}

fn prepare_frame(world: &mut World, node: Node) {
    if let Some(el) = world.el(node) {
        let _ = el.class_list().add_1("fx-frame");
    }
}

/// Writes one prop as inline styles, through `$write`.
macro_rules! patch {
    ($name:ident, $value:ty, |$style:ident, $v:ident| $write:expr) => {
        pub struct $name;

        impl Patch<Web, $value> for $name {
            fn patch(world: &mut World, node: Node, $v: &$value) {
                if let Some($style) = world.style(node) {
                    let _ = $write;
                }
            }
        }
    };
}

fn px(value: f32) -> String {
    format!("{value}px")
}

pub struct PatchText;

impl Patch<Web, String> for PatchText {
    fn patch(world: &mut World, node: Node, text: &String) {
        if let Some(el) = world.el(node) {
            el.set_text_content(Some(text));
        }
    }
}

patch!(PatchTextSize, f32, |style, v| style
    .set_property("font-size", &px(*v)));
patch!(PatchInk, Color, |style, v| style
    .set_property("color", &v.css()));
patch!(PatchBold, bool, |style, v| style
    .set_property("font-weight", if *v { "650" } else { "450" }));
patch!(PatchOpacity, f32, |style, v| style
    .set_property("opacity", &v.to_string()));
patch!(PatchScale, f32, |style, v| style
    .set_property("transform", &format!("scale({v})")));
patch!(PatchDirection, Direction, |style, v| style.set_property(
    "flex-direction",
    match v {
        Direction::Row => "row",
        Direction::Column => "column",
    }
));
patch!(PatchGap, f32, |style, v| style.set_property("gap", &px(*v)));
patch!(PatchPadding, Pad, |style, v| style
    .set_property("padding", &format!("{}px {}px", v.y, v.x)));
patch!(PatchAlign, Align, |style, v| style.set_property(
    "align-items",
    match v {
        Align::Center => "center",
        Align::Stretch => "stretch",
    }
));
patch!(PatchJustify, Justify, |style, v| style.set_property(
    "justify-content",
    match v {
        Justify::Start => "flex-start",
        Justify::Center => "center",
        Justify::End => "flex-end",
        Justify::Between => "space-between",
    }
));
patch!(PatchGrow, f32, |style, v| style
    .set_property("flex-grow", &v.to_string()));
patch!(PatchFill, Color, |style, v| style
    .set_property("background-color", &v.css()));
patch!(PatchRadius, f32, |style, v| style
    .set_property("border-radius", &px(*v)));
patch!(PatchBorder, f32, |style, v| style
    .set_property("border-width", &px(*v)));
patch!(PatchBorderColor, Color, |style, v| style
    .set_property("border-color", &v.css()));
patch!(PatchShadow, f32, |style, v| style.set_property(
    "box-shadow",
    &if *v <= 0.0 {
        "none".to_string()
    } else {
        format!(
            "0 {}px {}px rgba(0,0,0,{})",
            v / 3.0,
            v,
            (v / 60.0).min(0.35)
        )
    }
));
