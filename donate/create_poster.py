#!/usr/bin/env python3
"""Generate Rewoven Donation Drive poster as PDF (A3 size)."""

from reportlab.lib.pagesizes import A3
from reportlab.lib.units import mm, cm
from reportlab.lib.colors import HexColor, white, black
from reportlab.pdfgen import canvas
from reportlab.lib.enums import TA_CENTER, TA_LEFT
import math
import os

OUTPUT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "poster.pdf")

# Colors
GREEN_PRIMARY = HexColor("#10B981")
GREEN_DARK = HexColor("#064E3B")
GREEN_DEEPER = HexColor("#022C22")
GREEN_LIGHT = HexColor("#D1FAE5")
GREEN_50 = HexColor("#ECFDF5")
GOLD = HexColor("#F59E0B")
HEART = HexColor("#E11D48")
HEART_LIGHT = HexColor("#FFF1F2")
WARM = HexColor("#FEF3C7")
GRAY_600 = HexColor("#4B5563")
GRAY_200 = HexColor("#E5E7EB")
OFF_WHITE = HexColor("#F8FAF5")

W, H = A3  # 297mm x 420mm


def draw_rounded_rect(c, x, y, w, h, r, fill_color=None, stroke_color=None, stroke_width=1):
    """Draw a rounded rectangle."""
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
    if fill_color:
        c.setFillColor(fill_color)
    if stroke_color:
        c.setStrokeColor(stroke_color)
        c.setLineWidth(stroke_width)
    if fill_color and stroke_color:
        c.drawPath(p, fill=1, stroke=1)
    elif fill_color:
        c.drawPath(p, fill=1, stroke=0)
    elif stroke_color:
        c.drawPath(p, fill=0, stroke=1)


def draw_circle(c, cx, cy, r, fill_color):
    c.setFillColor(fill_color)
    c.circle(cx, cy, r, fill=1, stroke=0)


