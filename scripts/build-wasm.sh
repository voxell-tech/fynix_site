#!/bin/sh
# Builds js_fynix for the browser and copies the output into
# assets/wasm/, where the site serves it from. `tola dev` doesn't know
# js_fynix exists, it only watches Typst/asset files, so this has to be
# run by hand after changing js_fynix/src/.
#
# The JS glue is copied in as .mjs: it uses `import.meta`, which Tola
# 0.8's minifier only accepts in a file it knows is a module.
set -eu

cd "$(dirname "$0")/../js_fynix"
wasm-pack build --release --target web --out-dir pkg --out-name js_fynix

mkdir -p ../assets/wasm
cp pkg/js_fynix.js ../assets/wasm/js_fynix.mjs
cp pkg/js_fynix_bg.wasm ../assets/wasm/

echo "Copied js_fynix into assets/wasm/. Run 'tola build' or let 'tola dev' pick it up."
