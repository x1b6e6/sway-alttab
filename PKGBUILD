# Maintainer: x1b6e6 <ftdabcde@gmail.com>
pkgname=sway-alttab
pkgver=0.1.0
pkgrel=1
pkgdesc='Alt-Tab implementaion for sway'
url='https://github.com/x1b6e6/sway-alttab'
source=("$pkgname-$pkgver.tar.gz::$url/archive/v$pkgver.tar.gz")
arch=('i686' 'pentium4' 'x86_64' 'arm' 'armv7h' 'armv6h' 'aarch64')
license=('GPL3')
makedepends=('cargo')
depends=('libevdev')
sha256sums=('795f50fd015d5331071296f9d7b019f3051941a0d86f344be2fe9d7dd5040a3a')

build () {
  cd "$srcdir/$pkgname-$pkgver"

  cargo build --release --target-dir target
}

package() {
  cd "$srcdir/$pkgname-$pkgver"

  install -Dm4755 target/release/$pkgname "${pkgdir}/usr/bin/$pkgname"
}
