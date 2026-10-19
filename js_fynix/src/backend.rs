//! The backend: a node is a `<div>` in the page, and the world owns
//! every one of them, the pointer events they report, and the
//! resources the demos keep their data in.

use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use fynix::{Backend, Condition, Signal};
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CssStyleDeclaration, Document, HtmlElement};

/// Fynix, building into the DOM.
pub struct Web;

/// A node's id. Ids are never handed out twice, so one held after
/// its node is gone never names another.
pub type Node = u32;

/// What the pointer did to a node since the last frame.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    Enter(Node),
    Leave(Node),
    Down(Node),
    Up(Node),
}

/// Where listeners put what they saw, for the next frame to read.
pub type Queue = Rc<RefCell<Vec<Event>>>;

/// What a click on a node runs.
pub type Click = Rc<dyn Fn(&mut World)>;

type Listener = Closure<dyn FnMut(web_sys::PointerEvent)>;

/// Makes the event a listener reports for its node.
type ToEvent = fn(Node) -> Event;

/// How a node sits in its parent, measured when its space first
/// collapses or expands.
#[derive(Clone, Copy, Debug)]
struct Collapsing {
    size: f32,
    gap: f32,
    row: bool,
    first: bool,
    padding: (f32, f32),
}

/// One node of the page.
struct Data {
    el: HtmlElement,
    parent: Option<Node>,
    children: Vec<Node>,
    hovered: bool,
    pressed: bool,
    entering: bool,
    leaving: bool,
    /// Whether `Entering` was asked for, so it is set once.
    watched_entering: bool,
    held: bool,
    collapsing: Option<Collapsing>,
    /// The inline styles a hold or a collapse changed, with what
    /// they were, to put back on release.
    saved: Vec<(&'static str, String)>,
    listeners: Vec<Listener>,
}

pub struct World {
    doc: Document,
    root: HtmlElement,
    nodes: HashMap<Node, Data>,
    next: Node,
    pub(crate) queue: Queue,
    /// Nodes whose state changed during an update, to mark dirty
    /// before the next.
    pub(crate) dirty: Vec<Node>,
    /// Nodes that took `Entering` since the last frame.
    pub(crate) entrances: Vec<Node>,
    pub(crate) clicks: HashMap<Node, Click>,
    resources: HashMap<TypeId, (Box<dyn Any>, u64)>,
}

impl World {
    /// A world building into `root`.
    pub fn new(root: HtmlElement) -> Self {
        let doc = root.owner_document().expect("a document");
        Self {
            doc,
            root,
            nodes: HashMap::new(),
            next: 0,
            queue: Rc::default(),
            dirty: Vec::new(),
            entrances: Vec::new(),
            clicks: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    /// Every node built into this world.
    pub(crate) fn nodes(&self) -> Vec<Node> {
        self.nodes.keys().copied().collect()
    }

    pub(crate) fn el(&self, node: Node) -> Option<&HtmlElement> {
        self.nodes.get(&node).map(|data| &data.el)
    }

    pub(crate) fn style(
        &self,
        node: Node,
    ) -> Option<CssStyleDeclaration> {
        self.el(node).map(|el| el.style())
    }

    /// Adds `resource`, or replaces the one of its type.
    pub fn insert<R: 'static>(&mut self, resource: R) {
        let version = self.version::<R>() + 1;
        self.resources
            .insert(TypeId::of::<R>(), (Box::new(resource), version));
    }

    pub fn get<R: 'static>(&self) -> &R {
        self.resources[&TypeId::of::<R>()]
            .0
            .downcast_ref()
            .expect("a resource of its own type")
    }

    /// The resource `R`, marked changed for every signal reading it.
    pub fn get_mut<R: 'static>(&mut self) -> &mut R {
        let (resource, version) = self
            .resources
            .get_mut(&TypeId::of::<R>())
            .expect("an inserted resource");
        *version += 1;
        resource.downcast_mut().expect("a resource of its own type")
    }

