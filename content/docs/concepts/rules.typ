#import "@tola/source:0.0.0": tola-meta

#tola-meta((
  title: "Set and Show Rules",
  summary: "Set rules fill what a call site left unset, show rules transform every view of a kind, and both end with their scope, as in Typst.",
  layout: "docs",
))

= Set and Show Rules

Fynix styles a tree the way
#link("https://typst.app")[Typst] styles a document. A rule names a
kind of view and changes every view of that kind built after it, until
its scope ends.

== Set Rules

A set rule restyles every view of one kind built after it, like
Typst's `#set text(size: 12pt)`. It only fills what the call site left
unset.

```rust
fn main() {
    let mut ui = Ui::new(Warm);
    let (body, title) = ui.build(|cx| {
        cx.set::<Text>(|t, _| t.size(12.0));
        (
            cx.build(text("Body")),
            cx.build(text("Title").size(20.0)),
        )
    });
    assert_eq!(ui.size(body), 12.0);
    assert_eq!(ui.size(title), 20.0);
}
```

`Title` set its own size, so the rule left it alone.

== Show Rules

A show rule, like Typst's `#show`, transforms every view of one kind
after its call site, so it wins over a value set there.

```rust
fn main() {
    let mut ui = Ui::new(Warm);
    let title = ui.build(|cx| {
        cx.show::<Text>(|t, _| t.size(30.0));
        cx.build(text("Title").size(20.0))
    });
    assert_eq!(ui.size(title), 30.0);
}
```

== Scopes

Rules end with the scope they are set in.

```rust
fn main() {
    let mut ui = Ui::new(Warm);
    let (inside, after) = ui.build(|cx| {
        let inside = cx.scope(|cx| {
            cx.set::<Text>(|t, _| t.size(12.0));
            cx.build(text("Inside"))
        });
        (inside, cx.build(text("After")))
    });
    assert_eq!(ui.size(inside), 12.0);
    assert_eq!(ui.size(after), 14.0);
}
```

`After` is built outside the scope, so it falls back to the theme's
body size.

== Rules on a View

`.rules(..)` attaches a bundle of rules to one view, scoped to it. This
is how Bevy Fynix ships its button looks: `ghost`, `primary` and the
rest are rule bundles. See #link("/docs/concepts/styling/")[Styling]
for how they stack.
