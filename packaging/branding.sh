#!/usr/bin/env bash
# Render the supplied branding SVGs; originals are preserved in Vectnor logos icons svg/.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/brand"
command -v resvg >/dev/null || { echo "error: resvg not found" >&2; exit 1; }
resvg -w 1520 -h 1200 "$DIR/about.svg" "$DIR/about.png"
resvg -w 1800 -h 1160 "$DIR/splash.svg" "$DIR/splash.png"
resvg -w 1300 -h 310 "$DIR/logo-primary.svg" "$DIR/logo-primary.png"
resvg -w 1300 -h 310 "$DIR/logo-inverse.svg" "$DIR/logo-inverse.png"
resvg -w 128 -h 128 "$DIR/app-icon-light.svg" "$DIR/app-icon-light.png"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
sed 's/width="512" height="640" viewBox="0 0 440 560"/width="512" height="512" viewBox="-60 0 560 560"/' "$DIR/document.svg" > "$TMP/document.svg"
resvg -w 1024 -h 1024 "$TMP/document.svg" "$DIR/document.png"
for size in 16 20 24 32 40 48 64 128 256 512; do
  resvg -w "$size" -h "$size" "$TMP/document.svg" "$TMP/document-$size.png"
done
(cd "$ROOT" && cargo xtask ico "$DIR/document.ico" "$TMP"/document-{16,20,24,32,40,48,64,128,256}.png)
for size in 16 24 32 48 64 128 256 512; do
  mkdir -p "$ROOT/assets/app-icon/hicolor/${size}x${size}/mimetypes"
  cp "$TMP/document-$size.png" "$ROOT/assets/app-icon/hicolor/${size}x${size}/mimetypes/application-x-vectnor.png"
done
mkdir -p "$ROOT/assets/app-icon/hicolor/scalable/mimetypes"
cp "$DIR/document.svg" "$ROOT/assets/app-icon/hicolor/scalable/mimetypes/application-x-vectnor.svg"
if command -v iconutil >/dev/null; then
  mkdir -p "$TMP/document.iconset"
  for size in 16 32 128 256 512; do
    resvg -w "$size" -h "$size" "$TMP/document.svg" "$TMP/document.iconset/icon_${size}x${size}.png"
    resvg -w "$((size * 2))" -h "$((size * 2))" "$TMP/document.svg" "$TMP/document.iconset/icon_${size}x${size}@2x.png"
  done
  iconutil -c icns -o "$DIR/document.icns" "$TMP/document.iconset"
fi
bash "$ROOT/packaging/icons.sh"