    fn version<R: 'static>(&self) -> u64 {
        self.resources
            .get(&TypeId::of::<R>())
            .map_or(0, |(_, version)| *version)
    }

    /// Reports the pointer on `node` to the queue, once.
    pub(crate) fn listen(&mut self, node: Node) {
        let queue = self.queue.clone();
        let Some(data) = self.nodes.get_mut(&node) else {
            return;
        };
        if !data.listeners.is_empty() {
            return;
        }
        let events: [(&str, ToEvent); 4] = [
            ("pointerenter", Event::Enter),
            ("pointerleave", Event::Leave),
            ("pointerdown", Event::Down),
            ("pointerup", Event::Up),
        ];
        for (name, event) in events {
            let queue = queue.clone();
            let listener: Listener = Closure::new(move |_| {
                queue.borrow_mut().push(event(node));
            });
            let _ = data.el.add_event_listener_with_callback(
                name,
                listener.as_ref().unchecked_ref(),
            );
            data.listeners.push(listener);
        }
    }

    /// Applies what the pointer did, and returns the nodes whose
    /// state changed and the clicks to run.
    pub(crate) fn drain(&mut self) -> (Vec<Node>, Vec<Click>) {
        let events = core::mem::take(&mut *self.queue.borrow_mut());
        let mut changed = core::mem::take(&mut self.dirty);
        let mut clicks = Vec::new();
        for event in events {
            let (Event::Enter(node)
            | Event::Leave(node)
            | Event::Down(node)
            | Event::Up(node)) = event;
            let Some(data) = self.nodes.get_mut(&node) else {
                continue;
            };
            if data.leaving {
                continue;
            }
            match event {
                Event::Enter(_) => data.hovered = true,
                Event::Leave(_) => {
                    data.hovered = false;
                    data.pressed = false;
                }
                Event::Down(_) => data.pressed = true,
                Event::Up(_) => {
                    if data.pressed
                        && let Some(click) = self.clicks.get(&node)
                    {
                        clicks.push(click.clone());
                    }
                    data.pressed = false;
                }
            }
            changed.push(node);
        }
        (changed, clicks)
    }

    /// Takes `Entering` off every node written with it once, and
    /// returns them to be marked dirty.
    pub(crate) fn settle(&mut self) -> Vec<Node> {
        let entered = core::mem::take(&mut self.entrances);
        entered
            .into_iter()
            .filter(|node| {
                self.nodes.get_mut(node).is_some_and(|data| {
                    data.entering = false;
                    true
                })
            })
            .collect()
    }

    /// Sets `property` on `node`, remembering what it was if this is
    /// the first time a hold or a collapse touches it.
    fn touch(
        &mut self,
        node: Node,
        property: &'static str,
        value: &str,
    ) {
        let Some(data) = self.nodes.get_mut(&node) else {
            return;
        };
        let style = data.el.style();
        if !data.saved.iter().any(|(saved, _)| *saved == property) {
            let was = style
                .get_property_value(property)
                .unwrap_or_default();
            data.saved.push((property, was));
        }
        let _ = style.set_property(property, value);
    }

    fn measure(&self, node: Node) -> Collapsing {
        let data = &self.nodes[&node];
        let parent = data.parent.and_then(|parent| self.el(parent));
        let window = web_sys::window().expect("a window");
        let px = |el: &HtmlElement, property: &str| -> f32 {
            window
                .get_computed_style(el)
                .ok()
                .flatten()
                .and_then(|style| {
                    style.get_property_value(property).ok()
                })
                .and_then(|value| {
                    value.trim_end_matches("px").parse().ok()
                })
                .unwrap_or(0.0)
        };
        let row = parent.is_some_and(|parent| {
            window
                .get_computed_style(parent)
                .ok()
                .flatten()
                .and_then(|style| {
                    style.get_property_value("flex-direction").ok()
                })
                .is_some_and(|direction| direction.starts_with("row"))
        });
        let siblings =
            parent.map_or(0, |parent| parent.child_element_count());
        let gap = match parent {
            Some(parent) if siblings > 1 => {
                px(parent, if row { "column-gap" } else { "row-gap" })
            }
            _ => 0.0,
        };
        let first = parent.is_some_and(|parent| {
            parent.first_element_child().as_ref()
                == Some(data.el.unchecked_ref())
        });
        let (size, padding) = if row {
            (
                data.el.offset_width() as f32,
                (
                    px(&data.el, "padding-left"),
                    px(&data.el, "padding-right"),
                ),
            )
        } else {
            (
                data.el.offset_height() as f32,
                (
                    px(&data.el, "padding-top"),
                    px(&data.el, "padding-bottom"),
                ),
            )
        };
        Collapsing {
            size,
            gap,
            row,
            first,
            padding,
        }
    }
}

