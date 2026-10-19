// What the shared site kit (shared/site/) needs to know about Fynix.
// The site's title, description, and origin live in tola.toml's
// [site].
#let project = (
  repo: "https://github.com/voxell-tech/fynix",
  // SPDX identifier, for the home page's schema.org data.
  license: "MIT OR Apache-2.0",
  // Social preview card, 1200x630 (assets/images/og.png, made from
  // scripts/og-image.typ). Bump `v` whenever the image changes:
  // Discord, X, and others cache preview images by URL, so the same
  // URL keeps showing the old card.
  og-image: "/images/og.png?v=1",
  // The docs sidebar, in reading order: what previous/next walk.
  chapters: (
    (
      title: "Choose a Backend",
      items: (
        (title: "Overview", href: "/docs/"),
        (title: "Bevy Fynix", href: "/docs/bevy/"),
      ),
    ),
    (
      title: "Concepts",
      items: (
        (title: "Elements", href: "/docs/concepts/elements/"),
        (title: "Tokens and Themes", href: "/docs/concepts/themes/"),
        (title: "Set and Show Rules", href: "/docs/concepts/rules/"),
        (title: "State Rules", href: "/docs/concepts/states/"),
        (title: "Views", href: "/docs/concepts/views/"),
        (title: "Styling", href: "/docs/concepts/styling/"),
      ),
    ),
    (
      title: "Advanced",
      items: (
        (title: "Building a UI Framework", href: "/docs/advanced/"),
      ),
    ),
  ),
)
