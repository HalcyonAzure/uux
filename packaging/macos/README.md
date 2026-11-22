# macOS (Apple Silicon) build

Unofficial. Cross-compiled on Linux (x86_64 Debian 13) for `aarch64-apple-darwin`, then bundled and ad-hoc signed on a Mac.

## Linux host

1. Packages: `clang lld llvm libclang-dev protobuf-compiler pkg-config nasm autoconf automake libtool`; rustup with target `aarch64-apple-darwin`.
2. Copy a macOS SDK from a Mac you own (`/Library/Developer/CommandLineTools/SDKs/MacOSX*.sdk`) to `/opt/osx/`. The SDK is not redistributed here; Apple's license limits it to Apple hardware.
3. Install `mac-cc` / `mac-cxx` into `/opt/osx/bin/` and link `mac-ar`, `mac-ranlib`, `mac-nm`, `mac-strip` to the matching `llvm-*` tools. Link `ld64.lld` into `PATH`.
4. `build-media.sh` builds static opus 1.5.2, zimg 3.0.6 and FFmpeg 9.0.2 (H.264/HEVC decode, VideoToolbox) into `/opt/osx/deps`. Afterwards remove `-lstdc++` from `/opt/osx/deps/lib/pkgconfig/*.pc` (macOS only has libc++).
5. Add to `~/.cargo/config.toml`:

   ```toml
   [target.aarch64-apple-darwin]
   linker = "/opt/osx/bin/mac-cc"
   ar = "/opt/osx/bin/mac-ar"
   ```

6. `. packaging/macos/cross.env && cargo build --locked --release --target aarch64-apple-darwin`

## Mac

`packaging/macos/bundle.sh target/aarch64-apple-darwin/release/uux <version>` produces `UUX-<version>-macos-arm64.zip`.

## Licensing of bundled media libraries

FFmpeg (LGPL-2.1+), opus (BSD-3-Clause) and zimg (WTFPL) are linked statically. The exact FFmpeg source is attached to each release; this directory plus the tagged source is what is needed to rebuild and relink.
