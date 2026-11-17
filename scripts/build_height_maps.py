#!/usr/bin/env python3
"""Rebuilds the chip's height maps from the flat art, so they line up with it pixel for pixel.

Reads  public/images/chip/{front,back}_flat.png
Writes public/images/chip/{front,back}_height.png    16-bit grayscale, same size as the flat image
       scripts/chip_measurements.json                disc and rim block angles, read by build_chip.py

The first run keeps the old maps as {front,back}_height_old.png.

Height levels: recessed outlines < base plastic < raised art.

Needs numpy and Pillow:
    python3 -m venv /tmp/chip-venv && /tmp/chip-venv/bin/pip install numpy pillow
    /tmp/chip-venv/bin/python scripts/build_height_maps.py
"""
import json
import shutil
from pathlib import Path

import numpy as np
from PIL import Image

ROOT = Path(__file__).resolve().parent.parent
CHIP_DIR = ROOT / "public" / "images" / "chip"
MEASUREMENTS = ROOT / "scripts" / "chip_measurements.json"

HEIGHT_RECESSED = 0.20
HEIGHT_BASE = 0.50
HEIGHT_RAISED = 1.00

BLUR_SIGMA = 1.5       # px, soft molded edges
GRAIN_AMOUNT = 0.004   # plastic grain, standard deviation as a fraction of full height
GRAIN_SIGMA = 0.7      # px, keeps the grain fine but not single-pixel noise

# How much darker than the base plastic a pixel must be to count as an outline.
RECESS_START = 0.12
RECESS_FULL = 0.30

BLOCK_RING = (0.82, 0.97)  # radii, as fractions of the disc, where only rim blocks and dice can be
BLOCK_COVERAGE = 0.75      # fraction of that ring that must be art for an angle to be inside a block
EXPECTED_BLOCKS = 6


def smoothstep(x, lo, hi):
    t = np.clip((x - lo) / (hi - lo), 0, 1)
    return t * t * (3 - 2 * t)


def luminance(rgb):
    return rgb[..., 0] * 0.2126 + rgb[..., 1] * 0.7152 + rgb[..., 2] * 0.0722


def gaussian_blur(a, sigma):
    radius = int(np.ceil(sigma * 3))
    kernel = np.exp(-0.5 * (np.arange(-radius, radius + 1) / sigma) ** 2)
    kernel /= kernel.sum()
    p = np.pad(a, radius, mode="edge")
    p = np.apply_along_axis(lambda v: np.convolve(v, kernel, mode="valid"), 1, p)
    return np.apply_along_axis(lambda v: np.convolve(v, kernel, mode="valid"), 0, p)


def key_mask(rgb):
    """True where the pixel is the solid green background."""
    return (rgb[..., 1] > 0.6) & (rgb[..., 0] < 0.35) & (rgb[..., 2] < 0.35)


def fit_disc(chip):
    """Least-squares circle through the boundary between the chip and the background."""
    h, w = chip.shape
    edge = chip & ~(
        np.roll(chip, 1, 0) & np.roll(chip, -1, 0) & np.roll(chip, 1, 1) & np.roll(chip, -1, 1)
    )
    edge[0, :] = edge[-1, :] = edge[:, 0] = edge[:, -1] = False  # the image border isn't the chip's edge
    ys, xs = np.nonzero(edge)
    xs, ys = xs + 0.5, ys + 0.5
    for _ in range(3):  # refit twice without outliers
        A = np.column_stack([2 * xs, 2 * ys, np.ones_like(xs)])
        cx, cy, c = np.linalg.lstsq(A, xs * xs + ys * ys, rcond=None)[0]
        r = np.sqrt(c + cx * cx + cy * cy)
        keep = np.abs(np.hypot(xs - cx, ys - cy) - r) < 2
        xs, ys = xs[keep], ys[keep]
    return float(cx), float(cy), float(r)


def dominant_color(pixels):
    q = (pixels * 31).round().astype(int)
    keys = q[:, 0] * 1024 + q[:, 1] * 32 + q[:, 2]
    values, counts = np.unique(keys, return_counts=True)
    return pixels[keys == values[counts.argmax()]].mean(axis=0)


