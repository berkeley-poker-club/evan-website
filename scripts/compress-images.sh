#!/bin/bash
set -e

QUALITY=85
MAX_WIDTH=2000

compress_folder() {
    local folder=$1
    [ ! -d "$folder" ] && return

    for img in "$folder"/*.jpg "$folder"/*.jpeg "$folder"/*.JPG "$folder"/*.JPEG; do
        [ -f "$img" ] || continue
        original_size=$(stat -f%z "$img" 2>/dev/null || stat -c%s "$img")
        [ "$((original_size / 1024))" -lt 1024 ] && continue

        magick "$img" -resize "${MAX_WIDTH}x${MAX_WIDTH}>" -quality "$QUALITY" -strip -interlace Plane "$img"
    done
}

compress_folder "public/images/falltourney"
compress_folder "public/images/stanfxcal25"
compress_folder "public/images/officergroup"

echo "Compression complete"
du -sh public/images/
