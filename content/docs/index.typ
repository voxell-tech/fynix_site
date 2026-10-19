#import "@tola/source:0.0.0": tola-meta
#import "/shared/site/ui.typ" as ui

#tola-meta((
  title: "Choose a Backend",
  summary: "Pick a Fynix backend to start with: Bevy Fynix for Bevy games and apps, or build your own UI framework on any other engine.",
  layout: "docs",
))

= Choose a Backend

Fynix is a reactive view tree. A backend says what a world and a node
are, and writes the elements to them. Everything above the elements,
from composites to rules, themes and transitions, is the same on every
backend.

#ui.grid(
  cols: 2,
  gap: 4,
  ui.card(title: [#link("/docs/bevy/")[Bevy Fynix →]])[
    #html.p(class: "text-muted text-sm")[
      For Bevy games and apps. Nodes are entities, and it ships labels,
      frames, icons, buttons, menus and docking on top of `bevy_ui`.
    ]
  ],
  html.div(
    class: "flex flex-col items-start justify-center p-4 rounded-lg border border-dashed border-text/15",
  )[
    #html.p(class: "font-semibold text-subtle mb-1")[More coming soon]
    #html.p(class: "text-muted text-sm")[
      Other engines and renderers. Building your own now? See
      #link("/docs/advanced/")[Building a UI Framework].
    ]
  ],
)

== Then Learn the Concepts

Once a view is on screen, #link("/docs/concepts/elements/")[Concepts]
covers how Fynix builds and styles a tree. It holds on every backend.
