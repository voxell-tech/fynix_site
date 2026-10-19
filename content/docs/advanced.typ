#import "@tola/source:0.0.0": tola-meta

#tola-meta((
  title: "Building a UI Framework",
  summary: "Build a UI framework on Fynix for any engine: a backend, elements, token traits, states, and a mount.",
  layout: "docs",
))

= Building a UI Framework

Fynix is the part of a UI framework that does not depend on an engine.
A framework on top of it fills in five things, and
#link("/docs/bevy/")[Bevy Fynix] is a full example of each.

== The Five Pieces

+ *A backend*, which says what a world and a node are. `spawn`,
  `despawn` and `reorder` are required. `on_mount`, `leave`, `hold`,
  `collapse` and `release` have empty defaults. Cleanup, input
  blocking and the animation of a view's space go there: a dropped
  view is `leave`d, then `collapse`d from 0 to 1, and a new one is
  `hold`ed out of the layout, `collapse`d from 1 to 0 once its size
  can be measured, then `release`d.
+ *Elements*. For each prop you say how its value is written to a node
  (`Patch`), and optionally its default from the theme. See
  #link("/docs/concepts/elements/")[Elements].
+ *Token traits*. Elements and composites bound themselves on the
  tokens they read, and your framework's users bring their own themes.
  See #link("/docs/concepts/themes/")[Tokens and Themes].
+ *States*, which implement `Condition`: whether the state holds on a
  node, and a request to the backend to report the node to
  `Mounted::mark_dirty` whenever that changes. See
  #link("/docs/concepts/states/")[State Rules].
+ *A mount*, which builds a view at the root with a `Cx`, and a frame
  loop that calls `update_structure` and `update_elements`.

`#[element]` finds the names it uses at the root of the crate it is
used in, so a framework re-exports them from its own root.

== All Five in One File

```rust
use core::time::Duration;

// `#[element]` finds these at the root of the crate it is used in,
// so a framework re-exports them from its own root.
use fynix::{
    Backend, Condition, Curve, Cx, Element, Motion, MotionTokens,
    Mounted, Patch, Prop, ScopedExt, Slot, Styled, Tick, View,
    ViewExt, element, styled,
};

// 1. A backend.
#[derive(Default)]
pub struct World {
    nodes: Vec<Node>,
}

#[derive(Default)]
struct Node {
    parent: Option<usize>,
    children: Vec<usize>,
    text: String,
    size: f32,
    hovered: bool,
}

struct Toy;

impl Backend for Toy {
    type World = World;
    type Node = usize;

    fn spawn(world: &mut World, parent: Option<usize>) -> usize {
        let node = world.nodes.len();
        world.nodes.push(Node {
            parent,
            ..Node::default()
        });
        if let Some(parent) = parent {
            world.nodes[parent].children.push(node);
        }
        node
    }

    fn despawn(world: &mut World, node: usize) {
        // The node stays in the list, only unlinked from its parent.
        if let Some(parent) = world.nodes[node].parent {
            world.nodes[parent].children.retain(|&c| c != node);
        }
    }

    fn reorder(world: &mut World, parent: usize, children: &[usize]) {
        world.nodes[parent].children = children.to_vec();
    }
}

// 3. Token traits.
trait TextTokens {
    fn body_size(&self) -> f32;
}

// 2. An element, reading `TextTokens` from any theme.
#[element(backend = Toy, theme = TextTokens)]
pub struct Text {
    #[elem(patch = WriteText)]
    text: Prop<World, String>,
    /// The theme's body size when unset.
    #[elem(default = theme.body_size(), patch = WriteSize)]
    size: Prop<World, f32>,
}

fn text(text: impl Into<Prop<World, String>>) -> Text {
    Text {
        text: text.into(),
        ..Text::unset()
    }
}

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

// 4. A state.
struct Hovered;

impl Condition<Toy> for Hovered {
    fn holds(world: &World, node: usize) -> bool {
        world.nodes[node].hovered
    }

    fn watch(_: &mut World, _: usize) {
        // A real backend hooks its hover events up to `mark_dirty`.
    }
}

// A theme, which any app can write its own of.
struct Warm;

impl TextTokens for Warm {
    fn body_size(&self) -> f32 {
        14.0
    }
}

impl MotionTokens for Warm {
    fn motion(&self, _: Motion) -> Curve {
        Curve {
            duration: Duration::from_millis(100),
            ease: |t| t,
        }
    }
}

// 5. Mount a view, then update it from the frame loop.
fn main() {
    let mut world = World::default();
    let mut mounted = Mounted::<Toy, Warm>::default();
    let node = {
        let mut cx = Cx::new(&mut world, &Warm, &mut mounted);
        text("Save")
            // Every text under this one grows while it is hovered.
            .when_in::<Hovered, _>(|cx: &mut Cx<Toy, Warm>| {
                cx.set::<Text>(|t, _| t.size(20.0));
            })
            .build(&mut cx)
    };

    let tick = Tick {
        delta: Duration::from_millis(16),
        reduced_motion: false,
    };
    mounted.update_structure(&mut world, &Warm);
    mounted.update_elements(&mut world, &Warm, tick);
    assert_eq!(world.nodes[node].text, "Save");
    assert_eq!(world.nodes[node].size, 14.0);

    world.nodes[node].hovered = true;
    mounted.mark_dirty(node);
    mounted.update_elements(&mut world, &Warm, tick);
    assert_eq!(world.nodes[node].size, 20.0);
}
```
