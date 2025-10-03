#!/bin/bash
set -euo pipefail

export GSETTINGS_BACKEND=memory
export DBUS_SESSION_BUS_ADDRESS=/dev/null

optimize() {
  shopt -s nullglob
  for f in "$@"; do
    [[ -f "$f" ]] || continue
    tmp="${f%.png}.opt.png"
    zopflipng -m --lossy_transparent "$f" "$tmp" >/dev/null 2>&1 && mv -f "$tmp" "$f"
  done
  shopt -u nullglob
}

# preclean
find . -maxdepth 1 ! -name '.' ! -name "$(basename "$0")" -exec rm -rf {} +

# general
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/128x128.png" --export-type=png -w 128 -h 128 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/128x128@2x.png" --export-type=png -w 256 -h 256 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/32x32.png" --export-type=png -w 32 -h 32 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/icon.png" --export-type=png -w 512 -h 512 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square107x107Logo.png" --export-type=png -w 107 -h 107 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square142x142Logo.png" --export-type=png -w 142 -h 142 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square150x150Logo.png" --export-type=png -w 150 -h 150 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square284x284Logo.png" --export-type=png -w 284 -h 284 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square30x30Logo.png" --export-type=png -w 30 -h 30 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square310x310Logo.png" --export-type=png -w 310 -h 310 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square44x44Logo.png" --export-type=png -w 44 -h 44 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square71x71Logo.png" --export-type=png -w 71 -h 71 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/Square89x89Logo.png" --export-type=png -w 89 -h 89 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/StoreLogo.png" --export-type=png -w 50 -h 50 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &

wait
optimize 128x128.png 128x128@2x.png 32x32.png icon.png Square107x107Logo.png Square142x142Logo.png Square150x150Logo.png Square284x284Logo.png Square30x30Logo.png Square310x310Logo.png Square44x44Logo.png Square71x71Logo.png Square89x89Logo.png StoreLogo.png

# preclean
rm -rf temp/
mkdir temp
# ico/icns common
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/16.png" --export-type=png -w 16 -h 16 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/24.png" --export-type=png -w 24 -h 24 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/32.png" --export-type=png -w 32 -h 32 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/48.png" --export-type=png -w 48 -h 48 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/64.png" --export-type=png -w 64 -h 64 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/128.png" --export-type=png -w 128 -h 128 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/256.png" --export-type=png -w 256 -h 256 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/512.png" --export-type=png -w 512 -h 512 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &
inkscape "$(pwd)/../../static/logo.svg" --export-filename="$(pwd)/temp/1024.png" --export-type=png -w 1024 -h 1024 --export-png-color-mode=RGBA_8 --export-png-compression=9 --export-png-antialias=3 &

wait
optimize temp/*.png

# ico
magick temp/16.png temp/24.png temp/32.png temp/48.png temp/64.png temp/128.png temp/256.png icon.ico &
# icns
png2icns icon.icns temp/16.png temp/32.png temp/64.png temp/128.png temp/256.png temp/512.png temp/1024.png &

# postclean
wait
rm -rf temp/
