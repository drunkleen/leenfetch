# Maintainer: DrunkLeen <snape@drunkleen.com>
pkgname=leenfetch
pkgver=0.2.3
pkgrel=1
pkgdesc="Fast, minimal, customizable system info tool in Rust (Neofetch alternative)"
arch=('x86_64' 'aarch64')
url="https://github.com/drunkleen/leenfetch"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
  cd "$srcdir/$pkgname-$pkgver"
  cargo build --release --locked
}

package() {
  cd "$srcdir/$pkgname-$pkgver"
  install -Dm755 "target/release/leenfetch" "$pkgdir/usr/bin/leenfetch"
  install -Dm644 "LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
  install -Dm644 "leenfetch.1" "$pkgdir/usr/share/man/man1/leenfetch.1"
  install -Dm644 "README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
}
