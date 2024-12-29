# Simple Image Viewer
Simple minimal image viewer, built using SDL2.

## Usage
Requires SDL2 and SDL2_image.

Point the binary to an image, or a folder containing images.
`$binary path/to/image/or/folder/of/images`

Navigate with left/right arrow keys.
Close with Q.

Supported image formats: `jpg, png, webp`.

## Known Issues
- Panics when attempting to open an image that was in the folder at startup, but has been renamed/deleted/moved since
- Does not recognize images that have been added after startup
- only tested on Linux
- in retrospect, SDL2 was not a good choice for providing a statically linked executable
