# Maintainer: ImVILLS <cloxi19@gmail.com>
pkgname=neocash
pkgver=1.0.0
pkgrel=1
pkgdesc="NeoCASH shell (build from source)"
arch=('x86_64')
url="https://github.com/yourname/neocash"
license=('MIT')
makedepends=('cargo')
source=("https://github.com/ImVILLS/neocash/archive/v$pkgver.tar.gz")
sha256sums=("91ec1c23276f19e4dcf7cada61d1e67d8c0f2f6f5e9bbb80a77d933a6075d382")

build() {
  cd "$srcdir/neocash-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/neocash-$pkgver"
  install -Dm755 "target/release/neocash" "$pkgdir/usr/bin/neocash"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
