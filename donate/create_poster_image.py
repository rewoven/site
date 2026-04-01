#!/usr/bin/env python3
"""Generate Rewoven Donation Drive poster as a phone-friendly PNG for WhatsApp."""

from reportlab.lib.units import mm
from reportlab.lib.colors import HexColor, white
from reportlab.pdfgen import canvas
from reportlab.pdfbase import pdfmetrics
import subprocess
import os

DIR = os.path.dirname(os.path.abspath(__file__))
TEMP_PDF = os.path.join(DIR, "poster_temp.pdf")
OUTPUT = os.path.join(DIR, "poster.png")

# Phone-friendly: 1080x1920 (9:16 ratio) — standard story/WhatsApp size
# In PDF points: 1pt = 1px at 72dpi, but we'll render at higher DPI
W = 270 * mm  # ~1080px at 100dpi
H = 480 * mm  # ~1920px at 100dpi

# Colors
GREEN = HexColor("#10B981")
GREEN_DARK = HexColor("#064E3B")
GREEN_DEEPER = HexColor("#022C22")
GREEN_BG = HexColor("#0D3B2B")
HEART = HexColor("#E11D48")
GRAY = HexColor("#94A3B8")
GOLD = HexColor("#F59E0B")


def rr(c, x, y, w, h, r, fill):
    """Draw rounded rectangle."""
    p = c.beginPath()
    p.moveTo(x + r, y)
    p.lineTo(x + w - r, y)
    p.arcTo(x + w - r, y, x + w, y + r, r)
    p.lineTo(x + w, y + h - r)
    p.arcTo(x + w, y + h - r, x + w - r, y + h, r)
    p.lineTo(x + r, y + h)
    p.arcTo(x + r, y + h, x, y + h - r, r)
    p.lineTo(x, y + r)
    p.arcTo(x, y + r, x + r, y, r)
    p.close()
    c.setFillColor(fill)
    c.drawPath(p, fill=1, stroke=0)