def block_angles(raised, disc):
    """Centers and widths of the rim blocks, in degrees (0 = 3 o'clock, counterclockwise on screen)."""
    cx, cy, r = disc
    h, w = raised.shape
    radii = np.linspace(BLOCK_RING[0], BLOCK_RING[1], 24) * r
    angles = np.radians(np.arange(0, 360, 0.1))
    xs = np.clip((cx + np.outer(radii, np.cos(angles))).astype(int), 0, w - 1)
    ys = np.clip((cy - np.outer(radii, np.sin(angles))).astype(int), 0, h - 1)
    inside = (raised[ys, xs] > 0.5).mean(axis=0) >= BLOCK_COVERAGE

    # Start the scan just outside a block, and close the last one, so none is lost at the wraparound.
    start = int(np.argmin(inside))
    rolled = np.append(np.roll(inside, -start), False)
    changes = np.flatnonzero(np.diff(rolled.astype(int)))
    blocks = []
    for lo, hi in zip(changes[::2], changes[1::2]):
        center = ((lo + hi + 1) / 2 + start) * 0.1 % 360
        blocks.append({"center": round(float(center), 2), "width": round(float((hi - lo) * 0.1), 2)})
    return sorted(blocks, key=lambda b: b["center"])


def build(face, rng):
    flat_path = CHIP_DIR / f"{face}_flat.png"
    rgb = np.asarray(Image.open(flat_path).convert("RGB"), dtype=np.float32) / 255
    h, w, _ = rgb.shape
    disc = fit_disc(~key_mask(rgb))
    cx, cy, r = disc

    rows, cols = np.mgrid[:h, :w]
    dist = np.hypot(cols + 0.5 - cx, rows + 0.5 - cy) / r
    inner = dist < 0.985  # clear of the green-tinted edge pixels

    base = dominant_color(rgb[inner])
    far = np.linalg.norm(rgb - base, axis=-1) > 0.35
    art = dominant_color(rgb[inner & far & (luminance(rgb) > luminance(base))])

    # How far each pixel sits along the base-to-art mix. This keeps the art's antialiased edges soft.
    axis = art - base
    mix = np.clip(((rgb - base) * axis).sum(axis=-1) / (axis * axis).sum(), 0, 1)
    raised = smoothstep(mix, 0.25, 0.75)

    # Darker than the base plastic, and not part of an edge of the art.
    darkness = (luminance(base) - luminance(rgb)) / luminance(base)
    recessed = smoothstep(darkness, RECESS_START, RECESS_FULL) * (mix < 0.1)

    height = HEIGHT_BASE + (HEIGHT_RAISED - HEIGHT_BASE) * raised + (HEIGHT_RECESSED - HEIGHT_BASE) * recessed
    height = np.where(dist < 0.995, height, HEIGHT_BASE)
    height = gaussian_blur(height, BLUR_SIGMA)

    grain = gaussian_blur(rng.standard_normal((h, w)), GRAIN_SIGMA)
    height = height + grain / grain.std() * GRAIN_AMOUNT

    out = CHIP_DIR / f"{face}_height.png"
    backup = CHIP_DIR / f"{face}_height_old.png"
    if out.exists() and not backup.exists():
        shutil.copy2(out, backup)
    pixels = np.round(np.clip(height, 0, 1) * 65535).astype(np.uint16)
    Image.fromarray(pixels).save(out)

    blocks = block_angles(raised, disc)
    if len(blocks) != EXPECTED_BLOCKS:
        raise SystemExit(f"{face}: found {len(blocks)} rim blocks, expected {EXPECTED_BLOCKS}: {blocks}")

    print(
        f"{face}: disc ({cx:.1f}, {cy:.1f}) r={r:.1f}, "
        f"base {np.round(base * 255).astype(int).tolist()}, art {np.round(art * 255).astype(int).tolist()}, "
        f"raised {(raised[inner] > 0.5).mean() * 100:.1f}%, recessed {(recessed[inner] > 0.5).mean() * 100:.1f}%"
    )
    print(f"  blocks: {[(b['center'], b['width']) for b in blocks]}")
    return {
        "image_size": [w, h],
        "disc": {"x": round(cx, 2), "y": round(cy, 2), "r": round(r, 2)},
        "base_rgb": np.round(base * 255).astype(int).tolist(),
        "art_rgb": np.round(art * 255).astype(int).tolist(),
        "blocks": blocks,
    }


if __name__ == "__main__":
    rng = np.random.default_rng(1)  # fixed seed, so re-running gives the same files
    results = {face: build(face, rng) for face in ("front", "back")}
    MEASUREMENTS.write_text(json.dumps(results, indent=2) + "\n")
    print(f"wrote {MEASUREMENTS.relative_to(ROOT)}")
