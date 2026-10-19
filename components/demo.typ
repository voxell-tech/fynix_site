// Fynix running live in the page: js_fynix, a DOM backend compiled
// to wasm, mounts a demo into `#id`.
// Import: #import "/components/demo.typ": live-demo, showcase
//
// The shell carries `script` as `data-demo-src` rather than a
// `<script>` tag, so the kit's demo-bootstrap.js imports it on first
// load and after Tola's SPA navigation alike.

#import "/shared/site/ui.typ" as ui

/// The demo's mount point, inside its styled shell.
#let live-demo(id, script, caption: none) = html.div(
  class: "flex flex-col gap-2",
)[
  #html.elem("div", attrs: (
    class: "fynix-demo-shell",
    data-demo-src: script,
  ))[#html.div(id: id)[]]
  #if caption != none {
    html.p(class: "text-subtle text-xs text-center")[#caption]
  }
]

/// A landing page section: the demo's own Rust source beside it
/// running. `name` picks js_fynix/src/demos/<name>.rs,
/// assets/js/demos/<name>.js and the `<name>-demo` mount point.
#let showcase(name, title, description, caption: none, reverse: false) = ui.showcase-demo(
  title: title,
  description: description,
  code-label: "Rust",
  preview-label: "Running in Your Browser",
  code: ui.rust-snippet("/js_fynix/src/demos/" + name + ".rs"),
  preview: live-demo(
    name + "-demo",
    "/js/demos/" + name + ".js",
    caption: caption,
  ),
  reverse: reverse,
)
