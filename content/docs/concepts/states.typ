#import "@tola/source:0.0.0": tola-meta

#tola-meta((
  title: "State Rules",
  summary: "A state rule holds only while its view is in a state, such as hovered or pressed, and reaches every view under it.",
  layout: "docs",
))

= State Rules

A state rule holds only while its view is in a state, such as hovered.
Rules set inside it reach every view under that view.

```rust
fn main() {
    let mut ui = Ui::new(Warm);
    let node = ui.build(|cx| {
        text("Save")
            .when_in::<Hovered, _>(|cx: &mut Cx<Toy, Warm>| {
                cx.set::<Text>(|t, _| t.size(20.0));
            })
            .build(cx)
    });
    assert_eq!(ui.size(node), 14.0);

    ui.world.nodes[node].hovered = true;
    ui.mounted.mark_dirty(node);
    ui.frame();
    assert_eq!(ui.size(node), 20.0);
}
```

Nothing is rebuilt when the state changes. The backend reports the
node to `mark_dirty`, and the next frame writes the props the rule
changed.

== States

A state is a type that implements `Condition`: whether the state holds
on a node, and a request to the backend to report the node whenever
that changes.

```rust
struct Hovered;

impl Condition<Toy> for Hovered {
    fn holds(world: &World, node: usize) -> bool {
        world.nodes[node].hovered
    }

    fn watch(_: &mut World, _: usize) {
        // A real backend hooks its hover events up to `mark_dirty`.
    }
}
```

== States in Bevy

Bevy Fynix's states are components: `Hovered`, `Pressed`, `Focused`,
`Dragging`, `Entering` and `Leaving`, and any component of your own
works too. `.when::<S, _>(..)` holds while the view's root has the
component `S`.

```rust
button(label("Save"))
    // Every label in the button turns accent on hover.
    .when::<Hovered, _>(|cx: &mut Cx<Bevy, MyTheme>| {
        cx.set::<Label>(|l, _| l.tone(Tone::Accent));
    })
    .transition(Motion::Interact)
```

On the root, a state rule beats its call site. On a view under it, it
only fills what that view's call site left unset.

== Coming In and Going Out

`.appear(away)` uses `Entering` and `Leaving` to animate a view into
the tree from the look `away` sets, and back out to it. `hidden` is a
ready-made `away`: every element transparent and the root scaled down a
little. It needs a transition to move.

```rust
line(name).appear::<MyTheme>(hidden).transition(Motion::Expand)
```
