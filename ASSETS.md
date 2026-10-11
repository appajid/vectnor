# Asset attribution

Every non-code asset in this repository (icons, images, fonts, example art, colour profiles, presets) is listed here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if an asset file is missing from this table.

**Policy (mandatory):** Vectnor contains **no Adobe iconography, images, artwork, presets, swatch/brush/symbol/pattern libraries or ICC profiles**. Every asset is either original work by Vectnor contributors, or third-party material under an open licence (OSI open source, public domain / CC0, or Creative Commons that allows redistribution), and it is attributed below. Screenshots of Adobe software are never committed.

Generated-in-code art is original and has no file to list. This covers the default swatches, brushes, symbols, patterns, graphic styles, image-trace presets and the vector tool cursors (`crates/ui-egui/src/cursors.rs`).

**Font files are never committed here.** Fonts shared by the Crafting Apps live in [storytold/craft-fonts](https://github.com/storytold/craft-fonts) (rules: craftrules [`standards/fonts.md`](https://github.com/storytold/craftrules/blob/main/standards/fonts.md)). The small Latin UI/document fonts below predate that rule and stay.

**Optional build input: craft-fonts.** Builds made with `CRAFT_FONTS_DIR=<craft-fonts checkout>` (all official releases) embed the Japanese fonts listed in its [`ATTRIBUTION.md`](https://github.com/storytold/craft-fonts/blob/main/ATTRIBUTION.md): BIZ UDPGothic Regular and Bold, Shippori Mincho Regular and BIZ UDMincho Regular (web builds: BIZ UDPGothic Regular only), all under the SIL Open Font License 1.1. They are not files in this repository; release packages carry each one's `OFL-<family>.txt`. Builds without `CRAFT_FONTS_DIR` embed none of them.

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|

| `assets/app-icon/LICENSE.txt` | (licence/readme text) | Vectnor | MIT OR Apache-2.0 |  |
| `assets/app-icon/README.md` | (licence/readme text) | Vectnor | MIT OR Apache-2.0 |  |
| `assets/app-icon/hicolor/128x128/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/16x16/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/24x24/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/256x256/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/32x32/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/48x48/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/512x512/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/64x64/apps/org.vectnor.app.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/hicolor/scalable/apps/org.vectnor.app.svg` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/vectnor-1024.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/vectnor-macos-512.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/vectnor-small.svg` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/vectnor.icns` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/vectnor.ico` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `assets/app-icon/vectnor.svg` | Appaji Ambarisha (AI-assisted artwork) | Supplied App-Icon-Dark.svg in `Vectnor logos icons svg/`, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | Vectnor V mark; platform icon generated from the supplied SVG |
| `packaging/macos/dmg/background.svg` | Xus Badia (original layout, #493); project contributor (Vectnor branding) | Original artwork for Vectnor's DMG window; links the app icon `assets/app-icon/vectnor-small.svg` | MIT OR Apache-2.0 (licensed by the contributor under the project licence) | Rebranded with the supplied Vectnor icon and teal accent on 2026-10-12. DMG background source. Text in Inter and JetBrains Mono (OFL-1.1, `assets/fonts`), not embedded; paper grain generated by an SVG `feTurbulence` filter |
| `packaging/macos/dmg/background.tiff` | Xus Badia (original layout, #493); project contributor (Vectnor branding) | Rendered from `packaging/macos/dmg/background.svg` by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | 1x + 2x DMG background; includes the app icon (see its rows), text rendered from OFL-1.1 fonts and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `packaging/macos/dmg/DS_Store` | Xus Badia (layout, #493) | Written by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | Finder view settings for the DMG (window, icon size and positions, background alias); no machine data |
| `assets/fonts/Inter-Medium.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/Inter-Regular.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/Inter-SemiBold.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/JetBrainsMono-Regular.ttf` | The JetBrains Mono Project Authors | https://github.com/JetBrains/JetBrainsMono | OFL-1.1 (`assets/fonts/OFL-JetBrainsMono.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Bold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-It.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Regular.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Semibold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSerif4-Regular.ttf` | Frank Grießhammer / Adobe (released as open source) | https://github.com/adobe-fonts/source-serif | OFL-1.1 (`assets/fonts/OFL-SourceSerif4.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/OFL-Inter.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-JetBrainsMono.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSans3.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSerif4.txt` | (licence text) | upstream project | — |  |
| `assets/icons/LICENSE-lucide.txt` | (licence text) | upstream project | — |  |
| `assets/icons/align-center-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-center-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-left-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-up-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/blend.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/brush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chart-column.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/circle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/clipboard-paste.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/cloud.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/combine.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/copy.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/dc-actions.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-bottom.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-hcenter.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-left.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-right.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-top.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-vcenter.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-align.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-artboard.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-key.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-selection.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor-smooth.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-appearance.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arc.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-down.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-extend.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-tip.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-up.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboard-options.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboards.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-blend.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-bloat.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-butt.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-round.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-square.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-hide.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-show.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-clear.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-color-guide.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-crystallize.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cube.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-align.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-exact.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-off.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-on.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-direct.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-bottom.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hcenter.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hspace.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-left.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-right.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-top.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vcenter.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vspace.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-behind.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-inside.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-normal.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-ellipse.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fill-none.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-flare.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-folder.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-free-transform.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fx.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gamut.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-freeform.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-linear.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-radial.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-across.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-along.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-within.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gradient.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-graphic-styles.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grid-view.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-group-select.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-bevel.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-miter.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-round.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-knife.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line-cut.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-list-view.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-bucket.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-select.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mask-none.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-measure.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mesh.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mirror-cut.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-fill.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-item.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-stroke.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-options.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-center.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-all.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-center.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-left.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-right.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-left.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-ltr.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-right.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-rtl.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-path-eraser.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pathfinder.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-perspective.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-crop.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-divide.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-exclude.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-intersect.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-merge.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-back.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-front.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-outline.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-trim.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-unite.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-place-symbol.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polar-grid.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polygon.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pucker.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-puppet.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rearrange.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-cut.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-grid.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reference-point.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-remove-brush.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reshape.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reverse.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rotate-view.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rounded-rect.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-evenodd.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-nonzero.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-scallop.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-screen-mode.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-selection.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shape-builder.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shear.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-smooth.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-center.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-inside.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-outside.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swap.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swatch-kinds.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-symbol-sprayer.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-touch-type.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transform-panel.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transparency.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-twirl.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-area.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-path.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-vertical.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width-profile.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-wrinkle.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-large.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-small.svg` | Vectnor contributors | Original work, drawn for Vectnor | MIT OR Apache-2.0 |  |
| `assets/icons/ellipsis.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eraser.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/file-plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-horizontal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-vertical-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/folder-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/frame.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/grid-3x3.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/group.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hand.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hexagon.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/history.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/house.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/image.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/info.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lasso.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layers.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layout-grid.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/library.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link-2-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/globe.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | CSS Properties panel |
| `assets/icons/lock-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lock.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/map.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/maximize-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/menu.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/minus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/monitor.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move-diagonal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paint-bucket.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paintbrush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/palette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool-add.svg` | Lucide Icons and Contributors; badge by Vectnor contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a plus badge |
| `assets/icons/pen-tool-delete.svg` | Lucide Icons and Contributors; badge by Vectnor contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a minus badge |
| `assets/icons/pencil.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pilcrow.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pin.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pipette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/printer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/redo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-3d.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-ccw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-cw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ruler.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/save.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scaling.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scissors.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/search.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/settings.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/shapes.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/share-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/slice.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline-pointer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spray-can.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square-dashed.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-exclude.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-intersect.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-subtract.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-unite.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squircle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/star.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sun.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/swatch-book.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/text-cursor-input.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/tornado.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/trash-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/triangle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/type.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/undo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ungroup.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/wand-sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/waves.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/x.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/zoom-in.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `docs/images/dusk-poster.png` | Vectnor contributors | Rendered by Vectnor from `examples/dusk-poster.vcnr` | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.vcnr` | Vectnor contributors | Original artwork built through the Vectnor command API | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.svg` | Vectnor contributors | SVG export of the above | MIT OR Apache-2.0 |  |
| `docs/images/art-neon-drive.png` | Vectnor contributors | Rendered by Vectnor from `examples/neon-drive.vcnr` | MIT OR Apache-2.0 |  |
| `docs/images/art-ribbons.png` | Vectnor contributors | Rendered by Vectnor from `examples/ribbons.vcnr` | MIT OR Apache-2.0 |  |
| `docs/images/art-pathfinder.png` | Vectnor contributors | Rendered by Vectnor from `examples/feature-sheet.vcnr` (artboard 1) | MIT OR Apache-2.0 |  |
| `docs/images/art-mesh.png` | Vectnor contributors | Rendered by Vectnor from `examples/feature-sheet.vcnr` (artboard 2) | MIT OR Apache-2.0 |  |
| `docs/images/art-repeat.png` | Vectnor contributors | Rendered by Vectnor from `examples/feature-sheet.vcnr` (artboard 3) | MIT OR Apache-2.0 |  |
| `docs/images/art-envelope.png` | Vectnor contributors | Rendered by Vectnor from `examples/feature-sheet.vcnr` (artboard 4) | MIT OR Apache-2.0 |  |
| `examples/neon-drive.vcnr` | Vectnor contributors | Original artwork built through the Vectnor command API | MIT OR Apache-2.0 |  |
| `examples/ribbons.vcnr` | Vectnor contributors | Original artwork built through the Vectnor command API | MIT OR Apache-2.0 |  |
| `examples/feature-sheet.vcnr` | Vectnor contributors | Original artwork built through the Vectnor command API | MIT OR Apache-2.0 |  |
| `examples/perspective-city.vcnr` | Vectnor contributors | Original artwork built through the Vectnor command API | MIT OR Apache-2.0 |  |
| `assets/brand/LICENSE-APACHE` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Copy of the repository licence text |
| `assets/brand/LICENSE-MIT` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Copy of the repository licence text |
| `assets/brand/about.png` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app; developer credit added at project owner’s request: Appaji Ambarisha |
| `assets/brand/about.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app; developer credit added at project owner’s request: Appaji Ambarisha |
| `assets/brand/app-icon-dark.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/app-icon-light.png` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/app-icon-light.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/app-icon-monochrome.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/logo-inverse.png` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/logo-inverse.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/logo-primary.png` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/logo-primary.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/logo-symbol.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app |
| `assets/brand/splash.png` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app; developer credit added at project owner’s request: Appaji Ambarisha |
| `assets/brand/splash.svg` | Appaji Ambarisha (AI-assisted artwork) | Original SVG set in `Vectnor logos icons svg/`; supplied locally on 2026-10-12 | MIT OR Apache-2.0 (licence texts in `assets/brand/`) | Vectnor branding; PNGs rendered from the matching SVG, About version displayed by the app; developer credit added at project owner’s request: Appaji Ambarisha |
| `assets/brand/document.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/brand/document.icns` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/brand/document.svg` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/brand/README.md` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/brand/document.ico` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/512x512/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/48x48/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/scalable/mimetypes/application-x-vectnor.svg` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/256x256/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/128x128/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/24x24/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/32x32/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/64x64/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `assets/app-icon/hicolor/16x16/mimetypes/application-x-vectnor.png` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `Vectnor logos icons svg/LICENSE-APACHE` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `Vectnor logos icons svg/Vectnor-Document-Icon.svg` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |
| `Vectnor logos icons svg/LICENSE-MIT` | Appaji Ambarisha (AI-assisted artwork) | Supplied SVG artwork in `Vectnor logos icons svg/`, confirmed by contributor on 2026-10-12 | MIT OR Apache-2.0 (repository licences and `assets/brand/`) | Original Vectnor artwork or generated derivative; licence files retain their original notices |

| `Vectnor logos icons svg/About-Screen.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork; developer credit added at project owner’s request: Appaji Ambarisha |
| `Vectnor logos icons svg/App-Icon-Dark.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork |
| `Vectnor logos icons svg/App-Icon-Light.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork |
| `Vectnor logos icons svg/App-Icon-Monochrome.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork |
| `Vectnor logos icons svg/Logo-Inverse.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork |
| `Vectnor logos icons svg/Logo-Primary.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork |
| `Vectnor logos icons svg/Logo-Symbol.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork |
| `Vectnor logos icons svg/Splash-Screen.svg` | Appaji Ambarisha (AI-assisted artwork) | Original artwork supplied in this workspace; authorship confirmed by the contributor on 2026-10-12 | MIT OR Apache-2.0 (licence texts next to the assets) | Vectnor branding artwork; developer credit added at project owner’s request: Appaji Ambarisha |
