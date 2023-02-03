pkgname="audlyrics"
pkgver="1.0.0"
pkgrel="1"
pkgdesc="A plasmoid and server for displaying lyrics of audacious"
arch=("x86_64" "arm")
depends=("audacious" "rustup" "plasma-desktop")
license=("None")
source=("contents/ui/main.qml" "src/main.rs" "src/lib.rs" "src"
        "Cargo.toml" "AudLyrics.desktop" "Cargo.lock" "metadata.json")

package() {
    mkdir -p "${pkgdir}/usr/share/applications"
    cp "${srcdir}/AudLyrics.desktop" "${pkgdir}/usr/share/applications/"
    
    cd "${srcdir}"
    mkdir -p "${pkgdir}/usr/bin"
    cargo build --release --target-dir "${pkgdir}/usr/bin/"

    mkdir -p "${pkgdir}/usr/share/plasma/plasmoids/org.kde.audlyrics"
    cp "${srcdir}/metadata.json" "${pkgdir}/usr/share/plasma/plasmoids/org.kde.audlyrics/"
    cp -r "${srcdir}/content" "${pkgdir}/usr/share/plasma/plasmoids/org.kde.audlyrics/"
}
