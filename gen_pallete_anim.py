import os

def hex_to_rgb(hex_code):
    """Преобразует HEX (например, 'FF0') в кортеж (r, g, b) с значениями 0.0-1.0"""
    hex_code = hex_code.lstrip('#')
    if len(hex_code) == 3:
        hex_code = ''.join([c * 2 for c in hex_code])
    r = int(hex_code[0:2], 16) / 255.0
    g = int(hex_code[2:4], 16) / 255.0
    b = int(hex_code[4:6], 16) / 255.0
    return r, g, b

def generate_sgss(rows, cols, output_file="hex_palette_spiral.sgss"):
    """
    Генерирует .sgss файл с сеткой кубов, которые двигаются друг за другом
    к центру (0,0,5) и обратно, создавая эффект вращения палитры.
    """
    lines = []
    lines.append("# Snow Graphics System - HEX Palette Spiral Animation")
    lines.append("window 800 600")
    lines.append('title "HEX Palette Spiral"')
    lines.append("render_mode heavy")
    lines.append("camera true")
    lines.append("")
    
    # Смещение сетки от центра
    x_offset = (cols - 1) / 2.0
    y_offset = (rows - 1) / 2.0
    
    # Список кубов (координаты и цвет), чтобы потом отсортировать по удаленности от центра
    cubes = []
    
    for i in range(rows):
        for j in range(cols):
            x = i - x_offset
            y = j - y_offset
            
            # Генерируем HEX цвет (градиент по координатам)
            r_int = int((i / max(rows - 1, 1)) * 255)
            g_int = int((j / max(cols - 1, 1)) * 255)
            b_int = 0
            
            hex_code = f"{r_int:02X}{g_int:02X}{b_int:02X}"
            r, g, b = hex_to_rgb(hex_code)
            
            # Вычисляем расстояние от центра (для задержки)
            dist = (x**2 + y**2) ** 0.5
            
            cubes.append({
                'x': x,
                'y': y,
                'r': r,
                'g': g,
                'b': b,
                'dist': dist,
                'hex': hex_code
            })
    
    # Сортируем кубы по расстоянию от центра (ближние начнут первыми)
    cubes.sort(key=lambda c: c['dist'])
    
    # Параметры анимации
    base_delay = 0.0  # Задержка самого близкого куба
    delay_step = 0.1  # Задержка на каждый шаг (чем дальше куб, тем позже он начнет)
    
    for idx, cube in enumerate(cubes):
        x = cube['x']
        y = cube['y']
        r = cube['r']
        g = cube['g']
        b = cube['b']
        delay = base_delay + cube['dist'] * delay_step
        
        # Вычисляем время keyframe'ов с учетом задержки
        # Анимация: 3 секунды в центр, 3 секунды обратно
        kf1_time = delay           # Начало движения (или стартовая позиция)
        kf2_time = delay + 3.0     # Прилет в центр
        kf3_time = delay + 6.0     # Вернулся обратно
        
        # Итоговое время (чтобы сетка не "зависла", если delay больше 0)
        # Но так как delay маленький, всё должно быть ок
        
        lines.append(f"# Cube at ({x}, {y}) with color #{cube['hex']}, delay={delay:.2f}s")
        lines.append(f"position {x} {y} 0.0")
        lines.append("animate cube")
        
        # Keyframe 1: Начало в своей точке (остается здесь до своей задержки)
        lines.append(f"    keyframe {kf1_time:.2f}")
        lines.append(f"        pos {x} {y} 0.0")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        # Keyframe 2: Прилетел в центр (0,0,5)
        lines.append(f"    keyframe {kf2_time:.2f}")
        lines.append(f"        pos 0.0 0.0 5.0")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        # Keyframe 3: Вернулся обратно в свою точку (но Z=0)
        lines.append(f"    keyframe {kf3_time:.2f}")
        lines.append(f"        pos {x} {y} 0.0")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append("}")
        lines.append("")
    
    with open(output_file, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    
    print(f"Сгенерирован файл: {output_file}")
    print(f"Количество кубов: {rows * cols}")
    print("Эффект: кубы двигаются друг за другом к центру и обратно (спираль/волна)")

if __name__ == "__main__":
    rows = 64
    cols = 64
    generate_sgss(rows, cols, "assets/level1.sgss")