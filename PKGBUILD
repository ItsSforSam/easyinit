
# Maintainer: SforSam <81501696+ItsSforSam@users.noreply.github.com>
_pkgname="easyinit"
pkgname="${_pkgname}-git" # '-bzr', '-git', '-hg' or '-svn'
pkgver=VERSION
# pkgrel=1
pkgdesc="An alternative"
arch=('i686' 'x86_64' 'armv6h' 'armv7h') 
url="https://github.com/ItsSforSam/easyinit"
license=("LGPL-3.0-or-later")
groups=()
depends=()
makedepends=("git","cargo") # 'bzr', 'git', 'mercurial' or 'subversion'
provides=("easyinit=${pkgver}")
# when we are stable and have a stable package, we cannot have 
conflicts=("easyinit")
# replaces=()
# backup=()
# options=()
# install=
source=("${_pkgname}::git+https://github.com/ItsSforSam/easyinit.git")
noextract=()
sha256sums=("SKIP")

# Please refer to the 'USING VCS SOURCES' section of the PKGBUILD man page for
# a description of each element in the source array.

# helper function to ensure we are using the correct environment variables to ensure we have everything where expected
_export_env(){
	# don't need to define the toolchain, we have the "rust-toolchain.toml" file
	# export RUSTUP_TOOLCHAIN="stable"
	export CARGO_TARGET_DIR="target"
}
# used to be passed into the --features flag into cargo.
# storing into variable so tests, checks, and the final binaries use the same features
#
# comma separated with all the necessary features
# _features=""
pkgver() {
	cd "$srcdir/${$_pkgname}"

# The examples below are not absolute and need to be adapted to each repo. The
# primary goal is to generate version numbers that will increase according to
# pacman's version comparisons with later commits to the repo. The format
# VERSION='VER_NUM.rREV_NUM.HASH', or a relevant subset in case VER_NUM or HASH
# are not available, is recommended.


# # Git, tags available
# 	printf "%s" "$(git describe --long | sed 's/\([^-]*-\)g/r\1/;s/-/./g')"

# Git, no tags available
	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"

}

prepare() {
	# https://wiki.archlinux.org/title/Rust_package_guidelines#Prepare
	# the rustc
	cd "$srcdir/${$_pkgname}"
	cargo fetch --locked --target "$(rustc -vV | sed -n 's/host: //p')"
	
}

build() {
	cd "$srcdir/${$_pkgname}"
	cargo build --frozen --release # --features="${_features}"
}

check() {
	cd "$srcdir/${$_pkgname}"
	cargo test --frozen # --features="${_features}"
}

package() {
	install -Dm0755 -t "$pkgdir/sbin" "target/release/$_pkgname"
	install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${_pkgname}/LICENSE"

}
