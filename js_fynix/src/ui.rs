//! A world, a theme and what is mounted in it, stepped once a frame.

use core::time::Duration;

use fynix::{Cx, Mounted, Tick, View};
use web_sys::HtmlElement;

use crate::backend::{Node, Web, World};
use crate::tokens::Theme;

pub struct Ui<T> {
    pub world: World,
    theme: T,
    mounted: Mounted<Web, T>,
}

impl<T: Theme> Ui<T> {
    /// A UI building into `root`, with `theme`.
    pub fn new(root: HtmlElement, theme: T) -> Self {
        Self {
            world: World::new(root),
            theme,
            mounted: Mounted::default(),
        }
    }

    /// Builds `view` at the root.
    pub fn mount(&mut self, view: impl View<Web, T>) -> Node {
        let mut cx =
            Cx::new(&mut self.world, &self.theme, &mut self.mounted);
        cx.build(view)
    }

    /// Re-reads every prop of every element at the next frame, for
    /// a theme whose tokens just changed.
    pub fn restyle(&mut self) {
        for node in self.world.nodes() {
            self.mounted.mark_dirty(node);
        }
    }

    /// Runs one frame, `delta` after the last: the pointer, clicks,
    /// `keyed` and `each` rebuilds, then every element's writes.
    pub fn frame(&mut self, delta: Duration, reduced_motion: bool) {
        let (changed, clicks) = self.world.drain();
        for node in changed {
            self.mounted.mark_dirty(node);
        }
        for click in clicks {
            click(&mut self.world);
        }
        self.mounted.run_effects(&mut self.world);
        self.mounted.update_structure(&mut self.world, &self.theme);
        for node in core::mem::take(&mut self.world.dirty) {
            self.mounted.mark_dirty(node);
        }
        self.mounted.update_elements(
            &mut self.world,
            &self.theme,
            Tick {
                delta,
                reduced_motion,
            },
        );
        for node in self.world.settle() {
            self.mounted.mark_dirty(node);
        }
    }
}

/// `dt_ms` from JavaScript as a frame's delta, kept to a tenth of a
/// second so a tab coming back does not skip every animation.
pub fn delta(dt_ms: f64) -> Duration {
    Duration::from_secs_f64((dt_ms / 1000.0).clamp(0.0, 0.1))
}
