# Maintainer: ImVILLS <cloxi19@gmail.com>
pkgname=neocash
pkgver=1.0.1
pkgrel=1
pkgdesc="NeoCASH shell (build from source)"
arch=('x86_64')
url="https://github.com/yourname/neocash"
license=('MIT')
makedepends=('cargo')
source=("https://github.com/ImVILLS/neocash/archive/v$pkgver.tar.gz")
sha256sums=('351d359c4715dfbbb3621172ab70561cc5577ff8e072de903f0ee739c8703218')

build() {
  cd "$srcdir/neocash-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/neocash-$pkgver"
  install -Dm755 "target/release/neocash" "$pkgdir/usr/bin/neocash"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
