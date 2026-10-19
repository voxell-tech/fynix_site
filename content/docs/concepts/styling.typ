#import "@tola/source:0.0.0": tola-meta
#import "/components/visuals.typ": ladder, pieces

#tola-meta((
  title: "Styling",
  summary: "How theming, looks and state rules stay apart, the order every styling layer is applied in, and building a look of your own.",
  layout: "docs",
))

= Styling

Three things decide how a view looks, and they stay apart:

- *Theming* says what the colours, sizes and curves of an app are. A
  theme implements token traits. Views and styles ask for a role
  (`Tone::Dim`, `theme.hover()`), never a raw colour, so changing the
  theme restyles everything.
- *Styling* says which looks a view comes in: ghost, icon, primary. A
  look is a rule bundle, applied with `.rules(..)`. It is made of set
  rules, which restyle every `Frame`, `Label` or `Icon` built under it,
  with `cx.root(..)` keeping a rule to the view's root.
- *State rules* say what a look does while the pointer is on the view,
  while a button is down on it, or while any component of your own is
  on it: `.when::<Hovered, _>(..)`, `.when::<Pressed, _>(..)`.
  `.transition(Motion::Interact)` makes the change glide.

== Layers

#ladder((
  ([View default], [a theme token]),
  ([Composite defaults], [`cx.defaults(..)`]),
  ([Set rules, outer then inner], [`cx.set::<Label>(|l, _| l.size(12.0))`]),
  ([State rules from ancestors], [`row(..).when::<Hovered, _>(..)`]),
  ([Call site], [`label("x").size(20.0)`]),
  ([State rules on the node itself], [`label("x").when::<Hovered, _>(..)`]),
))

Show rules then transform the result. Of several state rules on one
node, the one in the outer scope wins, and within one scope the one
written later.

== Button Looks

Bevy Fynix's buttons ship with their looks as bundles. Each sets a
resting, a hovered and a pressed look from the theme, and glides
between them:

```rust
button(label("Cancel")).rules(ghost);
button(icon(save)).rules(icon_button);
button(label("Save")).rules(primary);
button(label("Delete")).rules(danger);
button(row((icon(tag), label("Tag")))).rules(tint);
```

== A Look of Your Own

For a look of your own, build a `Style` rather than writing `cx.set`
closures. It takes colours as closures over the theme, and a nested
look for each state:

```rust
let chunky = style::<MyTheme>()
    .fill(|t| t.panel())
    .frame(|f, t| f.radius(t.radius() * 3.0))
    .hovered(|s| s.fill(|t| t.fill()).tone(Tone::Accent))
    .pressed(|s| s.fill(|t| t.pressed()).tone(Tone::Critical))
    .transition(Motion::Interact);

button(label("Custom")).rules(chunky.bundle());
```

`fill` and `frame` reach the root frame alone, and `tone`, `label` and
`icon` reach every label and icon in the view. Later steps win, and a
press shows its hover look unless `pressed` says otherwise.

Styles join with `then`, and `with(ghost)` starts from a bundle.
Chained `.rules(a).rules(b)` also works, but as set rules go the inner
`a` wins over `b` for resting looks, while for state looks the outer
`b` wins. Use `then` when the order matters.
