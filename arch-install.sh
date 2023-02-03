# Build server
cd server
cargo build --release
cd ../

# Make server package
makepkg -f

# Install plasma package
plasmapkg2 -t Plasma/Applet -i plasmoid

# Install server package
sudo pacman -U audlyrics-*.pkg.tar.zst
