#!/usr/bin/env bash
set -euo pipefail
. /etc/devbox-mac.env
P=/opt/osx/deps; mkdir -p $P /opt/osx/src; cd /opt/osx/src
J=$(nproc)
HOSTF="--host=aarch64-apple-darwin"
export CC=/opt/osx/bin/mac-cc CXX=/opt/osx/bin/mac-cxx AR=/opt/osx/bin/mac-ar RANLIB=/opt/osx/bin/mac-ranlib NM=/opt/osx/bin/mac-nm STRIP=/opt/osx/bin/mac-strip
step(){ echo "### $(date +%T) $*"; }

step opus
[ -f opus-1.5.2.tar.gz ] || curl -fL --retry 5 -o opus-1.5.2.tar.gz https://downloads.xiph.org/releases/opus/opus-1.5.2.tar.gz
rm -rf opus-1.5.2 && tar xf opus-1.5.2.tar.gz && cd opus-1.5.2
./configure $HOSTF --prefix=$P --enable-static --disable-shared --with-pic --disable-doc --disable-extra-programs >/tmp/opus.cfg 2>&1 || { tail -30 config.log; exit 1; }
make -j$J >/tmp/opus.make 2>&1 && make install >/dev/null; cd ..

step zimg
[ -f zimg-3.0.6.tar.gz ] || curl -fL --retry 5 -o zimg-3.0.6.tar.gz https://github.com/sekrit-twc/zimg/archive/refs/tags/release-3.0.6.tar.gz
rm -rf zimg-release-3.0.6 && tar xf zimg-3.0.6.tar.gz && cd zimg-release-3.0.6
./autogen.sh >/tmp/zimg.ag 2>&1
./configure $HOSTF --prefix=$P --enable-static --disable-shared --with-pic >/tmp/zimg.cfg 2>&1 || { tail -30 config.log; exit 1; }
make -j$J >/tmp/zimg.make 2>&1 && make install >/dev/null; cd ..
# zimg is C++: make static consumers pull libc++
sed -i 's/^Libs.private:.*/& -lc++/; t; $a Libs.private: -lc++' $P/lib/pkgconfig/zimg.pc

step ffmpeg
[ -f ffmpeg-9.0.2.tar.xz ] || curl -fL --retry 5 -o ffmpeg-9.0.2.tar.xz https://ffmpeg.org/releases/ffmpeg-9.0.2.tar.xz
echo '8c3850283eb25fa026482078a04051e0be17347b09ef81a0849bec15a96e002e  ffmpeg-9.0.2.tar.xz' | sha256sum -c
rm -rf ffmpeg-9.0.2 && tar xf ffmpeg-9.0.2.tar.xz && cd ffmpeg-9.0.2
./configure --prefix=$P --libdir=$P/lib \
  --enable-cross-compile --target-os=darwin --arch=aarch64 \
  --cc=$CC --cxx=$CXX --ar=$AR --ranlib=$RANLIB --nm=$NM --strip=$STRIP \
  --pkg-config=pkg-config --pkg-config-flags=--static \
  --enable-static --disable-shared --enable-pic --disable-doc --disable-programs \
  --disable-autodetect --disable-network --disable-everything \
  --disable-avdevice --disable-swresample --enable-avformat \
  --enable-decoder=h264,hevc --enable-parser=h264,hevc \
  --enable-filter=buffer,buffersink,format,scale,zscale,tonemap \
  --enable-libzimg \
  --enable-videotoolbox --enable-hwaccel=h264_videotoolbox,hevc_videotoolbox >/tmp/ff.cfg 2>&1 || { tail -40 ffbuild/config.log; exit 1; }
grep -E 'videotoolbox|zimg' /tmp/ff.cfg | head
make -j$J >/tmp/ff.make 2>&1 || { tail -40 /tmp/ff.make; exit 1; }
make install >/dev/null; cd ..
step done
ls $P/lib; cat $P/lib/pkgconfig/libavcodec.pc | grep -i libs

