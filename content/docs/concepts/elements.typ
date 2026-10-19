#import "@tola/source:0.0.0": tola-meta

#tola-meta((
  title: "Elements",
  summary: "An element is a struct of props that ends up on one node, with a Patch per prop that writes it there.",
  layout: "docs",
))

= Elements

An element is a struct of props that ends up on one node. Each prop
names the `Patch` that writes it to the node, and `#[element]` writes
the rest: the unset state, a builder method per prop, and the updates
when a bound prop changes.

The examples in these Concepts pages run against a toy backend from
#link("https://github.com/voxell-tech/fynix/blob/main/crates/fynix/docs/toy.rs")[`docs/toy.rs`]:
a flat list of nodes that hold text, and a `Ui` helper that builds
views and runs a frame. A real backend, such as
#link("/docs/bevy/")[Bevy Fynix], works the same way.

== Define an Element

```rust
trait TextTokens {
    fn body_size(&self) -> f32;
}

#[element(backend = Toy, theme = TextTokens)]
pub struct Text {
    #[elem(patch = WriteText)]
    text: Prop<World, String>,
    // Unset, the size comes from the theme.
    #[elem(default = theme.body_size(), patch = WriteSize)]
    size: Prop<World, f32>,
}

fn text(text: impl Into<Prop<World, String>>) -> Text {
    Text {
        text: text.into(),
        ..Text::unset()
    }
}
```

`theme = TextTokens` says which token trait the element reads, so it
works under any theme that implements it. A prop with a `default` takes
it from the theme when nothing else sets it.

== Write Props to the Node

A `Patch` is how one prop's value reaches the node.

```rust
pub struct WriteText;

impl Patch<Toy, String> for WriteText {
    fn patch(world: &mut World, node: usize, text: &String) {
        world.nodes[node].text.clone_from(text);
    }
}

pub struct WriteSize;

impl Patch<Toy, f32> for WriteSize {
    fn patch(world: &mut World, node: usize, size: &f32) {
        world.nodes[node].size = *size;
    }
}
```

== Build It

```rust
// A theme: any type that implements the token traits.
struct Warm;

impl TextTokens for Warm {
    fn body_size(&self) -> f32 {
        14.0
    }
}

fn main() {
    let mut ui = Ui::new(Warm);
    let node = ui.build(|cx| cx.build(text("Hello")));
    assert_eq!(ui.world.nodes[node].text, "Hello");
    assert_eq!(ui.size(node), 14.0);
}
```

== Bound Props

A prop can also be bound to the world, and it is read again only when
its source changes. In Bevy Fynix, `resource` binds a prop to a
resource:

```rust
label(resource::<Clicks, _>(|clicks| {
    format!("Counter reads {}", clicks.0)
}))
```
