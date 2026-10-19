#import "@tola/source:0.0.0": tola-meta
#import "/shared/site/ui.typ" as ui

#tola-meta((
  title: none,
  summary: "Fynix is a backend agnostic reactive view tree for Rust, styled with set and show rules like a Typst document. Free and open-source forever.",
))

#ui.hero(
  [Fy#html.span(class: "text-accent")[nix]],
  tagline: [
    A *backend agnostic* reactive view tree, styled the way Typst
    styles a document.
  ],
  subtitle: [Free and open-source forever!],
  actions: (
    ui.button("/docs/", large: true)[Get Started],
    ui.github-button("https://github.com/voxell-tech/fynix"),
  ),
)

#ui.hr

// Feature: set and show rules
#ui.feature(
  [Set and Show Rules],
  [
    A set rule restyles every view of one kind built after it, and
    fills only what the call site left unset. A show rule wins over
    the call site. Both end with the scope they are set in, as in
    Typst.
  ],
  action: ui.feature-button("/docs/concepts/rules/")[Read About Rules →],
  media: ui.code-panel[
    ```rust
    cx.scope(|cx| {
        // Every label in this scope is small...
        cx.set::<Label>(|l, _| l.size(12.0));
        // ...unless the call site says otherwise.
        cx.build(label("Title").size(20.0));
    });
    ```
  ],
)

// Feature: state rules
#ui.feature(
  [States Are Rules Too],
  [
    A state rule holds while its view is hovered, pressed, entering,
    leaving, or in any state of your own, and reaches every view under
    it. Add `.transition(..)` and the change glides.
  ],
  reverse: true,
  action: ui.feature-button(
    "/docs/concepts/states/",
  )[Read About States →],
  media: ui.code-panel[
    ```rust
    button(label("Save"))
        // Every label in the button turns accent on hover.
        .when::<Hovered, _>(|cx: &mut Cx<Bevy, MyTheme>| {
            cx.set::<Label>(|l, _| l.tone(Tone::Accent));
        })
        .transition(Motion::Interact)
    ```
  ],
)

// Feature: tokens and themes
#ui.feature(
  [Themes Are Traits],
  [
    A view asks for the token traits it reads, never a raw colour. A
    theme is any type that implements them, so swapping it restyles
    every view at once.
  ],
  action: ui.feature-button(
    "/docs/concepts/themes/",
  )[Read About Themes →],
  media: ui.code-panel[
    ```rust
    trait TextTokens {
        fn body_size(&self) -> f32;
    }

    struct Large;

    impl TextTokens for Large {
        fn body_size(&self) -> f32 {
            18.0
        }
    }
    ```
  ],
)

// Feature: keyed and each
#ui.feature(
  [In and Out, Animated],
  [
    `keyed` and `each` rebuild parts of the tree as the world changes,
    keeping every view whose key stays. A dropped view fades out and
    its space collapses; a new one makes room, then fades in.
  ],
  reverse: true,
  action: ui.feature-button("/docs/concepts/views/")[Read About Views →],
  media: ui.code-panel[
    ```rust
    each(
        resource::<Rows, _>(|rows| rows.ids.clone()),
        |id| *id,
        |id| line(&format!("Row {id}")).boxed(),
    )
    .within(column(()).gap(4.0))
    ```
  ],
)

#ui.hr

// Feature: backend agnostic
#ui.feature(
  [Backend Agnostic],
  [
    A backend says what a world and a node are, and writes the
    elements. Everything above them, from composites to rules, themes
    and transitions, is the same on every backend. The core is
    `#![no_std]`, with `alloc` only.
  ],
  action: ui.feature-button(
    "/docs/advanced/",
  )[Build Your Own Backend →],
  media: ui.backend-diagram(
    "Fynix",
    (
      (label: "Bevy", ready: true),
      (label: "Your Engine"),
      (label: "..."),
    ),
    label: "Diagram: Fynix connects to Bevy today, with your own engine or others pluggable in the same way.",
    accent: (dark: rgb("#fc9867"), light: rgb("#b55e1e")),
  ),
)

#ui.hr

// Feature: the Bevy backend, with its gallery. Full width below the
// text, like MotionGfx's Moxie section: the screenshot's own labels
// need real width to read at all.
#html.section(class: "py-12 px-4 sm:px-6")[
  #ui.section-heading(
    [Start With Bevy],
    [
      Bevy Fynix ships labels, frames and icons on `bevy_ui`, rows,
      columns, buttons in ready-made looks, popups and docking panes.
      Run `cargo run -p bevy_fynix --example gallery` to see every
      idea in a window.
    ],
    action: ui.feature-button(
      "/docs/bevy/",
      primary: true,
    )[Set Up Bevy Fynix →],
  )
  #html.div[
    #html.elem("img", attrs: (
      src: "/images/gallery.webp",
      alt: "The Bevy Fynix gallery example, showing buttons in each look, a keyed list, popups and docked panes",
      class: "rounded-lg",
      style: "width: 100%; height: auto;",
    ))
  ]
]

#ui.hr

// Closing CTA
#ui.cta(
  [Ready to Build?],
  ui.button("/docs/", large: true)[Get Started →],
)
