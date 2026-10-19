#import "@tola/source:0.0.0": tola-meta
#import "/shared/site/ui.typ" as ui
#import "/components/demo.typ": showcase

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

// The Bevy gallery, straight under the hero: the first thing a
// visitor sees past the buttons is a real app built with Fynix.
#html.div(class: "max-w-5xl mx-auto px-4 -mt-6 mb-8")[
  #html.elem("img", attrs: (
    src: "/images/gallery.webp",
    alt: "The Bevy Fynix gallery example: buttons in each look, a keyed list, popups and docked panes",
    class: "rounded-xl border border-text/10 shadow-2xl shadow-accent/10",
    style: "width: 100%; height: auto;",
  ))
  #html.p(class: "text-subtle text-xs text-center mt-3")[
    The Bevy backend's gallery example.
  ]
]

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
