#import "@tola/source:0.0.0": tola-meta

#tola-meta((
  title: "Views",
  summary: "Elements, composites and wrappers; keyed and each for structure that changes; and transitions that animate it.",
  layout: "docs",
))

= Views

A view is a struct that owns its props. There are three kinds:

- *Elements* are one node each, written by the backend: `Label`,
  `Frame`, `Icon`.
- *Composites* are built out of other views: `row`, `column`,
  `button`, `foldable`. Their parts are ordinary views, so ordinary
  rules reach them.
- *Wrappers* add something to any view: `.when(..)`,
  `.transition(..)`, `.rules(..)`, `.on_activate(..)`.

Because a composite's parts are ordinary views, a set rule for `Label`
reaches the label inside a `button` the same way it reaches one on its
own.

== Changing Structure

Most of a tree stays put, and only its props change. `keyed` and `each`
build and drop parts of the tree as the world changes, and keep every
view whose key stays.

`keyed` builds a view again when its key changes:

```rust
keyed(resource::<Screen, _>(|screen| *screen), |screen| {
    match screen {
        Screen::Overview => label("Overview").boxed(),
        Screen::Details => label("Details").boxed(),
    }
})
.within(column(()))
```

`each` builds one view per item, keyed by the function in the middle:

```rust
each(
    resource::<Rows, _>(|rows| rows.ids.clone()),
    |id| *id,
    |id| label(format!("Row {id}")).boxed(),
)
.within(column(()).gap(4.0))
```

A row whose id stays keeps its node, so a transition under way on it
survives a reorder.

== Transitions

Add `.transition(..)` to a view and its changes animate over a curve
from the theme's `MotionTokens`.

When `keyed` or `each` drops a view, the old view fades out and its
space collapses. A new view's space expands first, and then it fades
in.

== Reduced Motion

`Tick::reduced_motion` skips every step: transitions finish at once,
and views come in and leave without fading or resizing. In Bevy Fynix,
the `ReducedMotion` resource sets it.