def create_poster():
    c = canvas.Canvas(OUTPUT, pagesize=A3)
    c.setTitle("Rewoven Donation Drive")
    c.setAuthor("Rewoven")

    margin = 20 * mm

    # ─── BACKGROUND ───
    # Main dark green background
    c.setFillColor(GREEN_DEEPER)
    c.rect(0, 0, W, H, fill=1, stroke=0)

    # Decorative circles (subtle)
    c.saveState()
    c.setFillColor(HexColor("#0A3D2E"))
    c.circle(-30 * mm, H - 50 * mm, 120 * mm, fill=1, stroke=0)
    c.circle(W + 20 * mm, 100 * mm, 80 * mm, fill=1, stroke=0)
    c.circle(W / 2, -40 * mm, 100 * mm, fill=1, stroke=0)
    c.restoreState()

    # ─── TOP BADGE ───
    badge_y = H - 42 * mm
    badge_w = 72 * mm
    badge_h = 9 * mm
    badge_x = (W - badge_w) / 2
    draw_rounded_rect(c, badge_x, badge_y, badge_w, badge_h, 4.5 * mm,
                       fill_color=HexColor("#1A0A12"))
    c.setFillColor(HEART)
    c.setFont("Helvetica-Bold", 10)
    c.drawCentredString(W / 2, badge_y + 2.8 * mm, "\u2764  DONATION DRIVE NOW OPEN")

    # ─── MAIN TITLE ───
    title_y = H - 72 * mm
    c.setFillColor(white)
    c.setFont("Helvetica-Bold", 18)
    c.drawCentredString(W / 2, title_y + 22 * mm, "REWOVEN")

    c.setFont("Helvetica-Bold", 40)
    c.setFillColor(white)
    c.drawCentredString(W / 2, title_y, "Give Clothes a")

    c.setFont("Helvetica-Bold", 40)
    c.setFillColor(GREEN_PRIMARY)
    c.drawCentredString(W / 2, title_y - 14 * mm, "Second Life")

    c.setFont("Helvetica-Bold", 30)
    c.setFillColor(white)
    c.drawCentredString(W / 2, title_y - 30 * mm, "Give Kids a")

    c.setFillColor(HEART)
    c.setFont("Helvetica-Bold", 30)
    c.drawCentredString(W / 2, title_y - 44 * mm, "Better One")

    # ─── SUBTITLE ───
    sub_y = title_y - 62 * mm
    c.setFillColor(HexColor("#94A3B8"))
    c.setFont("Helvetica", 12)
    c.drawCentredString(W / 2, sub_y, "Donate your pre-loved clothes to underprivileged children")
    c.drawCentredString(W / 2, sub_y - 5 * mm, "in the Philippines. Receive a handmade reward in return.")

    # ─── DIVIDER LINE ───
    div_y = sub_y - 14 * mm
    c.setStrokeColor(HexColor("#1A3D2E"))
    c.setLineWidth(1)
    c.line(margin + 20 * mm, div_y, W - margin - 20 * mm, div_y)

    # ─── STATS BAR ───
    stats_y = div_y - 22 * mm
    stats = [
        ("92M", "tons textile waste yearly"),
        ("31M", "Filipino children in poverty"),
        ("95%", "of clothes could be reused"),
    ]
    stat_w = (W - 2 * margin) / 3
    for i, (num, label) in enumerate(stats):
        sx = margin + stat_w * i + stat_w / 2
        c.setFillColor(GREEN_PRIMARY)
        c.setFont("Helvetica-Bold", 28)
        c.drawCentredString(sx, stats_y, num)
        c.setFillColor(HexColor("#94A3B8"))
        c.setFont("Helvetica", 9)
        c.drawCentredString(sx, stats_y - 6 * mm, label)

    # ─── HOW IT WORKS ───
    how_y = stats_y - 26 * mm
    c.setFillColor(GREEN_PRIMARY)
    c.setFont("Helvetica-Bold", 8)
    c.drawCentredString(W / 2, how_y, "HOW IT WORKS")

    steps = [
        ("\U0001F6D2", "Gather\nClothes", "Collect gently used\nclothing you no longer wear"),
        ("\U0001F4E6", "Drop Off\nor Ship", "Bring to a drop-off\npoint or arrange pickup"),
        ("\u2708", "We Ship to\nPhilippines", "Sorted, packed &\nsent to kids in need"),
        ("\U0001F381", "Get Your\nReward", "Receive a handmade\nhandbag or bookmark"),
    ]

    step_w = (W - 2 * margin - 12 * mm) / 4
    step_h = 48 * mm
    step_y = how_y - 10 * mm - step_h

    for i, (icon, title, desc) in enumerate(steps):
        sx = margin + 3 * mm + step_w * i + 3 * mm * i
        # Card background
        draw_rounded_rect(c, sx, step_y, step_w, step_h, 4 * mm,
                           fill_color=HexColor("#0D3B2B"))

        # Step number circle
        draw_circle(c, sx + step_w / 2, step_y + step_h - 8 * mm, 5 * mm, GREEN_PRIMARY)
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 12)
        c.drawCentredString(sx + step_w / 2, step_y + step_h - 10 * mm, str(i + 1))

        # Title
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 10)
        lines = title.split("\n")
        for j, line in enumerate(lines):
            c.drawCentredString(sx + step_w / 2, step_y + step_h - 20 * mm - j * 4 * mm, line)

        # Description
        c.setFillColor(HexColor("#94A3B8"))
        c.setFont("Helvetica", 7)
        desc_lines = desc.split("\n")
        for j, line in enumerate(desc_lines):
            c.drawCentredString(sx + step_w / 2, step_y + step_h - 32 * mm - j * 3.5 * mm, line)

        # Arrow between steps
        if i < 3:
            arrow_x = sx + step_w + 1.5 * mm
            c.setFillColor(GREEN_PRIMARY)
            c.setFont("Helvetica-Bold", 14)
            c.drawCentredString(arrow_x, step_y + step_h / 2, "\u2192")

    # ─── REWARDS SECTION ───
    reward_y = step_y - 18 * mm
    c.setFillColor(GREEN_PRIMARY)
    c.setFont("Helvetica-Bold", 8)
    c.drawCentredString(W / 2, reward_y, "YOUR REWARD")

    # Two reward cards
    card_w = (W - 2 * margin - 8 * mm) / 2
    card_h = 42 * mm
    card_y = reward_y - 8 * mm - card_h

    rewards = [
        ("\U0001F45C", "Sustainable Handbag", "A unique, one-of-a-kind tote bag\nhandmade from upcycled fabric\noffcuts. No two bags are alike.", "Donate 5+ items"),
        ("\U0001F516", "Fabric Bookmark", "Beautifully crafted from sustainable\nfabric scraps with a hand-stitched\nRewoven tag.", "Donate 1+ items"),
    ]

    for i, (icon, title, desc, badge) in enumerate(rewards):
        rx = margin + i * (card_w + 8 * mm)

        # Card
        draw_rounded_rect(c, rx, card_y, card_w, card_h, 4 * mm,
                           fill_color=HexColor("#0D3B2B"))
        # Green top accent
        c.setFillColor(GREEN_PRIMARY)
        c.rect(rx + 4 * mm, card_y + card_h - 1.5 * mm, card_w - 8 * mm, 1.5 * mm, fill=1, stroke=0)

        # Icon
        c.setFont("Helvetica", 24)
        c.drawString(rx + 6 * mm, card_y + card_h - 14 * mm, icon)

        # Title
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 13)
        c.drawString(rx + 20 * mm, card_y + card_h - 12 * mm, title)

        # Description
        c.setFillColor(HexColor("#94A3B8"))
        c.setFont("Helvetica", 7.5)
        for j, line in enumerate(desc.split("\n")):
            c.drawString(rx + 6 * mm, card_y + card_h - 22 * mm - j * 3.5 * mm, line)

        # Badge
        badge_bx = rx + 6 * mm
        badge_by = card_y + 4 * mm
        badge_bw = c.stringWidth(badge, "Helvetica-Bold", 8) + 10 * mm
        draw_rounded_rect(c, badge_bx, badge_by, badge_bw, 6 * mm, 3 * mm,
                           fill_color=HexColor("#064E3B"))
        c.setFillColor(GREEN_PRIMARY)
        c.setFont("Helvetica-Bold", 8)
        c.drawString(badge_bx + 5 * mm, badge_by + 1.8 * mm, badge)

    # ─── DESTINATION ───
    dest_y = card_y - 16 * mm
    # Philippine flag and text
    dest_card_w = W - 2 * margin
    dest_card_h = 26 * mm
    draw_rounded_rect(c, margin, dest_y - dest_card_h + 8 * mm, dest_card_w, dest_card_h, 4 * mm,
                       fill_color=HexColor("#0D3B2B"))

    c.setFont("Helvetica", 20)
    c.drawString(margin + 8 * mm, dest_y - 8 * mm, "\U0001F1F5\U0001F1ED")

    c.setFillColor(white)
    c.setFont("Helvetica-Bold", 14)
    c.drawString(margin + 24 * mm, dest_y - 5 * mm, "Your Clothes Change Lives")

    c.setFillColor(HexColor("#94A3B8"))
    c.setFont("Helvetica", 8.5)
    c.drawString(margin + 24 * mm, dest_y - 12 * mm, "Donated directly to underprivileged children & families in the Philippines")

    # Three fact icons
    facts_y = dest_y - dest_card_h - 4 * mm
    fact_items = [
        ("\U0001F3E0", "Direct Distribution", "Partnered with local NGOs"),
        ("\u267B", "Zero Waste", "Nothing goes to landfill"),
        ("\U0001F4F7", "Full Transparency", "Photos & updates shared"),
    ]
    fact_w = (W - 2 * margin) / 3
    for i, (icon, title, desc) in enumerate(fact_items):
        fx = margin + fact_w * i + fact_w / 2
        c.setFont("Helvetica", 14)
        c.setFillColor(GREEN_PRIMARY)
        c.drawCentredString(fx, facts_y, icon)
        c.setFillColor(white)
        c.setFont("Helvetica-Bold", 8)
        c.drawCentredString(fx, facts_y - 5 * mm, title)
        c.setFillColor(HexColor("#94A3B8"))
        c.setFont("Helvetica", 7)
        c.drawCentredString(fx, facts_y - 10 * mm, desc)

    # ─── QR CODE AREA + CONTACT ───
    bottom_y = facts_y - 28 * mm

    # QR placeholder
    qr_size = 22 * mm
    qr_x = margin + 8 * mm
    qr_y = bottom_y - 4 * mm
    draw_rounded_rect(c, qr_x, qr_y, qr_size, qr_size, 3 * mm,
                       fill_color=white)
    c.setFillColor(GRAY_600)
    c.setFont("Helvetica-Bold", 7)
    c.drawCentredString(qr_x + qr_size / 2, qr_y + qr_size / 2 + 2 * mm, "QR CODE")
    c.setFont("Helvetica", 6)
    c.drawCentredString(qr_x + qr_size / 2, qr_y + qr_size / 2 - 2 * mm, "Scan to")
    c.drawCentredString(qr_x + qr_size / 2, qr_y + qr_size / 2 - 5 * mm, "Register")

    c.setFillColor(white)
    c.setFont("Helvetica-Bold", 7)
    c.drawString(qr_x + qr_size + 5 * mm, bottom_y + 12 * mm, "Scan to Donate")
    c.setFillColor(HexColor("#94A3B8"))
    c.setFont("Helvetica", 7)
    c.drawString(qr_x + qr_size + 5 * mm, bottom_y + 7 * mm, "rewovenapp.com/donate")

    # Contact info - right side
    contact_x = W - margin - 8 * mm
    c.setFillColor(white)
    c.setFont("Helvetica-Bold", 8)
    c.drawRightString(contact_x, bottom_y + 12 * mm, "hello@rewovenapp.com")
    c.setFont("Helvetica", 8)
    c.setFillColor(HexColor("#94A3B8"))
    c.drawRightString(contact_x, bottom_y + 6 * mm, "@rewoven.app")
    c.drawRightString(contact_x, bottom_y, "rewovenapp.com")

    # ─── BOTTOM TAGLINE ───
    c.setFillColor(HexColor("#1A3D2E"))
    c.setLineWidth(0.5)
    c.line(margin, bottom_y - 10 * mm, W - margin, bottom_y - 10 * mm)

    c.setFillColor(HexColor("#64748B"))
    c.setFont("Helvetica", 8)
    c.drawCentredString(W / 2, bottom_y - 16 * mm, "Every donation matters. Zero waste. Full transparency.")

    c.setFillColor(GREEN_PRIMARY)
    c.setFont("Helvetica-Bold", 9)
    c.drawCentredString(W / 2, bottom_y - 23 * mm, "REWOVEN  \u00B7  Turning Fashion Waste Into Action")

    c.save()
    print(f"Poster saved to: {OUTPUT}")


if __name__ == "__main__":
    create_poster()