impl Backend for Web {
    type World = World;
    type Node = Node;

    fn spawn(world: &mut World, parent: Option<Node>) -> Node {
        let node = world.next;
        world.next += 1;
        let el = world
            .doc
            .create_element("div")
            .expect("a div")
            .unchecked_into::<HtmlElement>();
        let host = match parent.and_then(|parent| world.el(parent)) {
            Some(parent) => parent.clone(),
            None => world.root.clone(),
        };
        let _ = host.append_child(&el);
        if let Some(parent) =
            parent.and_then(|parent| world.nodes.get_mut(&parent))
        {
            parent.children.push(node);
        }
        world.nodes.insert(
            node,
            Data {
                el,
                parent,
                children: Vec::new(),
                hovered: false,
                pressed: false,
                entering: false,
                leaving: false,
                watched_entering: false,
                held: false,
                collapsing: None,
                saved: Vec::new(),
                listeners: Vec::new(),
            },
        );
        node
    }

    fn despawn(world: &mut World, node: Node) {
        let Some(data) = world.nodes.remove(&node) else {
            return;
        };
        data.el.remove();
        world.clicks.remove(&node);
        if let Some(parent) = data
            .parent
            .and_then(|parent| world.nodes.get_mut(&parent))
        {
            parent.children.retain(|&child| child != node);
        }
        let mut stack = data.children;
        while let Some(child) = stack.pop() {
            world.clicks.remove(&child);
            if let Some(data) = world.nodes.remove(&child) {
                stack.extend(data.children);
            }
        }
    }

    fn reorder(world: &mut World, parent: Node, children: &[Node]) {
        let Some(host) = world.el(parent).cloned() else {
            return;
        };
        for child in children {
            if let Some(el) = world.el(*child) {
                let _ = host.append_child(el);
            }
        }
        if let Some(data) = world.nodes.get_mut(&parent) {
            let rest = data
                .children
                .iter()
                .copied()
                .filter(|child| !children.contains(child))
                .collect::<Vec<_>>();
            data.children =
                children.iter().copied().chain(rest).collect();
        }
    }

    fn leave(world: &mut World, node: Node) {
        if let Some(data) = world.nodes.get_mut(&node) {
            data.leaving = true;
            let _ = data
                .el
                .style()
                .set_property("pointer-events", "none");
            world.dirty.push(node);
        }
    }

    fn hold(world: &mut World, node: Node) {
        let row = world.measure(node).row;
        world.touch(node, "position", "absolute");
        if row {
            world.touch(node, "top", "0");
            world.touch(node, "bottom", "0");
        } else {
            world.touch(node, "left", "0");
            world.touch(node, "right", "0");
        }
        world.touch(node, "visibility", "hidden");
        world.touch(node, "pointer-events", "none");
        if let Some(data) = world.nodes.get_mut(&node) {
            data.held = true;
        }
        world.entrances.retain(|&entered| entered != node);
    }

