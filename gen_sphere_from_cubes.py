import math

def hex_to_rgb(hex_code):
    hex_code = hex_code.lstrip('#')
    if len(hex_code) == 3:
        hex_code = ''.join([c * 2 for c in hex_code])
    r = int(hex_code[0:2], 16) / 255.0
    g = int(hex_code[2:4], 16) / 255.0
    b = int(hex_code[4:6], 16) / 255.0
    return r, g, b

def generate_sphere_dyson(output_file="../Snow-Graphics-System/assets/level1.mff"):
    """Сфера из кубов, вращающаяся вокруг Y, затем схлопывается в центр."""
    lines = []
    lines.append("# main.frame - Dyson Sphere (Spiral Collapse)")
    lines.append("window 1024 768")
    lines.append('title "Dyson Sphere - 4096 Cubes"')
    lines.append("render_mode heavy")
    lines.append("camera true")
    lines.append("")

    radius = 4.0
    num_cubes = 16498  # 16x16x16 = 4096 (или другое точное число)
    
    # Генерируем координаты на сфере (равномерное распределение Фибоначчи)
    cubes = []
    golden_ratio = (1 + math.sqrt(5)) / 2
    
    for i in range(num_cubes):
        # Равномерное распределение точек на сфере
        theta = 2 * math.pi * i / golden_ratio
        phi = math.acos(1 - 2 * (i + 0.5) / num_cubes)
        
        x = radius * math.sin(phi) * math.cos(theta)
        y = radius * math.sin(phi) * math.sin(theta)
        z = radius * math.cos(phi)
        
        # Цвет: градиент по позиции
        r = (x + radius) / (2 * radius)
        g = (y + radius) / (2 * radius)
        b = (z + radius) / (2 * radius)
        
        cubes.append((x, y, z, r, g, b))
    
    for idx, (x, y, z, r, g, b) in enumerate(cubes):
        # Анимация: каждые 2 секунды куб смещается на 0.2 по касательной (вращение)
        # Проще: кубы будут следовать друг за другом к центру
        delay = (idx % 64) * 0.02  # волна
        
        kf1_time = delay
        kf2_time = delay + 3.0
        kf3_time = delay + 6.0
        
        lines.append(f"# Cube {idx} at ({x:.2f}, {y:.2f}, {z:.2f})")
        lines.append(f"position {x:.2f} {y:.2f} {z:.2f}")
        lines.append("animate cube")
        lines.append(f"    keyframe {kf1_time:.2f}")
        lines.append(f"        pos {x:.2f} {y:.2f} {z:.2f}")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append(f"    keyframe {kf2_time:.2f}")
        lines.append(f"        pos 0.0 0.0 5.0")  # Центр
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append(f"    keyframe {kf3_time:.2f}")
        lines.append(f"        pos {x:.2f} {y:.2f} {z:.2f}")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append("}")
        lines.append("")
    
    with open(output_file, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    
    print(f"Сгенерирован файл: {output_file}")
    print(f"Количество кубов: {num_cubes} (сфера)")

generate_sphere_dyson()