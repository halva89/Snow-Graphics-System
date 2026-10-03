#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Генератор сцены: бордовая рамка, появляющаяся из левого верхнего угла
вдоль края поля зрения (fov). Сектор квадратов-сегментов по периметру
включается последовательно. Результат: assets/frame.mff
"""
import os

SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(SCRIPT_DIR, "assets", "frame.mff")

BG = (0.04, 0.01, 0.02)      # фон / скрытое состояние
BORD = (0.55, 0.03, 0.06)    # бордовый

# Периметр рамки (мир. XY, камера смотрит на центр).
X0, X1 = -8.0, 8.0
Y0, Y1 = -5.0, 5.0
STEP = 1.0                   # шаг/размер сегмента (animate square: сторона 1.0)

def points_on_edge(ax, ay, bx, by):
    """Точки вдоль ребра от A к B через каждые STEP, включая начальную."""
    length = ((bx - ax) ** 2 + (by - ay) ** 2) ** 0.5
    n = int(round(length / STEP))
    pts = []
    for i in range(n):
        t = i / n
        pts.append((ax + (bx - ax) * t, ay + (by - ay) * t))
    return pts

# Обход периметра от левого верхнего угла по часовой стрелке.
path = []
path += points_on_edge(X0, Y1, X1, Y1)   # верх: слева -> направо
path += points_on_edge(X1, Y1, X1, Y0)   # право: сверху -> вниз
path += points_on_edge(X1, Y0, X0, Y0)   # низ: справа -> налево
path += points_on_edge(X0, Y0, X0, Y1)   # лево: снизу -> вверх

SPACING = 0.085
DUR = 7.0

lines = []
L = lambda s="": lines.append(s)
L("# main.frame - появление бордовой рамки вдоль края fov")
L("# Рамка рисуется из левого верхнего угла по часовой стрелке и петляет.")
L("window 900 600")
L('title "main.frame - Frame Draw"')
L("background 0.04 0.01 0.02")
L("camera true")
L("camera_pos 0 0.5 16")
L("camera_target 0 0 0")
L("aa-type none")
L("")

for i, (px, py) in enumerate(path):
    t0 = i * SPACING
    L("# segment %d" % i)
    L("animate square")
    L("keyframe 0.0")
    L("pos %.3f %.3f 0" % (px, py))
    L("color %.2f %.2f %.2f" % BG)
    L("keyframe %.3f" % t0)
    L("pos %.3f %.3f 0" % (px, py))
    L("color %.2f %.2f %.2f" % BG)
    L("keyframe %.3f" % (t0 + 0.012))
    L("pos %.3f %.3f 0" % (px, py))
    L("color %.2f %.2f %.2f" % BORD)
    L("keyframe %.3f" % DUR)
    L("pos %.3f %.3f 0" % (px, py))
    L("color %.2f %.2f %.2f" % BORD)
    L("")
    L("")

with open(OUT, "w", encoding="utf-8", newline="\n") as f:
    f.write("\n".join(lines))

print("OK ->", OUT)
print("segments:", len(path))