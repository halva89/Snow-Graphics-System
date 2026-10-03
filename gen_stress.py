#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Генератор стресс-тестовой сцены main.frame (MF).

Запуск:  python gen_stress.py
Результат: assets/stress.mff  (отдельный файл, не трогает main/level/test)

Слишком тяжело? убавь CUBE_ROWS / BALLS и т.п. Слишком легко? увеличь.
"""

import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(SCRIPT_DIR, "assets", "stress.mff")

# --- Настройки нагрузки ---
CUBE_ROWS = 10        # кубовое поле: Rows x Rows
CUBE_SPACING = 2.0
SPHERE_ROWS = 8       # сферное поле
SPHERE_SPACING = 2.0
BALLS = 40            # падающие шары (физика)
PILLARS = 6           # высокие обелиски (стресс теней)
ANIMS = 12            # анимированные кубы
OBJ_ANIMS = 4         # анимированные obj (cube.obj)

PALETTE = [
    (1.0, 0.3, 0.3), (0.3, 1.0, 0.3), (0.3, 0.5, 1.0), (1.0, 0.8, 0.2),
    (1.0, 0.3, 1.0), (0.3, 1.0, 1.0), (0.95, 0.6, 0.3), (0.6, 0.4, 1.0),
    (0.4, 0.85, 0.4), (1.0, 0.6, 0.6),
]


def colr(i):
    r, g, b = PALETTE[i % len(PALETTE)]
    return f"{r:.2f} {g:.2f} {b:.2f}"


lines = []


def L(s=""):
    lines.append(s)


L("# =======================================================")
L("# MF — STRESS TEST SCENE")
L("# Сгенерирована gen_stress.py (запусти снова — изменится нагрузка)")
L(f"# Нагрузка: {CUBE_ROWS}x{CUBE_ROWS} кубов, "
  f"{SPHERE_ROWS}x{SPHERE_ROWS} сфер, {BALLS} физика-шаров, "
  f"{PILLARS} обелисков, {ANIMS} анимаций, {OBJ_ANIMS} obj-анимаций.")
L("# Тени + wireframe + MSAA включены — это стресс.")
L("# =======================================================")
L("")
L("window 1280 720")
L('title "MF - Stress Test"')
L("background 0.12 0.12 0.2")
L("camera true")
L("camera_pos 22 30 26")
L("camera_target 0 1 0")
L("wireframe true")
L("render_mode light")
L("aa-type msaa2")
L("")

L("# ===== Пол (ресивер теней) =====")
L("position 0 -0.15 0")
L("scale 70 0.3 70")
L("cube")
L("1.0")
L("0.78 0.78 0.85")
L("")

L("# ===== КУБОВОЕ ПОЛЕ (без текстур, каждый 4-й с текстурой) =====")
gi = 0
for gx in range(CUBE_ROWS):
    for gz in range(CUBE_ROWS):
        x = -14 + gx * CUBE_SPACING
        z = -12 + gz * CUBE_SPACING
        rot = round(((gi % 5) / 5.0) * 0.5, 3)
        L(f"position {x:.2f} 0.5 {z:.2f}")
        L(f"rotation 0 0 {rot}")
        L("cube")
        L("1.0")
        L(colr(gi))
        if gi % 4 == 0:
            L('texture "assets/rgb.png" expand')
        L("")
        gi += 1

L("# ===== СФЕРНОЕ ПОЛЕ (каждая 3-я с шахматной текстурой) =====")
gi = 0
for gx in range(SPHERE_ROWS):
    for gz in range(SPHERE_ROWS):
        x = -12 + gx * SPHERE_SPACING
        z = 8 + gz * SPHERE_SPACING
        L(f"position {x:.2f} 0.5 {z:.2f}")
        L("sphere")
        L("0.5")
        L("18")
        L(colr(gi + 3))
        if gi % 3 == 0:
            L('texture "assets/checker.png" expand')
        L("")
        gi += 1

L("# ===== ФИЗИКА: падающие шары (разная упругость) =====")
for i in range(BALLS):
    colI = i % 5
    row = i // 5
    x = 18 + colI * 2.0
    z = -14 + row * 2.0
    y = 4 + (i % 5) * 4
    f = round((i % 10) / 10.0, 2)
    L(f"position {x:.2f} {y:.2f} {z:.2f}")
    L(f"physics: m=1, f={f}, p=1")
    L("sphere")
    L("0.4")
    L("14")
    L(colr(i))
    L("")

L("# ===== ФИЗИКА: пирамидка из кубов =====")
PY_LEVELS = 4
PY_X = 14.0
PY_Z = 6.0
for lv in range(PY_LEVELS):
    count = PY_LEVELS - lv
    for p in range(count):
        x = round(PY_X + p * 1.0 + (count - 1) * (-0.5), 2)
        z = PY_Z + lv * 1.0
        y = 0.5 + lv * 1.0
        L(f"position {x:.2f} {y:.2f} {z:.2f}")
        L("physics: m=2, f=0.2, p=2")
        L("cube")
        L("0.9")
        L(colr(p + lv))
        L("")

L("# ===== ТЕНЕВОЙ СТРЕСС: высокие обелиски =====")
for i in range(PILLARS):
    x = -12 + i * 5.0
    L(f"position {x:.2f} 2 22")
    L("scale 0.6 4 0.6")
    L("cube")
    L("1.0")
    L(colr(i + 5))
    L("")

L("# ===== АНИМАЦИОННЫЙ СТРЕСС: летающие кубы =====")
for i in range(ANIMS):
    x = round(2 + i * 1.2, 2)
    z = round(-4 + i * 0.8, 2)
    cA = colr(i)
    cB = colr(i + 4)
    L("animate cube")
    L("keyframe 0.0")
    L(f"pos {x:.2f} 3 {z:.2f}")
    L(f"color {cA}")
    L("keyframe 1.0")
    L(f"pos {x:.2f} 6 {z:.2f}")
    L(f"color {cB}")
    L("keyframe 2.0")
    L(f"pos {x:.2f} 3 {z:.2f}")
    L(f"color {cA}")
    L("}")
    L("")

L("# ===== OBJ-СТРЕСС: анимированные cube.obj =====")
for i in range(OBJ_ANIMS):
    x = -22 + i * 2.5
    z = 14 + i * 2.0
    L('animate mesh "assets/cube.obj"')
    L("keyframe 0.0")
    L(f"pos {x:.2f} 3 {z:.2f}")
    L("color 0.9 0.9 0.9")
    L("keyframe 2.0")
    L(f"pos {x:.2f} 5 {z:.2f}")
    L("color 0.9 0.9 0.9")
    L("}")
    L("")

with open(OUT, "w", encoding="utf-8", newline="\n") as f:
    f.write("\n".join(lines))

print("OK ->", OUT, f"({len(lines)} lines, {sum(len(x) for x in lines)} chars)")