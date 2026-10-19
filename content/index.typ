#import "@tola/source:0.0.0": tola-meta
#import "/shared/site/ui.typ" as ui
#import "/components/demo.typ": showcase, wasm-attrs

#tola-meta((
  title: none,
  summary: "Fynix is a backend agnostic reactive view tree for Rust, styled with set and show rules like a Typst document. Free and open-source forever.",
))

// Hero: the copy beside a live Fynix card (js_fynix's HeroDemo,
// mounted by assets/js/demos/hero.js) that restyles itself between
// three themes. Its backdrop follows the skin showing, through
// `data-skin`. Built in code mode: in markup, the whitespace between
// elements becomes spans that the grid and flex layouts pick up.
#html.elem("section", attrs: (class: "fx-hero", data-skin: "monokai"), {
  html.div(class: "fx-hero-glow")[]
  html.div(class: "fx-hero-inner", {
    html.div(class: "fx-hero-copy", {
      html.h1(class: "fx-hero-title", {
        [Fy]
        html.span(class: "fx-hero-nix")[nix]
      })
      html.p(class: "text-subtle text-lg sm:text-2xl mb-2")[
        A *backend agnostic* reactive view tree, styled the way Typst
        styles a document.
      ]
      html.p(class: "text-muted mb-8")[Free and open-source forever!]
      html.div(class: "flex flex-wrap gap-3 justify-center lg:justify-start", {
        ui.button("/docs/", large: true)[Get Started]
        ui.github-button("https://github.com/voxell-tech/fynix")
      })
    })
    html.elem("div", attrs: (
      class: "fx-hero-stage",
      data-demo-src: "/js/demos/hero.js",
      ..wasm-attrs,
    ), {
      html.div(id: "hero-demo")[]
      html.p(class: "fx-hero-hint")[
        Real Fynix, running in your browser. Pick a theme or flip a
        switch: the same views restyle.
      ]
    })
  })
})

#ui.hr

// Each section below is real Fynix: js_fynix, a DOM backend compiled
// to wasm, runs the code shown beside it.
#showcase(
  "themes",
  "One View, Any Theme",
  "A view reads token traits. Each card below is the same code under a different theme.",
)

#showcase(
  "rules",
  "Set and Show Rules",
  "Set rules fill what the call site left unset. Show rules win over it. Both end with their scope, as in Typst.",
  reverse: true,
)

#showcase(
  "states",
  "Hover It",
  [A state rule holds while its view is hovered or pressed, and `.transition(..)` makes the change glide.],
  caption: [Hover and press the buttons.],
)

#showcase(
  "each",
  "Lists That Animate Themselves",
  [`each` keeps a view for every key that stays. New rows make room and fade in; dropped ones fade out and close the gap.],
  caption: [Add, remove and reverse the rows.],
  reverse: true,
)

#ui.hr

#ui.feature(
  [Backend Agnostic],
  [
    A backend says what a world and a node are. Everything above it
    runs unchanged, which is how this page runs Fynix on the DOM.
  ],
  action: ui.feature-button(
    "/docs/advanced/",
  )[Build Your Own Backend →],
  media: ui.backend-diagram(
    "Fynix",
    (
      (label: "Bevy", ready: true),
      (label: "This Page", ready: true),
      (label: "Your Engine"),
    ),
    label: "Diagram: Fynix runs on Bevy and in this page's DOM, and your own engine can plug in the same way.",
    accent: (dark: rgb("#fc9867"), light: rgb("#b55e1e")),
  ),
)

#ui.feature(
  [Start With Bevy],
  [
    Labels, frames, buttons in ready-made looks, popups and docking
    panes, on bevy_ui.
  ],
  reverse: true,
  action: ui.feature-button(
    "/docs/bevy/",
    primary: true,
  )[Set Up Bevy Fynix →],
  media: html.elem("img", attrs: (
    src: "/icons/bevy.svg",
    alt: "Bevy",
    style: "height: 120px; width: auto;",
  )),
)

#ui.hr

#ui.cta(
  [Ready to Build?],
  ui.button("/docs/", large: true)[Get Started →],
)
