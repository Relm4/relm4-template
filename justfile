builddir := "builddir"
profile := "debug"
prefix := "~/.local"
bin := "gtk-rust-template"

flatpak_id := "com.belmoussaoui.GtkRustTemplate"
flatpak_manifest := "com.belmoussaoui.GtkRustTemplate.json"
flatpak_builddir := "build-flatpak"

meson_flags := "-Dbuildtype=" + profile + " -Dprefix=" + prefix

# Configure meson build directory
setup:
    @if [ ! -f {{ builddir }}/build.ninja ]; then \
        meson setup {{ builddir }} {{ meson_flags }}; \
    elif ! meson configure {{ builddir }} | grep -q "buildtype.*{{ profile }}"; then \
        meson setup {{ builddir }} --reconfigure {{ meson_flags }}; \
    fi

# Reconfigure existing build directory
reconfigure:
    meson setup {{ builddir }} --reconfigure {{ meson_flags }}

# Build the project
build: setup
    meson compile -C {{ builddir }}

# Install to local prefix
install: build
    meson install -C {{ builddir }}

# Build, install, and run the app
run: install
    RUST_LOG={{ bin }}=DEBUG \
    GSETTINGS_SCHEMA_DIR={{ prefix }}/share/glib-2.0/schemas \
    {{ prefix }}/bin/{{ bin }}

# Clean build directory
clean:
    rm -rf {{ builddir }} \
    {{ prefix }}/bin/{{ bin }}

# Watch for changes and rebuild
watch:
    bacon

# Run clippy lints
lint:
    cargo clippy --manifest-path Cargo.toml

# Format code
fmt:
    cargo fmt --manifest-path Cargo.toml

# Fix lints and format code
fix:
    cargo clippy --manifest-path Cargo.toml --fix --allow-dirty --allow-staged
    cargo fmt --manifest-path Cargo.toml

# Clean and reconfigure from scratch
rebuild: clean setup build

# Build the Flatpak and install it into the user installation (incremental, cached)
build-flatpak:
    flatpak-builder --user --install --force-clean --ccache \
        {{ flatpak_builddir }} {{ flatpak_manifest }}

# Build, install, and run the Flatpak
run-flatpak: build-flatpak
    flatpak run {{ flatpak_id }}

# Remove the Flatpak build dir + builder cache and uninstall the app
clean-flatpak:
    rm -rf {{ flatpak_builddir }} .flatpak-builder
    -flatpak uninstall --user -y {{ flatpak_id }}