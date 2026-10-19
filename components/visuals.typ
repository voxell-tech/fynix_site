// Diagrams for the docs, drawn in HTML so they follow the theme.
// Import: #import "/components/visuals.typ": ladder, pieces

/// A stack of layers from weakest (top) to strongest (bottom). Each
/// row's bar grows and brightens with its strength, so the order
/// reads at a glance. `rows` is an array of `(label, example)`.
#let ladder(rows) = html.div(class: "my-6 flex flex-col gap-1.5")[
  #html.div(
    class: "flex justify-between text-xs uppercase tracking-wide text-subtle mb-1",
  )[#html.span[Weakest]#html.span[Strongest]]
  #for (i, (label, example)) in rows.enumerate() {
    let strength = (i + 1) / rows.len()
    html.div(class: "relative rounded-lg overflow-hidden border border-text/10")[
      #html.div(
        class: "absolute inset-y-0 left-0 bg-accent",
        style: "width: "
          + str(calc.round(strength * 100))
          + "%; opacity: "
          + str(0.06 + 0.22 * strength)
          + ";",
      )[]
      #html.div(
        class: "relative flex flex-wrap items-center justify-between gap-x-4 gap-y-1 px-4 py-2.5",
      )[
        #html.span(class: "font-semibold text-sm")[#label]
        #html.span(class: "text-sm")[#example]
      ]
    ]
  }
]

/// Numbered tiles in a row (a column on small screens), for a short
/// sequence of parts. `items` is an array of `(title, body, href)`;
/// `href` may be `none`.
/// safelist: sm:grid-cols-3 sm:grid-cols-4 sm:grid-cols-5
#let pieces(items) = html.div(
  class: "my-6 grid gap-3 sm:grid-cols-" + str(items.len()),
)[
  #for (i, (title, body, href)) in items.enumerate() {
    let tile = html.div(
      class: "h-full rounded-lg border border-text/10 bg-surface/50 p-3 transition-colors hover:border-accent/50",
    )[
      #html.div(
        class: "flex items-center justify-center w-7 h-7 mb-2 rounded-full bg-accent text-bg text-sm font-bold",
      )[#str(i + 1)]
      #html.div(class: "font-semibold text-sm mb-1")[#title]
      #html.div(class: "text-xs text-muted")[#body]
    ]
    if href != none {
      html.a(class: "no-underline text-text", href: href)[#tile]
    } else { tile }
  }
]
