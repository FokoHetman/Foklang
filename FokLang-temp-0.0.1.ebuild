# Copyright foko 2049
# Distributed under the terms of the AUTHOR DOES NOT SIGN BELOW EXISTENCE OF THIS THING BECAUSE TF IS THAT CODE :tm:
 
EAPI=8
 
DESCRIPTION="Foklang - a definitely working language that is... nvm it panics 90% of the time without a reason"
HOMEPAGE="https://github.com/fokohetman/foklang-temp"
SRC_URI="https://github.com/fokohetman/${PN}/archive/${P}.tar.gz"
 
LICENSE="GPL-2" # I don't know which license is that but I am afraid changing it will break soemthing
SLOT="0"
KEYWORDS="amd64 x86" # I don't kno what that is either
IUSE=""

S="${WORKDIR}/${PN}-${P}"

DEPEND="dev-util/rustup" # Nothing (actully rust)
RDEPEND="${DEPEND}"
BDEPEND=""

src_install() {
  make DESTDIR=${D} install || die "make faield for some fucking reason"
}
