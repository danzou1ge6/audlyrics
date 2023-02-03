pkgname="audlyrics"
pkgver="1.0.0"
pkgrel="1"
pkgdesc="A plasmoid and server for displaying lyrics of audacious"
arch=("x86_64" "arm")
depends=("audacious" "rustup" "plasma-desktop")
license=("None")
source=("audlyrics" "AudLyrics.desktop")
sha256sums=("SKIP" "SKIP")

package() {
    mkdir -p "${pkgdir}/usr/share/applications"
    cp "${srcdir}/AudLyrics.desktop" "${pkgdir}/usr/share/applications/"
    
    mkdir -p "${pkgdir}/usr/bin"
    cp "${srcdir}/audlyrics" "${pkgdir}/usr/bin/"
    chmod +x "${pkgdir}/usr/bin/audlyrics"

}
