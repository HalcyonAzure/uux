#!/usr/bin/env bash
# Run on a Mac: wrap the cross-built binary into UUX.app, ad-hoc sign, zip.
# usage: packaging/macos/bundle.sh path/to/uux VERSION
set -euo pipefail
bin=${1:?binary required}; version=${2:?version required}
here=$(cd "$(dirname "$0")" && pwd); root=$(cd "$here/../.." && pwd)
out=$(mktemp -d); app="$out/UUX.app"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources" "$out/uux.iconset"
for s in 16 32 128 256 512; do
  sips -z $s $s "$root/assets/uux.png" --out "$out/uux.iconset/icon_${s}x${s}.png" >/dev/null
  d=$((s * 2)); sips -z $d $d "$root/assets/uux.png" --out "$out/uux.iconset/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$out/uux.iconset" -o "$app/Contents/Resources/uux.icns"
install -m 755 "$bin" "$app/Contents/MacOS/uux"
sed -e "s|<string>0.2.1</string>|<string>$version</string>|" "$here/Info.plist" > "$app/Contents/Info.plist"
printf 'APPL????' > "$app/Contents/PkgInfo"
plutil -lint "$app/Contents/Info.plist" >/dev/null
codesign --force --sign - --identifier io.github.vvdkzievqcrq.uux "$app"
codesign --verify --strict "$app"
zip="UUX-$version-macos-arm64.zip"
(cd "$out" && ditto -c -k --sequesterRsrc --keepParent UUX.app "$PWD/$zip")
mv "$out/$zip" .
rm -rf "$out"
shasum -a 256 "$zip"
