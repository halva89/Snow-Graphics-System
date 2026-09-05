import math

def hex_to_rgb(hex_code):
    hex_code = hex_code.lstrip('#')
    if len(hex_code) == 3:
        hex_code = ''.join([c * 2 for c in hex_code])
    r = int(hex_code[0:2], 16) / 255.0
    g = int(hex_code[2:4], 16) / 255.0
    b = int(hex_code[4:6], 16) / 255.0
    return r, g, b

def mandelbrot_3d(x, y, z, max_iter=20):
    """Быстрая проверка: принадлежит ли точка 3D-множеству Мандельброта."""
    cx, cy, cz = x, y, z
    for i in range(max_iter):
        # 3D-итерация (упрощенная формула для визуального фрактала)
        x, y, z = x*x - y*y - z*z + cx, 2*x*y + cy, 2*x*z + cz
        if x*x + y*y + z*z > 4:
            return False, i
    return True, max_iter

def generate_mandelbrot(output_file="../Snow-Graphics-System/assets/level1.sgss"):
    """3D-фрактал Мандельброта из кубов."""
    lines = []
    lines.append("# Snow Graphics System - 3D Mandelbrot Fractal")
    lines.append("window 1024 768")
    lines.append('title "3D Mandelbrot Fractal"')
    lines.append("render_mode heavy")
    lines.append("camera true")
    lines.append("")

    grid_size = 24  # 24x24x24 = 13824 потенциальных кубов (но мы возьмем только те, что внутри)
    scale = 3.0
    cubes = []
    
    for i in range(grid_size):
        for j in range(grid_size):
            for k in range(grid_size):
                # Преобразуем индексы в координаты [-scale, scale]
                x = (i - grid_size/2) * (2*scale / grid_size)
                y = (j - grid_size/2) * (2*scale / grid_size)
                z = (k - grid_size/2) * (2*scale / grid_size)
                
                # Проверяем принадлежность фракталу
                is_inside, iter_count = mandelbrot_3d(x, y, z)
                
                if is_inside:
                    # Цвет: затенение по количеству итераций (более яркие точки глубже)
                    t = iter_count / 20.0
                    r = t
                    g = 1.0 - t
                    b = (1.0 - t) * 0.5
                    
                    cubes.append((x, y, z, r, g, b, i, j, k))
    
    for x, y, z, r, g, b, i, j, k in cubes:
        # Задержка зависит от "глубины" в фрактале (чем ближе к центру, тем позже)
        delay = (math.sqrt(x*x + y*y + z*z) / scale) * 2.0
        
        kf1_time = delay
        kf2_time = delay + 3.0
        kf3_time = delay + 6.0
        
        lines.append(f"# Fractal point ({x:.2f}, {y:.2f}, {z:.2f})")
        lines.append(f"position {x:.2f} {y:.2f} {z:.2f}")
        lines.append("animate cube")
        lines.append(f"    keyframe {kf1_time:.2f}")
        lines.append(f"        pos {x:.2f} {y:.2f} {z:.2f}")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append(f"    keyframe {kf2_time:.2f}")
        lines.append(f"        pos 0.0 0.0 5.0")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append(f"    keyframe {kf3_time:.2f}")
        lines.append(f"        pos {x:.2f} {y:.2f} {z:.2f}")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append("}")
        lines.append("")
    
    with open(output_file, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    
    print(f"Сгенерирован файл: {output_file}")
    print(f"Количество кубов: {len(cubes)} (внутри фрактала)")

generate_mandelbrot()