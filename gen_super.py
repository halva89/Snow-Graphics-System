#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Супер-стресс: куча сфер по 64 сегмента, без wireframe, все анимируются
(движение вверх-вниз волной + смена цвета). Результат: assets/super.mff
"""
import os, math

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(SCRIPT_DIR, "assets", "super.mff")

# Параметры
GRID = 12          # сетка GRID x GRID анимированных сфер
SEGS = 64          # сегменты сферы (детализация)
SPACING = 1.6      # шаг сетки
R = 0.5            # радиус сферы
AMPL = 1.0         # амплитуда вертикального движения
CYCLES = 3.0       # сколько циклов волны за длительность
DUR = 6.0
FPS = 24

def color_lerp_hex(t):
    # переход по палитре: синий -> зелёный -> жёлтый -> красный
    stops = [(0.2, 0.6, 1.0), (0.2, 1.0, 0.6), (1.0, 1.0, 0.2), (1.0, 0.3, 0.2)]
    seg = min(int(t * (len(stops) - 1)), len(stops) - 2)
    f = t * (len(stops) - 1) - seg
    a, b = stops[seg], stops[seg + 1]
    return (a[0] + (b[0] - a[0]) * f, a[1] + (b[1] - a[1]) * f, a[2] + (b[2] - a[2]) * f)

lines = []
L = lambda s="": lines.append(s)
L("# main.frame - SUPER TEST: %d x %d spheres, %d segments, animated, no wireframe" % (GRID, GRID, SEGS))
L("window 1280 720")
L('title "main.frame - Super Test"')
L("background 0.07 0.08 0.12")
L("camera true")
L("camera_pos 18 22 30")
L("camera_target 0 0 0")
L("wireframe false")
L("render_mode light")
L("aa-type none")
L("")

# Пол (статичный)
L("# floor")
L("position 0 -0.7 0")
L("scale 26 0.3 26")
L("cube")
L("0.32 0.34 0.4")
L("")

COUNT = GRID * GRID
half = (GRID - 1) * 0.5

# Чтобы анимация была в приемлемых размерах файла, используем ограниченное
# число keyframe (FPS кадров на длительность) с линейной интерполяцией.
def gen_keyframes(px, pz, phase):
    """Вертикальная волна: y = AMPL * sin(2pi * (t/T - x/space)) + base"""
    frames = []
    for f in range(int(FPS * DUR)):
        t = f / FPS
        y = AMPL * math.sin(2 * math.pi * (t / DUR - phase))
        col = color_lerp_hex(((y + AMPL) / (2 * AMPL)))
        frames.append((t, y, col))
    return frames

for idx in range(COUNT):
    gx = idx % GRID
    gz = idx // GRID
    px = (gx - half) * SPACING
    pz = (gz - half) * SPACING
    phase = (gz * 0.7 + gx * 0.3) / max(GRID, 1)
    L("# sphere %d" % idx)
    L("animate sphere %d" % SEGS)
    for (t, y, c) in gen_keyframes(px, pz, phase):
        L("keyframe %.3f" % t)
        L("pos %.3f %.3f %.3f" % (px, y, pz))
        L("color %.3f %.3f %.3f" % c)
    L("")
    L("")

with open(OUT, "w", encoding="utf-8", newline="\n") as f:
    f.write("\n".join(lines))

print("OK ->", OUT)
print("spheres:", COUNT, "| segments:", SEGS, "| keyframes per sphere:", FPS * DUR, "| total lines:", len(lines))