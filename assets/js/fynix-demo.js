// Mounts the live Fynix demos (js_fynix, compiled to wasm) and steps
// each one from a requestAnimationFrame loop with real frame times.
//
// A demo's page markup is a `[data-demo-src]` shell around an empty
// container (see content/index.typ); shared/scripts/demo-bootstrap.js
// imports the demo's module, which calls `mountDemo`. Modules only run
// once per URL, so every mounted selector is remembered and mounted
// again whenever Tola's SPA navigation brings its container back.
// Each loop stops and frees its demo once its container leaves the
// page or is mounted again (a navigation can morph the same element
// in place), and idles while offscreen or while the tab is hidden.
import init, * as fynix from "/wasm/js_fynix.mjs";

let ready;
let mounts = 0;
const demos = new Map();
const reducedMotion = window.matchMedia(
  "(prefers-reduced-motion: reduce)",
);

function run(container, demo, token) {
  let visible = true;
  let last = performance.now();
  const observer = new IntersectionObserver((entries) => {
    visible = entries.some((entry) => entry.isIntersecting);
  });
  observer.observe(container);

  const step = (now) => {
    if (!container.isConnected || container.dataset.mounted !== token) {
      observer.disconnect();
      demo.free();
      return;
    }
    if (visible && !document.hidden) {
      demo.frame(now - last, reducedMotion.matches);
    }
    last = now;
    requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
}

function mount(selector, name) {
  const container = document.querySelector(selector);
  if (!container || container.dataset.mounted) return;
  const token = String(++mounts);
  container.dataset.mounted = token;
  ready ??= init();
  ready
    .then(() => run(container, new fynix[name](container), token))
    .catch((err) => {
      console.error("fynix-demo: failed to mount", selector, err);
    });
}

// Mounts the demo struct `name` (e.g. "ThemesDemo") into the element
// at `selector`, now and after every navigation that brings it back.
export function mountDemo(selector, name) {
  demos.set(selector, name);
  mount(selector, name);
}

document.addEventListener("tola:navigate", () => {
  for (const [selector, name] of demos) mount(selector, name);
});
