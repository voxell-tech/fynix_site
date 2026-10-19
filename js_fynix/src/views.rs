//! Composites and wrappers built from the two elements: rows,
//! columns, buttons and their looks, clicks, and views that animate
//! in and out.

use std::rc::Rc;

use fynix::{Cx, ScopedExt, Styled, View, ViewSeq, Visual, When};

use crate::backend::{
    Entering, Hovered, Leaving, Node, Pressed, Web, World,
};
use crate::elements::{
    Align, Color, Direction, Frame, FrameProps, Justify, Label, Pad,
    Tone,
};
use crate::tokens::{SpacingTokens, Theme};

/// A [`Frame`] with `children` built under it, in order. It takes the
/// frame's props, so `row((a, b)).gap(8.0)` styles the frame.
pub struct Stack<C> {
    pub frame: Frame,
    pub children: C,
}

/// A [`Stack`] laying `children` out left to right.
pub fn row<C>(children: C) -> Stack<C> {
    Stack {
        frame: Frame::unset().direction(Direction::Row),
        children,
    }
}

/// A [`Stack`] laying `children` out top to bottom.
pub fn column<C>(children: C) -> Stack<C> {
    Stack {
        frame: Frame::unset().direction(Direction::Column),
        children,
    }
}

impl<C> FrameProps for Stack<C> {
    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }
}

impl<T, C> View<Web, T> for Stack<C>
where
    T: SpacingTokens + Send + Sync + 'static,
    C: ViewSeq<Web, T>,
{
    fn build(self, cx: &mut Cx<'_, Web, T>) -> Node {
        let node = cx.build(self.frame);
        cx.under(node, |cx| self.children.build_each(cx));
        node
    }
}

/// A frame holding `content` that answers the pointer. Its frame
/// styles the button.
pub struct Button<C> {
    pub frame: Frame,
    content: C,
}

pub fn button<C>(content: C) -> Button<C> {
    Button {
        frame: Frame::unset(),
        content,
    }
}

impl<C> FrameProps for Button<C> {
    fn frame_mut(&mut self) -> &mut Frame {
        &mut self.frame
    }
}

/// A button's own look, weaker than any rule of an app's: a fill
/// that lightens under the pointer and darkens when pressed.
fn defaults<T: Theme>(cx: &mut Cx<'_, Web, T>) {
    cx.root(|cx| {
        cx.set::<Frame>(|f, t| {
            f.direction(Direction::Row)
                .align(Align::Center)
                .justify(Justify::Center)
                .padding(Pad::new(t.pad() * 1.6, t.pad()))
                .radius(t.radius())
                .fill(t.fill())
        })
    });
    cx.when::<Hovered>(|cx| {
        cx.root(|cx| cx.set::<Frame>(|f, t| f.fill(t.hover())))
    });
    cx.when::<Pressed>(|cx| {
        cx.root(|cx| {
            cx.set::<Frame>(|f, t| f.fill(t.pressed()).scale(0.96))
        })
    });
}

impl<T, C> View<Web, T> for Button<C>
where
    T: Theme,
    C: View<Web, T>,
{
    fn build(self, cx: &mut Cx<'_, Web, T>) -> Node {
        cx.scope(|cx| {
            cx.defaults(defaults);
            let node = cx.build(self.frame);
            if let Some(el) = cx.world.el(node) {
                let _ = el.class_list().add_1("fx-button");
            }
            cx.world.listen(node);
            cx.under(node, |cx| cx.build(self.content));
            node
        })
    }
}

/// No surface until the pointer is on it.
pub fn ghost<T: Theme>(cx: &mut Cx<'_, Web, T>) {
    cx.root(|cx| cx.set::<Frame>(|f, _| f.fill(Color::NONE)));
}

/// Filled with the accent, and lifted under the pointer.
pub fn primary<T: Theme>(cx: &mut Cx<'_, Web, T>) {
    filled(cx, |t| t.accent());
}

/// Filled with the critical colour, for what cannot be undone.
pub fn danger<T: Theme>(cx: &mut Cx<'_, Web, T>) {
    filled(cx, |t| t.critical());
}

fn filled<T: Theme>(cx: &mut Cx<'_, Web, T>, color: fn(&T) -> Color) {
    cx.root(|cx| cx.set::<Frame>(move |f, t| f.fill(color(t))));
    cx.set::<Label>(|l, _| l.tone(Tone::OnAccent).bold(true));
    cx.when::<Hovered>(|cx| {
        cx.root(|cx| {
            cx.set::<Frame>(move |f, t| {
                f.fill(color(t).lighten(0.18)).shadow(16.0)
            })
        })
    });
    cx.when::<Pressed>(|cx| {
        cx.root(|cx| {
            cx.set::<Frame>(move |f, t| f.fill(color(t)).shadow(2.0))
        })
    });
}

/// A card: the theme's panel, padded and bordered.
pub fn panel<T: Theme>(cx: &mut Cx<'_, Web, T>) {
    cx.root(|cx| {
        cx.set::<Frame>(|f, t| {
            f.fill(t.panel())
                .radius(t.radius() * 1.6)
                .padding(Pad::all(t.gap() * 2.0))
                .gap(t.gap() * 1.5)
                .border(1.0)
                .border_color(t.border())
                .shadow(24.0)
        })
    });
}

/// A view that runs `click` when pressed and let go on.
pub struct OnClick<V> {
    view: V,
    click: Rc<dyn Fn(&mut World)>,
}

/// What any view can be clicked with.
pub trait ClickExt: Sized {
    /// This view, running `click` when it is clicked.
    fn on_click(
        self,
        click: impl Fn(&mut World) + 'static,
    ) -> OnClick<Self> {
        OnClick {
            view: self,
            click: Rc::new(click),
        }
    }
}

impl<V> ClickExt for V {}

impl<T, V: View<Web, T>> View<Web, T> for OnClick<V> {
    fn build(self, cx: &mut Cx<'_, Web, T>) -> Node {
        let node = self.view.build(cx);
        cx.world.clicks.insert(node, self.click);
        cx.world.listen(node);
        node
    }
}

/// Where a view is while out of sight, for [`AppearExt::appear`]:
/// every element in it transparent, and its root scaled down a
/// little.
pub fn hidden<T: 'static>(cx: &mut Cx<'_, Web, T>) {
    cx.set::<Visual<World>>(|v, _| v.opacity(0.0));
    cx.root(|cx| cx.set::<Visual<World>>(|v, _| v.scale(0.92)));
}

type Away<T> = fn(&mut Cx<'_, Web, T>);

/// What any view can animate in and out with.
pub trait AppearExt: Sized {
    /// This view, coming in from and leaving to the look `away`
    /// sets. It needs a transition to move.
    #[allow(clippy::type_complexity)]
    fn appear<T: 'static>(
        self,
        away: Away<T>,
    ) -> When<When<Self, Away<T>, Entering>, Away<T>, Leaving> {
        self.when_in::<Entering, _>(away)
            .when_in::<Leaving, _>(away)
    }
}

impl<V> AppearExt for V {}
