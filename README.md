# Simple Image Viewer
Simple minimal image viewer, built using SDL2.

## Usage
Point the binary to an image, or a folder containing images.
`$binary path/to/image/or/folder/of/images`

Navigate with left/right arrow keys.
Close with Q.

Supported image formats: `jpg, png, webp`.

## Known Issues
- Panics when an image in the currently open folder is deleted
- Does not recognize images that are added during runtime
- (only tested on Linux so far)
