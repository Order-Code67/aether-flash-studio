"""Generate Aether Flash Studio brand raster icons from the geometric mark."""
from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw

ROOT = Path(__file__).resolve().parents[1]
BRAND = ROOT / "assets" / "brand"
TAURI = ROOT / "src-tauri" / "icons"
PUBLIC = ROOT / "public"


def lerp(a: tuple[int, int, int], b: tuple[int, int, int], t: float) -> tuple[int, int, int]:
    return tuple(int(a[i] + (b[i] - a[i]) * t) for i in range(3))  # type: ignore[return-value]


TEAL = (25, 198, 193)
BLUE = (67, 141, 255)
BG = (16, 19, 24)


def draw_mark(size: int) -> Image.Image:
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    s = size / 64.0

    def xy(*pts: float) -> list[tuple[float, float]]:
        return [(pts[i] * s, pts[i + 1] * s) for i in range(0, len(pts), 2)]

    # Rounded frame
    margin = 4 * s
    stroke = max(2, int(3 * s))
    for i in range(stroke):
        color = lerp(TEAL, BLUE, i / max(stroke - 1, 1))
        inset = margin + i * 0.35
        draw.rounded_rectangle(
            [inset, inset, size - inset, size - inset],
            radius=max(2, 10 * s - i * 0.2),
            outline=color + (255,),
            width=1,
        )

    # Flash bolt
    bolt = xy(34, 14, 26, 32, 34, 32, 28, 50, 42, 28, 33, 28, 38, 14)
    draw.polygon(bolt, fill=lerp(TEAL, BLUE, 0.35) + (255,))

    # Radio arcs
    for radius, color, width in (
        (10 * s, BLUE + (255,), max(1, int(2.2 * s))),
        (15 * s, TEAL + (230,), max(1, int(2.2 * s))),
    ):
        bbox = [
            32 * s - radius,
            32 * s - radius,
            32 * s + radius + 18 * s,
            32 * s + radius,
        ]
        # Approximate arcs with short polylines on the right side
        pts = []
        import math

        for deg in range(-50, 51, 4):
            rad = math.radians(deg)
            cx = 36 * s
            cy = 32 * s
            r = radius + (4 * s if color[0] == TEAL[0] else 0)
            pts.append((cx + r * math.cos(rad), cy + r * math.sin(rad)))
        if len(pts) >= 2:
            draw.line(pts, fill=color, width=width, joint="curve")

    # Signal node
    r = max(1.5, 2.2 * s)
    draw.ellipse([20 * s - r, 32 * s - r, 20 * s + r, 32 * s + r], fill=TEAL + (255,))
    return img


def save_png(img: Image.Image, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    img.save(path, format="PNG")


def main() -> None:
    BRAND.mkdir(parents=True, exist_ok=True)
    TAURI.mkdir(parents=True, exist_ok=True)
    PUBLIC.mkdir(parents=True, exist_ok=True)

    master = draw_mark(1024)
    sizes = {
        BRAND / "icon-16.png": 16,
        BRAND / "icon-32.png": 32,
        BRAND / "icon-256.png": 256,
        TAURI / "32x32.png": 32,
        TAURI / "128x128.png": 128,
        TAURI / "128x128@2x.png": 256,
        TAURI / "icon.png": 512,
        TAURI / "Square30x30Logo.png": 30,
        TAURI / "Square44x44Logo.png": 44,
        TAURI / "Square71x71Logo.png": 71,
        TAURI / "Square89x89Logo.png": 89,
        TAURI / "Square107x107Logo.png": 107,
        TAURI / "Square142x142Logo.png": 142,
        TAURI / "Square150x150Logo.png": 150,
        TAURI / "Square284x284Logo.png": 284,
        TAURI / "Square310x310Logo.png": 310,
        TAURI / "StoreLogo.png": 50,
        PUBLIC / "aether.svg": None,  # skip
    }

    for path, size in sizes.items():
        if size is None:
            continue
        save_png(master.resize((size, size), Image.Resampling.LANCZOS), path)

    # favicon.ico + icon.ico multi-size
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    ico_images = [master.resize((s, s), Image.Resampling.LANCZOS) for s in ico_sizes]
    ico_images[0].save(
        BRAND / "favicon.ico",
        format="ICO",
        sizes=[(s, s) for s in ico_sizes],
        append_images=ico_images[1:],
    )
    ico_images[0].save(
        TAURI / "icon.ico",
        format="ICO",
        sizes=[(s, s) for s in ico_sizes],
        append_images=ico_images[1:],
    )
    # public copies
    (PUBLIC / "brand").mkdir(parents=True, exist_ok=True)
    for name in ("icon.svg", "logo.svg", "logo-horizontal.svg"):
        src = BRAND / name
        if src.exists():
            (PUBLIC / "brand" / name).write_text(src.read_text(encoding="utf-8"), encoding="utf-8")
    save_png(master.resize((32, 32), Image.Resampling.LANCZOS), PUBLIC / "favicon.png")
    ico_images[0].save(PUBLIC / "favicon.ico", format="ICO", sizes=[(16, 16), (32, 32)])
    # Replace public aether.svg with brand icon
    (PUBLIC / "aether.svg").write_text((BRAND / "icon.svg").read_text(encoding="utf-8"), encoding="utf-8")
    print("Brand icons generated.")


if __name__ == "__main__":
    main()
