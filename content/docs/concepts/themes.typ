#import "@tola/source:0.0.0": tola-meta

#tola-meta((
  title: "Tokens and Themes",
  summary: "A token trait is all a view knows about a theme, so swapping the theme restyles every view that reads it.",
  layout: "docs",
))

= Tokens and Themes

A token trait, like `TextTokens` on #link("/docs/concepts/elements/")[Elements],
is all a view knows about a theme. A theme is any type that implements
the token traits its views read, so swapping the theme restyles every
view that reads it.

```rust
struct Large;

impl TextTokens for Large {
    fn body_size(&self) -> f32 {
        18.0
    }
}

fn main() {
    let mut ui = Ui::new(Large);
    let node = ui.build(|cx| cx.build(text("Hello")));
    assert_eq!(ui.size(node), 18.0);
}
```

== Ask for Roles, Not Values

Tokens name a role, never a raw value: `Tone::Dim` for secondary text,
`theme.hover()` for a hovered surface. A view that only says which role
it plays looks right under every theme.

Bevy Fynix's views read four token traits:

- `TextTokens`: text colours by `Tone`, and the body and small sizes.
- `SurfaceTokens`: fills for surfaces, hovers, panels and the accent.
- `SpacingTokens`: gaps, row heights and corner radii.
- `MotionTokens`: the `Curve` for each kind of `Motion`.

Many tokens have defaults built from others, so a theme implements only
the few it has to. #link("/docs/bevy/")[Bevy Fynix] has a whole theme
in one block.

== Motion Is a Token Too

How long a transition takes, and how it eases, comes from the theme's
`MotionTokens`, like any other token.

```rust
impl MotionTokens for Warm {
    fn motion(&self, _: Motion) -> Curve {
        Curve {
            duration: Duration::from_millis(100),
            ease: |t| t,
        }
    }
}
```

`Motion::Interact` is for answering the pointer, such as a hover or a
press, and `Motion::Expand` is for something opening or growing.
