pkgname=well-winget
pkgver=0.0.1
pkgrel=1
pkgdesc="Wayland overlay widget using GTK4 + WebKitGTK 6"
arch=('x86_64')
url="https://github.com/luis-codex/well-winget"
license=('MIT')
depends=('gtk4' 'webkitgtk-6.0')
makedepends=('rust' 'cargo' 'pkgconf')
source=()
sha256sums=()

build() {
  cd "$startdir"
  cargo build --release --locked
}

package() {
  cd "$startdir"
  install -Dm755 "target/release/well-winget" "$pkgdir/usr/bin/well-winget"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}

