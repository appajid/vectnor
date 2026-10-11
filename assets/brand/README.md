# Vectnor branding

Original AI-assisted SVGs supplied by the project contributor in `Vectnor logos icons svg/`, licensed MIT OR Apache-2.0. The source artwork is kept there, with the project owner’s requested developer credit added to both screens.

The document icon is also exported as Finder ICNS, Windows ICO and Linux MIME icons.

The app uses theme-specific icons, primary/inverse wordmarks on Home, the About artwork with a live version label and clickable Close control, and the splash artwork while preparing the first frame. Both screens display **Developer: Appaji Ambarisha** within the artwork. The web build also shows the credited splash while WASM loads.

`about.svg` replaces the supplied HTML `foreignObject` paragraph with SVG text for native renderer compatibility and leaves the static version blank; the app paints its actual build version. PNGs are rendered at 2× so desktop and WASM show identical lettering without bundled fonts. Run `packaging/branding.sh` with resvg to regenerate them.
