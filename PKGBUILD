# Maintainer: x1b6e6 <ftdabcde@gmail.com>
pkgname=sway-alttab
pkgver=0.2.0
pkgrel=1
pkgdesc='Alt-Tab implementaion for sway'
url='https://github.com/x1b6e6/sway-alttab'
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
arch=('i686' 'pentium4' 'x86_64' 'arm' 'armv7h' 'armv6h' 'aarch64')
license=('GPL3')
makedepends=('cargo')
depends=('libevdev')
sha256sums=('7dabc40a7e05f3b3dcb4aefe984de9dd67e9027139d2a812886c42c19ce328b0')

build () {
  cd "$srcdir/$pkgname-$pkgver"

  cargo build --release --target-dir target
}

package() {
  cd "$srcdir/$pkgname-$pkgver"

  install -Dm4755 target/release/$pkgname "${pkgdir}/usr/bin/$pkgname"
}
