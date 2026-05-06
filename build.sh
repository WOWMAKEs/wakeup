#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"

echo "Generating app icon..."

python3 -c "
import struct, zlib, os, math

def create_power_icon_png(size):
    center = size / 2.0
    ring_radius = size * 0.36
    ring_thickness = size * 0.13
    gap_half_angle = 0.55
    stem_half_width = size * 0.08
    stem_top = center - ring_radius + ring_thickness * 0.5
    stem_bottom = center - size * 0.07

    pixels = bytearray()
    for y in range(size):
        for x in range(size):
            px = x + 0.5
            py = y + 0.5
            dx = px - center
            dy = py - center
            dist = (dx*dx + dy*dy) ** 0.5
            angle = math.atan2(dy, dx)

            in_ring = (dist >= ring_radius - ring_thickness/2.0 and
                       dist <= ring_radius + ring_thickness/2.0 and
                       not (angle > -gap_half_angle and angle < gap_half_angle))

            in_stem = (px >= center - stem_half_width and
                       px <= center + stem_half_width and
                       py >= stem_top and py <= stem_bottom)

            if in_ring or in_stem:
                pixels.extend([0, 0, 0, 255])
            else:
                pixels.extend([0, 0, 0, 0])

    def make_chunk(chunk_type, data):
        chunk = chunk_type + data
        crc = zlib.crc32(chunk) & 0xffffffff
        return struct.pack('>I', len(data)) + chunk + struct.pack('>I', crc)

    sig = b'\x89PNG\r\n\x1a\n'
    ihdr = struct.pack('>IIBBBBB', size, size, 8, 6, 0, 0, 0)

    raw_data = bytearray()
    for y in range(size):
        raw_data.append(0)
        raw_data.extend(pixels[y*size*4:(y+1)*size*4])

    compressed = zlib.compress(bytes(raw_data))

    return sig + make_chunk(b'IHDR', ihdr) + make_chunk(b'IDAT', compressed) + make_chunk(b'IEND', b'')

iconset_path = os.path.join(os.environ.get('SCRIPT_DIR', '.'), 'wakeUP.iconset')
os.makedirs(iconset_path, exist_ok=True)

entries = [(16, 'icon_16x16'), (32, 'icon_16x16@2x'), (32, 'icon_32x32'), (64, 'icon_32x32@2x'),
           (128, 'icon_128x128'), (256, 'icon_128x128@2x'), (256, 'icon_256x256'), (512, 'icon_256x256@2x'),
           (512, 'icon_512x512')]

for sz, name in entries:
    png_data = create_power_icon_png(sz)
    filepath = os.path.join(iconset_path, name + '.png')
    with open(filepath, 'wb') as f:
        f.write(png_data)

print('Iconset generated.')
"

iconutil -c icns wakeUP.iconset -o wakeUP.icns
rm -rf wakeUP.iconset

echo "Icon generated: wakeUP.icns"
echo ""
echo "Building release..."
cargo bundle --release

echo ""
echo "Done! App at: target/release/bundle/osx/wakeUP.app"