def create():
    c = canvas.Canvas(TEMP_PDF, pagesize=(W, H))
    mx = 18 * mm  # margin

    # ── Background ──
    c.setFillColor(GREEN_DEEPER)
    c.rect(0, 0, W, H, fill=1, stroke=0)

    # Subtle decorative circles
    c.setFillColor(HexColor("#0A3D2E"))
    c.circle(-20 * mm, H - 40 * mm, 80 * mm, fill=1, stroke=0)
    c.circle(W + 15 * mm, 80 * mm, 60 * mm, fill=1, stroke=0)

    # ── Top badge ──
    y = H - 32 * mm
    bw = 72 * mm
    rr(c, (W - bw) / 2, y, bw, 10 * mm, 5 * mm, HexColor("#2A0A16"))
    c.setFillColor(HEART)
    c.setFont("Helvetica-Bold", 11)
    c.drawCentredString(W / 2, y + 3 * mm, "\u2764  DONATION DRIVE")

    # ── Rewoven logo text ──
    y -= 20 * mm
    c.setFillColor(GREEN)
    c.setFont("Helvetica-Bold", 16)
    c.drawCentredString(W / 2, y, "REWOVEN")

    # ── Main title ──
    y -= 22 * mm
    c.setFillColor(white)
    c.setFont("Helvetica-Bold", 42)
    c.drawCentredString(W / 2, y, "Give Clothes a")

    y -= 15 * mm
    c.setFillColor(GREEN)
    c.setFont("Helvetica-Bold", 42)
    c.drawCentredString(W / 2, y, "Second Life")

    y -= 20 * mm
    c.setFillColor(white)
    c.setFont("Helvetica-Bold", 34)
    c.drawCentredString(W / 2, y, "Give Kids a")

    y -= 14 * mm
    c.setFillColor(HEART)
    c.setFont("Helvetica-Bold", 34)
    c.drawCentredString(W / 2, y, "Better One")

    # ── Subtitle ──
    y -= 20 * mm
    c.setFillColor(GRAY)
    c.setFont("Helvetica", 13)
    c.drawCentredString(W / 2, y, "Donate your pre-loved clothes to")
    y -= 6 * mm
    c.drawCentredString(W / 2, y, "underprivileged children in the Philippines.")
    y -= 6 * mm
    c.drawCentredString(W / 2, y, "Receive a handmade reward in return.")

    # ── Divider ──
    y -= 12 * mm
    c.setStrokeColor(HexColor("#1A3D2E"))
    c.setLineWidth(1)
    c.line(mx + 20 * mm, y, W - mx - 20 * mm, y)

    # ── Stats ──
    y -= 18 * mm
    stats = [("92M", "tons textile\nwaste yearly"), ("31M", "Filipino children\nin poverty"), ("95%", "of clothes could\nbe reused")]
    sw = (W - 2 * mx) / 3
    for i, (num, label) in enumerate(stats):
        sx = mx + sw * i + sw / 2
        c.setFillColor(GREEN)
        c.setFont("Helvetica-Bold", 30)
        c.drawCentredString(sx, y, num)
        c.setFillColor(GRAY)
        c.setFont("Helvetica", 9)
        for j, line in enumerate(label.split("\n")):
            c.drawCentredString(sx, y - 7 * mm - j * 4 * mm, line)

    # ── How It Works ──
    y -= 32 * mm
    c.setFillColor(GREEN)
    c.setFont("Helvetica-Bold", 10)
    c.drawCentredString(W / 2, y, "HOW IT WORKS")

    steps = [
        ("1", "Gather Clothes", "Collect gently used clothing\nyou no longer wear"),
        ("2", "Drop Off or Ship", "Bring to a drop-off point\nor arrange a pickup"),
        ("3", "We Ship to Philippines", "Sorted, packed & sent\nto kids in need"),
        ("4", "Get Your Reward", "Receive a handmade\nhandbag or bookmark"),
    ]

    # 2x2 grid
    cw = (W - 2 * mx - 8 * mm) / 2
    ch = 42 * mm
    y -= 12 * mm

    for i, (num, title, desc) in enumerate(steps):
        col = i % 2
        row = i // 2
        sx = mx + col * (cw + 8 * mm)
        sy = y - row * (ch + 8 * mm) - ch

        rr(c, sx, sy, cw, ch, 5 * mm, GREEN_BG)

        # Number circle
        c.setFillColor(GREEN)
        c.circle(sx + 14 * mm, sy + ch - 12 * mm, 5 * mm, fill=1, stroke=0)
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 13)
        c.drawCentredString(sx + 14 * mm, sy + ch - 14 * mm, num)

        # Title
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 12)
        c.drawString(sx + 24 * mm, sy + ch - 13 * mm, title)

        # Desc
        c.setFillColor(GRAY)
        c.setFont("Helvetica", 8.5)
        for j, line in enumerate(desc.split("\n")):
            c.drawString(sx + 8 * mm, sy + ch - 24 * mm - j * 4 * mm, line)

    # ── Rewards ──
    y = y - 2 * (ch + 8 * mm) - 14 * mm
    c.setFillColor(GREEN)
    c.setFont("Helvetica-Bold", 10)
    c.drawCentredString(W / 2, y, "YOUR REWARD")

    rewards = [
        ("\U0001F45C", "Sustainable Handbag", "Unique tote bag from upcycled\nfabric offcuts. No two are alike.", "Donate 5+ items"),
        ("\U0001F516", "Fabric Bookmark", "Crafted from sustainable fabric\nscraps with Rewoven tag.", "Donate 1+ items"),
    ]

    y -= 10 * mm
    rw = (W - 2 * mx - 8 * mm) / 2
    rh = 50 * mm

    for i, (icon, title, desc, badge) in enumerate(rewards):
        rx = mx + i * (rw + 8 * mm)
        ry = y - rh

        rr(c, rx, ry, rw, rh, 5 * mm, GREEN_BG)

        # Gold top accent
        c.setFillColor(GOLD)
        c.rect(rx + 5 * mm, ry + rh - 2 * mm, rw - 10 * mm, 2 * mm, fill=1, stroke=0)

        # Icon
        c.setFont("Helvetica", 28)
        c.drawCentredString(rx + rw / 2, ry + rh - 16 * mm, icon)

        # Title
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 12)
        c.drawCentredString(rx + rw / 2, ry + rh - 24 * mm, title)

        # Desc
        c.setFillColor(GRAY)
        c.setFont("Helvetica", 8)
        for j, line in enumerate(desc.split("\n")):
            c.drawCentredString(rx + rw / 2, ry + rh - 32 * mm - j * 3.5 * mm, line)

        # Badge
        btext = badge
        btw = c.stringWidth(btext, "Helvetica-Bold", 8.5) + 10 * mm
        rr(c, rx + (rw - btw) / 2, ry + 4 * mm, btw, 7 * mm, 3.5 * mm, GREEN_DARK)
        c.setFillColor(GREEN)
        c.setFont("Helvetica-Bold", 8.5)
        c.drawCentredString(rx + rw / 2, ry + 6 * mm, btext)

    # ── Philippines destination ──
    y = y - rh - 14 * mm
    dw = W - 2 * mx
    dh = 28 * mm
    rr(c, mx, y - dh, dw, dh, 5 * mm, GREEN_BG)

    c.setFont("Helvetica", 24)
    c.drawString(mx + 10 * mm, y - 16 * mm, "\U0001F1F5\U0001F1ED")

    c.setFillColor(white)
    c.setFont("Helvetica-Bold", 16)
    c.drawString(mx + 28 * mm, y - 10 * mm, "Your Clothes Change Lives")

    c.setFillColor(GRAY)
    c.setFont("Helvetica", 9.5)
    c.drawString(mx + 28 * mm, y - 18 * mm, "Donated to underprivileged children in the Philippines")

    # ── Three facts ──
    y = y - dh - 10 * mm
    facts = [
        ("\U0001F3E0", "Direct\nDistribution"),
        ("\u267B", "Zero\nWaste"),
        ("\U0001F4F7", "Full\nTransparency"),
    ]
    fw = (W - 2 * mx) / 3
    for i, (icon, label) in enumerate(facts):
        fx = mx + fw * i + fw / 2
        c.setFillColor(GREEN)
        c.setFont("Helvetica", 18)
        c.drawCentredString(fx, y, icon)
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 9)
        for j, line in enumerate(label.split("\n")):
            c.drawCentredString(fx, y - 7 * mm - j * 4 * mm, line)

    # ── Drop-off points ──
    y -= 26 * mm
    c.setFillColor(GREEN)
    c.setFont("Helvetica-Bold", 10)
    c.drawCentredString(W / 2, y, "DROP-OFF LOCATIONS")

    y -= 10 * mm
    locs = ["Drop-off Point 1", "Drop-off Point 2", "We also pick up!"]
    for i, loc in enumerate(locs):
        lx = mx + (W - 2 * mx) / 3 * i + (W - 2 * mx) / 6
        c.setFillColor(white)
        c.setFont("Helvetica", 10)
        pin = "\U0001F4CD" if i < 2 else "\U0001F69A"
        c.drawCentredString(lx, y, pin)
        c.setFont("Helvetica-Bold", 9)
        c.drawCentredString(lx, y - 6 * mm, loc)

    # ── CTA ──
    y -= 24 * mm
    cta_w = 180 * mm
    cta_h = 14 * mm
    rr(c, (W - cta_w) / 2, y - cta_h, cta_w, cta_h, 7 * mm, GREEN)
    c.setFillColor(GREEN_DEEPER)
    c.setFont("Helvetica-Bold", 16)
    c.drawCentredString(W / 2, y - cta_h + 4 * mm, "rewovenapp.com/donate")

    # ── Contact ──
    y -= cta_h + 14 * mm
    c.setFillColor(GRAY)
    c.setFont("Helvetica", 10)
    c.drawCentredString(W / 2, y, "hello@rewovenapp.com  |  @rewoven.app")

    # ── Bottom tagline ──
    y -= 14 * mm
    c.setStrokeColor(HexColor("#1A3D2E"))
    c.line(mx + 30 * mm, y, W - mx - 30 * mm, y)
    y -= 10 * mm
    c.setFillColor(HexColor("#64748B"))
    c.setFont("Helvetica", 9)
    c.drawCentredString(W / 2, y, "Every donation matters. Zero waste. Full transparency.")
    y -= 8 * mm
    c.setFillColor(GREEN)
    c.setFont("Helvetica-Bold", 10)
    c.drawCentredString(W / 2, y, "REWOVEN")

    c.save()

    # Convert PDF to PNG at high resolution
    # 1080px wide: W is 270mm = 765pt. To get 1080px: dpi = 1080/765 * 72 = ~101.6
    subprocess.run([
        "pdftoppm", "-png", "-r", "144", "-singlefile",
        TEMP_PDF, os.path.join(DIR, "poster")
    ], check=True)

    # Clean up temp PDF
    os.remove(TEMP_PDF)
    print(f"Poster image saved to: {OUTPUT}")
    # Check dimensions
    result = subprocess.run(["sips", "-g", "pixelWidth", "-g", "pixelHeight", OUTPUT],
                            capture_output=True, text=True)
    print(result.stdout.strip())


if __name__ == "__main__":
    create()