    fn collapse(world: &mut World, node: Node, progress: f32) {
        let Some(data) = world.nodes.get(&node) else {
            return;
        };
        let collapsing = match data.collapsing {
            Some(collapsing) => collapsing,
            None => {
                let collapsing = world.measure(node);
                if let Some(data) = world.nodes.get_mut(&node) {
                    data.collapsing = Some(collapsing);
                }
                collapsing
            }
        };
        let held = world.nodes[&node].held;
        if held {
            world.touch(node, "position", "relative");
            for edge in ["left", "right", "top", "bottom"] {
                world.touch(node, edge, "auto");
            }
            world.touch(node, "visibility", "visible");
        }
        let keep = 1.0 - progress;
        let size = format!("{}px", collapsing.size * keep);
        let gap = format!("{}px", -collapsing.gap * progress);
        let (start, end) = collapsing.padding;
        let (length, min, pad_start, pad_end, before, after) =
            if collapsing.row {
                (
                    "width",
                    "min-width",
                    "padding-left",
                    "padding-right",
                    "margin-left",
                    "margin-right",
                )
            } else {
                (
                    "height",
                    "min-height",
                    "padding-top",
                    "padding-bottom",
                    "margin-top",
                    "margin-bottom",
                )
            };
        world.touch(node, "overflow", "hidden");
        world.touch(node, "flex-shrink", "1");
        world.touch(node, length, &size);
        world.touch(node, min, "0");
        world.touch(node, pad_start, &format!("{}px", start * keep));
        world.touch(node, pad_end, &format!("{}px", end * keep));
        // A negative margin takes back the parent's gap beside it.
        world.touch(
            node,
            if collapsing.first { after } else { before },
            &gap,
        );
    }

    fn release(world: &mut World, node: Node) {
        let Some(data) = world.nodes.get_mut(&node) else {
            return;
        };
        let style = data.el.style();
        for (property, was) in data.saved.drain(..) {
            if was.is_empty() {
                let _ = style.remove_property(property);
            } else {
                let _ = style.set_property(property, &was);
            }
        }
        data.held = false;
        data.collapsing = None;
        data.entering = false;
        world.dirty.push(node);
    }
}

/// The pointer is over the node.
pub struct Hovered;

impl Condition<Web> for Hovered {
    fn holds(world: &World, node: Node) -> bool {
        world.nodes.get(&node).is_some_and(|data| data.hovered)
    }

    fn watch(world: &mut World, node: Node) {
        world.listen(node);
    }
}

/// A button of the pointer is down on the node.
pub struct Pressed;

impl Condition<Web> for Pressed {
    fn holds(world: &World, node: Node) -> bool {
        world.nodes.get(&node).is_some_and(|data| data.pressed)
    }

    fn watch(world: &mut World, node: Node) {
        world.listen(node);
    }
}

/// The node's view was just built: for its first frame, or, when a
/// `keyed` or `each` built it later, until its space has expanded.
pub struct Entering;

impl Condition<Web> for Entering {
    fn holds(world: &World, node: Node) -> bool {
        world.nodes.get(&node).is_some_and(|data| data.entering)
    }

    fn watch(world: &mut World, node: Node) {
        let Some(data) = world.nodes.get_mut(&node) else {
            return;
        };
        if core::mem::replace(&mut data.watched_entering, true) {
            return;
        }
        data.entering = true;
        world.entrances.push(node);
    }
}

/// The node's view was dropped, and is animating out.
pub struct Leaving;

impl Condition<Web> for Leaving {
    fn holds(world: &World, node: Node) -> bool {
        world.nodes.get(&node).is_some_and(|data| data.leaving)
    }

    fn watch(_: &mut World, _: Node) {}
}

/// A signal reading the resource `R`, re-read whenever it is taken
/// with [`World::get_mut`].
pub fn resource<R: 'static, T>(
    read: impl Fn(&R) -> T + Send + Sync + 'static,
) -> Signal<World, T> {
    let mut seen = None;
    Signal::new(
        move |world: &World| read(world.get::<R>()),
        move |world: &World| {
            let version = Some(world.version::<R>());
            core::mem::replace(&mut seen, version) != version
        },
    )
}
