//! What views here read from a theme, and the themes the demos use.

use core::time::Duration;

use fynix::{Curve, Motion, MotionTokens};

use crate::elements::{Color, Tone};

pub trait TextTokens {
    fn tone(&self, tone: Tone) -> Color;
    fn body_size(&self) -> f32;
}

pub trait SurfaceTokens {
    /// A card or panel.
    fn panel(&self) -> Color;
    /// A button at rest.
    fn fill(&self) -> Color;
    fn hover(&self) -> Color;
    fn pressed(&self) -> Color;
    fn accent(&self) -> Color;
    fn critical(&self) -> Color;
    fn border(&self) -> Color;
}

pub trait SpacingTokens {
    fn gap(&self) -> f32;
    fn radius(&self) -> f32;
    /// A button's padding, down. Across is half again.
    fn pad(&self) -> f32;
}

/// Every token trait the views here read.
pub trait Theme:
    TextTokens
    + SurfaceTokens
    + SpacingTokens
    + MotionTokens
    + Send
    + Sync
    + 'static
{
}

impl<T> Theme for T where
    T: TextTokens
        + SurfaceTokens
        + SpacingTokens
        + MotionTokens
        + Send
        + Sync
        + 'static
{
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

/// Quick for the pointer, slower for things coming and going.
fn motion(motion: Motion, slow: u64) -> Curve {
    match motion {
        Motion::Interact => Curve {
            duration: Duration::from_millis(160),
            ease: ease_out_cubic,
        },
        Motion::Expand => Curve {
            duration: Duration::from_millis(slow),
            ease: ease_in_out_cubic,
        },
    }
}

/// Dark, after Monokai Pro, like this site.
pub struct Monokai;

impl TextTokens for Monokai {
    fn tone(&self, tone: Tone) -> Color {
        match tone {
            Tone::Body => Color::hex(0xfcfcfa),
            Tone::Dim => Color::hex(0x939293),
            Tone::Accent => Color::hex(0xfc9867),
            Tone::OnAccent => Color::hex(0x19181a),
            Tone::Critical => Color::hex(0xff6188),
        }
    }

    fn body_size(&self) -> f32 {
        14.0
    }
}

impl SurfaceTokens for Monokai {
    fn panel(&self) -> Color {
        Color::hex(0x221f22)
    }

    fn fill(&self) -> Color {
        Color::hex(0x2d2a2e)
    }

    fn hover(&self) -> Color {
        Color::hex(0x403e41)
    }

    fn pressed(&self) -> Color {
        Color::hex(0x19181a)
    }

    fn accent(&self) -> Color {
        Color::hex(0xfc9867)
    }

    fn critical(&self) -> Color {
        Color::hex(0xff6188)
    }

    fn border(&self) -> Color {
        Color::hex(0xfcfcfa).alpha(0.1)
    }
}

impl SpacingTokens for Monokai {
    fn gap(&self) -> f32 {
        8.0
    }

    fn radius(&self) -> f32 {
        6.0
    }

    fn pad(&self) -> f32 {
        8.0
    }
}

impl MotionTokens for Monokai {
    fn motion(&self, kind: Motion) -> Curve {
        motion(kind, 260)
    }
}

/// Light and quiet, square-ish, blue.
pub struct Paper;

impl TextTokens for Paper {
    fn tone(&self, tone: Tone) -> Color {
        match tone {
            Tone::Body => Color::hex(0x1f2328),
            Tone::Dim => Color::hex(0x6e7781),
            Tone::Accent => Color::hex(0x0969da),
            Tone::OnAccent => Color::hex(0xffffff),
            Tone::Critical => Color::hex(0xcf222e),
        }
    }

    fn body_size(&self) -> f32 {
        13.0
    }
}

impl SurfaceTokens for Paper {
    fn panel(&self) -> Color {
        Color::hex(0xffffff)
    }

    fn fill(&self) -> Color {
        Color::hex(0xf3f4f6)
    }

    fn hover(&self) -> Color {
        Color::hex(0xe5e7eb)
    }

    fn pressed(&self) -> Color {
        Color::hex(0xd1d5db)
    }

    fn accent(&self) -> Color {
        Color::hex(0x0969da)
    }

    fn critical(&self) -> Color {
        Color::hex(0xcf222e)
    }

    fn border(&self) -> Color {
        Color::hex(0x1f2328).alpha(0.15)
    }
}

impl SpacingTokens for Paper {
    fn gap(&self) -> f32 {
        6.0
    }

    fn radius(&self) -> f32 {
        3.0
    }

    fn pad(&self) -> f32 {
        6.0
    }
}

impl MotionTokens for Paper {
    fn motion(&self, kind: Motion) -> Curve {
        motion(kind, 220)
    }
}

/// Big, round and sweet.
pub struct Candy;

impl TextTokens for Candy {
    fn tone(&self, tone: Tone) -> Color {
        match tone {
            Tone::Body => Color::hex(0x3b1d4a),
            Tone::Dim => Color::hex(0x8a6a99),
            Tone::Accent => Color::hex(0xe83e8c),
            Tone::OnAccent => Color::hex(0xffffff),
            Tone::Critical => Color::hex(0xd6336c),
        }
    }

    fn body_size(&self) -> f32 {
        16.0
    }
}

impl SurfaceTokens for Candy {
    fn panel(&self) -> Color {
        Color::hex(0xfde2f3)
    }

    fn fill(&self) -> Color {
        Color::hex(0xffffff)
    }

    fn hover(&self) -> Color {
        Color::hex(0xfff0fa)
    }

    fn pressed(&self) -> Color {
        Color::hex(0xf8c8e6)
    }

    fn accent(&self) -> Color {
        Color::hex(0xe83e8c)
    }

    fn critical(&self) -> Color {
        Color::hex(0xd6336c)
    }

    fn border(&self) -> Color {
        Color::hex(0xe83e8c).alpha(0.25)
    }
}

impl SpacingTokens for Candy {
    fn gap(&self) -> f32 {
        12.0
    }

    fn radius(&self) -> f32 {
        18.0
    }

    fn pad(&self) -> f32 {
        11.0
    }
}

impl MotionTokens for Candy {
    fn motion(&self, kind: Motion) -> Curve {
        motion(kind, 320)
    }
}
