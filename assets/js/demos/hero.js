// Mounts js_fynix's HeroDemo (js_fynix/src/demos/hero.rs) into the
// landing hero. Until the visitor touches it, the card steps through
// the three skins on its own; the hero's backdrop follows whichever
// skin is showing, through its `data-skin`.
import { mountDemo } from "/js/fynix-demo.js";

const STEP_MS = 3200;

mountDemo("#hero-demo", "HeroDemo", (demo, container, alive) => {
  const hero = container.closest(".fx-hero");
  const reduced = window.matchMedia("(prefers-reduced-motion: reduce)");
  let touched = false;
  container.addEventListener("pointerdown", () => {
    touched = true;
  });

  const follow = () => {
    if (hero) hero.dataset.skin = demo.skin();
  };
  follow();

  const cycle = setInterval(() => {
    if (!alive()) return clearInterval(cycle);
    if (!touched && !reduced.matches && !document.hidden) demo.next_skin();
  }, STEP_MS);
  const watch = setInterval(() => {
    if (!alive()) return clearInterval(watch);
    follow();
  }, 120);
});
