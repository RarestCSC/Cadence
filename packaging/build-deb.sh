#!/usr/bin/env bash
# 仅构建 .deb：使用系统 dpkg-deb，不依赖 nfpm。
set -euo pipefail

version="${1:-0.1.0}"
arch="${2:-amd64}"
dist_dir="$(realpath -m "${3:-dist}")"
case "$arch" in
  amd64|arm64) ;;
  *) echo "unsupported architecture: $arch" >&2; exit 1 ;;
esac

cd "$(dirname "$0")/.."
cargo build --locked --release -p cadence
binary="target/release/cadence"
test -x "$binary"

glibc_version=$(readelf --version-info "$binary" \
  | grep -oE 'GLIBC_[0-9]+\.[0-9]+(\.[0-9]+)?' \
  | sed 's/GLIBC_//' | sort -Vu | tail -n 1)
test -n "$glibc_version"

stage="$(mktemp -d)"
trap 'rm -rf "$stage"' EXIT
chmod 0755 "$stage"
install -Dm755 "$binary" "$stage/usr/bin/cadence"
install -Dm644 packaging/linux/cadence.desktop \
  "$stage/usr/share/applications/cadence.desktop"
install -Dm644 packaging/linux/cadence.svg \
  "$stage/usr/share/icons/hicolor/scalable/apps/cadence.svg"
install -Dm644 README.md "$stage/usr/share/doc/cadence/README.md"
install -Dm644 THIRD_PARTY_NOTICES.md \
  "$stage/usr/share/doc/cadence/THIRD_PARTY_NOTICES.md"
install -Dm644 packaging/linux/license-unknown.txt \
  "$stage/usr/share/licenses/cadence/license-unknown.txt"

mkdir -p "$stage/DEBIAN"
installed_size=$(du -sk "$stage/usr" | cut -f1)
cat > "$stage/DEBIAN/control" <<EOF
Package: cadence
Version: $version
Section: sound
Priority: optional
Architecture: $arch
Installed-Size: $installed_size
Depends: libc6 (>= $glibc_version), libasound2, libfontconfig1, libgcc-s1, libglib2.0-0, libwayland-client0, libwayland-egl1, libegl1, libxcb1, libxkbcommon0, libxkbcommon-x11-0, libvulkan1
Maintainer: CSC <rarestcsc@qq.com>
Homepage: https://github.com/RarestCSC/Cadence
Description: Native QQ Music desktop client built with Rust and GPUI
 Cadence is an unofficial QQ Music desktop client built with Rust and GPUI.
EOF
chmod 0644 "$stage/DEBIAN/control"

(cd "$stage" && find usr -type f -print0 | sort -z | xargs -0 md5sum > DEBIAN/md5sums)

mkdir -p "$dist_dir"
target="$dist_dir/cadence-v$version-linux-$arch.deb"
dpkg-deb --build --root-owner-group "$stage" "$target"
echo "$target"
