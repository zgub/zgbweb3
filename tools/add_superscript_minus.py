"""Add U+207B SUPERSCRIPT MINUS to a Nunito build that lacks it.

Built as a composite of the normal `minus`, scaled and raised so it sits at the
same size and height as Nunito's own superscript digits (measured from
SUPERSCRIPT FOUR vs FOUR). Weight variation comes along for free, because the
component is the real (variable) minus glyph.

usage: python3 -I add_superscript_minus.py in.ttf out.ttf
"""
import sys

from fontTools.ttLib import TTFont
from fontTools.ttLib.tables._g_l_y_f import Glyph, GlyphComponent

src, dst = sys.argv[1], sys.argv[2]
font = TTFont(src)
# Decompile every table now: some (gvar) check the glyph count when loaded lazily.
for tag in font.keys():
    font[tag]
glyf, hmtx, cmap = font["glyf"], font["hmtx"], font.getBestCmap()
if 0x207B in cmap:
    font.save(dst)
    sys.exit(0)


def bounds(name):
    g = glyf[name]
    g.recalcBounds(glyf)
    return g.xMin, g.yMin, g.xMax, g.yMax


_, f_y0, _, f_y1 = bounds("four")
_, s_y0, _, s_y1 = bounds(cmap[0x2074])
m_x0, m_y0, m_x1, m_y1 = bounds("minus")

scale = (s_y1 - s_y0) / (f_y1 - f_y0)
# Centre the scaled minus on the superscript digits' vertical centre.
dy = (s_y0 + s_y1) / 2 - scale * (m_y0 + m_y1) / 2
advance = round(hmtx[cmap[0x2074]][0] * 1.0)
dx = (advance - scale * (m_x1 - m_x0)) / 2 - scale * m_x0

comp = GlyphComponent()
comp.glyphName = "minus"
comp.x, comp.y = round(dx), round(dy)
comp.transform = [[round(scale, 4), 0], [0, round(scale, 4)]]
comp.flags = 0x0004  # ROUND_XY_TO_GRID; scale flags are set on compile
g = Glyph()
g.numberOfContours = -1
g.components = [comp]

name = "uni207B"
font.setGlyphOrder(font.getGlyphOrder() + [name])
glyf.glyphs[name] = g
glyf.glyphOrder = font.getGlyphOrder()
hmtx[name] = (advance, round(dx + scale * m_x0))
if "gvar" in font:
    font["gvar"].variations[name] = []
if "HVAR" in font:
    # The new glyph has no HVAR entry; dropping HVAR makes advances come from gvar.
    del font["HVAR"]
for table in font["cmap"].tables:
    if table.isUnicode():
        table.cmap[0x207B] = name
font.save(dst)
print(f"added {name}: scale {scale:.3f}, offset ({comp.x}, {comp.y}), advance {advance}")
