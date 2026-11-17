#!/usr/bin/env python3
"""One-time export of the chip's normal and roughness maps, used by public/hero-chip.html.

Re-run this only if front_height.png or back_height.png change. If the images move or
resize, update the `disc` values here AND in FRONT_MAPS / BACK_MAPS in hero-chip.html.

Needs numpy and Pillow:
    python3 -m venv /tmp/chip-venv && /tmp/chip-venv/bin/pip install numpy pillow
    /tmp/chip-venv/bin/python scripts/bake_chip_maps.py

Writes to public/images/chip/:
    front_normal.png, back_normal.png            tangent-space normals (green = up)
    front_rough_final.png, back_rough_final.png  roughness (0-255 = 0-1)
"""
from pathlib import Path

import numpy as np
from PIL import Image

CHIP_DIR = Path(__file__).resolve().parent.parent / "public" / "images" / "chip"

# The chip's circle in each source image, in image pixels (x, y, radius).
DISCS = {
    "front": (625.8, 624.0, 618.3),
    "back": (545.2, 543.5, 543.4),
}

SIZE = 2048            # output size, in pixels
MASK_INSET = 0.01      # must match FACE_MASK_INSET in hero-chip.html, so the maps line up
EDGE_FADE = 0.015      # relief fades flat over this outer fraction, so the faces meet the rim cleanly
SOBEL_STRENGTH = 8     # slope steepness baked into the normal map (NORMAL_SCALE adjusts it at runtime)

ROUGHNESS_RAISED = 0.4
ROUGHNESS_BASE = 0.6
ROUGHNESS_NOISE = 0.02  # random variation, plus or minus
RAISED_START = 20 / 255  # height above the base plastic where "raised" begins...
RAISED_FULL = 60 / 255   # ...and where it is fully raised


def smoothstep(x, lo, hi):
    t = np.clip((x - lo) / (hi - lo), 0, 1)
    return t * t * (3 - 2 * t)


def bake(face, rng):
    x, y, radius = DISCS[face]
    r = radius * (1 - MASK_INSET)

    # Float mode, so upscaling from 8 bits gives smooth slopes rather than stair-steps.
    source = Image.open(CHIP_DIR / f"{face}_height.png").convert("L").convert("F")
    crop = source.resize((SIZE, SIZE), Image.LANCZOS, box=(x - r, y - r, x + r, y + r))
    height = np.clip(np.asarray(crop) / 255, 0, 1)

    rows, cols = np.mgrid[:SIZE, :SIZE]
    dist = np.hypot(cols + 0.5 - SIZE / 2, rows + 0.5 - SIZE / 2) / (SIZE / 2)

    # The base plastic is the most common height inside the disc.
    histogram = np.bincount((height[dist < 1] * 255).astype(int), minlength=256)
    base = histogram.argmax() / 255

    # Fade to the base level at the edge and beyond it.
    keep = 1 - smoothstep(dist, 1 - EDGE_FADE, 1)
    height = base + (height - base) * keep

    # Sobel.
    p = np.pad(height, 1, mode="edge")
    gx = (p[:-2, 2:] + 2 * p[1:-1, 2:] + p[2:, 2:] - p[:-2, :-2] - 2 * p[1:-1, :-2] - p[2:, :-2]) / 8
    gy = (p[2:, :-2] + 2 * p[2:, 1:-1] + p[2:, 2:] - p[:-2, :-2] - 2 * p[:-2, 1:-1] - p[:-2, 2:]) / 8

    # Image Y runs down and texture V runs up, so green takes +gy.
    nx, ny = -gx * SOBEL_STRENGTH, gy * SOBEL_STRENGTH
    length = np.sqrt(nx * nx + ny * ny + 1)
    normal = np.stack([nx / length, ny / length, 1 / length], axis=-1)
    normal_path = CHIP_DIR / f"{face}_normal.png"
    Image.fromarray(np.round((normal * 0.5 + 0.5) * 255).astype(np.uint8), "RGB").save(normal_path, optimize=True)

    raised = smoothstep(height - base, RAISED_START, RAISED_FULL)
    roughness = ROUGHNESS_BASE + (ROUGHNESS_RAISED - ROUGHNESS_BASE) * raised
    roughness += rng.uniform(-ROUGHNESS_NOISE, ROUGHNESS_NOISE, roughness.shape)
    rough_path = CHIP_DIR / f"{face}_rough_final.png"
    Image.fromarray(np.round(np.clip(roughness, 0, 1) * 255).astype(np.uint8), "L").save(rough_path, optimize=True)

    print(f"{face}: base height {base * 255:.0f}/255, "
          f"raised {(raised > 0.5).mean() * 100:.0f}% of the square, "
          f"normal {normal_path.stat().st_size / 1e6:.1f} MB, "
          f"roughness {rough_path.stat().st_size / 1e6:.1f} MB")


if __name__ == "__main__":
    rng = np.random.default_rng(1)  # fixed seed, so re-running gives the same files
    for face in DISCS:
        bake(face, rng)
