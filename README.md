# Simple Image Viewer
Simple minimal image viewer, built using SDL2.

## Usage
Requires SDL2 and SDL2_image.

Point the binary to an image, or a folder containing images.
`simple-image-viewer path/to/image/or/folder/of/images`

- Navigate with left/right arrow keys.
- Close with Q.
- Supported image formats: `jpg, png, webp`.

## Installation
- Run `cargo install --path /path/to/crate` to install `simple-image-viewer` (will usually be placed in `~/.cargo/bin`).
- Move the provided .desktop file to `~/.local/share/applications`.
- Run: `update-desktop-database ~/.local/share/applications`.

## Known Issues
- Panics when attempting to open an image that was in the folder at startup, but has been renamed/deleted/moved since
- Does not recognize images that have been added after startup
- only tested on Linux
- in retrospect, SDL2 was not a good choice for providing a statically linked executable
