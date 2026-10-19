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
//
// The glue and wasm are loaded from the hashed addresses the page puts
// on each demo's shell (components/demo.typ's `wasm-attrs`), so the two
// always come from the same build, whatever a browser has cached.

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

// Imports the glue and starts the wasm named on `container`'s shell,
// once, resolving to the glue's exports.
async function load(container) {
  const shell = container.closest("[data-fynix-glue]");
  const glue = shell?.dataset.fynixGlue ?? "/wasm/js_fynix.mjs";
  const wasm = shell?.dataset.fynixWasm;
  const fynix = await import(glue);
  await fynix.default(wasm ? { module_or_path: wasm } : undefined);
  return fynix;
}

function mount(selector, name, onMount) {
  const container = document.querySelector(selector);
  if (!container || container.dataset.mounted) return;
  const token = String(++mounts);
  container.dataset.mounted = token;
  ready ??= load(container);
  ready
    .then((fynix) => {
      const demo = new fynix[name](container);
      run(container, demo, token);
      // Whether `demo` is still the one running: once it is not, it
      // has been freed and must not be called.
      const alive = () =>
        container.isConnected && container.dataset.mounted === token;
      onMount?.(demo, container, alive);
    })
    .catch((err) => {
      console.error("fynix-demo: failed to mount", selector, err);
    });
}

// Mounts the demo struct `name` (e.g. "ThemesDemo") into the element
// at `selector`, now and after every navigation that brings it back.
// `onMount(demo, container, alive)`, if given, runs after each mount.
export function mountDemo(selector, name, onMount) {
  demos.set(selector, [name, onMount]);
  mount(selector, name, onMount);
}

document.addEventListener("tola:navigate", () => {
  for (const [selector, [name, onMount]] of demos) {
    mount(selector, name, onMount);
  }
});
