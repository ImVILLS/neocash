# Maintainer: ImVILLS <cloxi19@gmail.com>
pkgname=neocash
pkgver=1.0.2
pkgrel=1
pkgdesc="New Era of Community-Adaptive SHell"
arch=('x86_64')
url="https://github.com/ImVILLS/neocash"
license=('MIT')
depends=('rust' 'git')
makedepends=('cargo')
source=("git+$url.git#tag=v$pkgver")
sha256sums=('SKIP')

build() {
  cd "$srcdir/neocash"
  cargo build --release --locked
}

package() {
  cd "$srcdir/neocash"
  install -Dm755 "target/release/neocash" "$pkgdir/usr/bin/neocash"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
