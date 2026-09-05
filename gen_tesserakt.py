import math

def hex_to_rgb(hex_code):
    hex_code = hex_code.lstrip('#')
    if len(hex_code) == 3:
        hex_code = ''.join([c * 2 for c in hex_code])
    r = int(hex_code[0:2], 16) / 255.0
    g = int(hex_code[2:4], 16) / 255.0
    b = int(hex_code[4:6], 16) / 255.0
    return r, g, b

def generate_tesseract(output_file="../Snow-Graphics-System/assets/level1.sgss"):
    """Тессеракт (4D-гиперкуб), вращающийся в 4D и проецируемый на 3D."""
    lines = []
    lines.append("# Snow Graphics System - Tesseract 4D Projection")
    lines.append("window 1024 768")
    lines.append('title "Tesseract - 4D Hypercube"')
    lines.append("render_mode heavy")
    lines.append("camera true")
    lines.append("")

    # 16 вершин тессеракта в 4D координатах
    vertices_4d = []
    for i in range(16):
        x = 1 if (i & 1) else -1
        y = 1 if (i & 2) else -1
        z = 1 if (i & 4) else -1
        w = 1 if (i & 8) else -1
        vertices_4d.append((x, y, z, w))
    
    # Ребра тессеракта (соединяем вершины, отличающиеся на 1 бит)
    edges = []
    for i in range(16):
        for j in range(i+1, 16):
            xor = i ^ j
            if xor & (xor - 1) == 0:  # степень двойки -> соседние по 1 биту
                edges.append((i, j))
    
    # Проекция 4D -> 3D (средняя точка между ребрами)
    # Мы создадим куб на каждом ребре (в точке середины ребра)
    cubes = []
    for edge_idx, (v1_idx, v2_idx) in enumerate(edges):
        v1 = vertices_4d[v1_idx]
        v2 = vertices_4d[v2_idx]
        
        # 4D середина
        mid_x = (v1[0] + v2[0]) / 2.0
        mid_y = (v1[1] + v2[1]) / 2.0
        mid_z = (v1[2] + v2[2]) / 2.0
        mid_w = (v1[3] + v2[3]) / 2.0
        
        # 4D -> 3D проекция (делим на (1 + w))
        proj_factor = 1.0 / (1.0 + mid_w * 0.5)
        proj_x = mid_x * proj_factor * 4.0
        proj_y = mid_y * proj_factor * 4.0
        proj_z = mid_z * proj_factor * 4.0
        
        # Цвет: по номеру ребра (радуга)
        hue = edge_idx / len(edges)
        r = 0.5 + 0.5 * math.sin(hue * 2 * math.pi)
        g = 0.5 + 0.5 * math.sin(hue * 2 * math.pi + 2.094)
        b = 0.5 + 0.5 * math.sin(hue * 2 * math.pi + 4.189)
        
        cubes.append((proj_x, proj_y, proj_z, r, g, b, edge_idx))
    
    for proj_x, proj_y, proj_z, r, g, b, idx in cubes:
        delay = (idx % 32) * 0.02
        
        kf1_time = delay
        kf2_time = delay + 3.0
        kf3_time = delay + 6.0
        
        lines.append(f"# Edge {idx} projected at ({proj_x:.2f}, {proj_y:.2f}, {proj_z:.2f})")
        lines.append(f"position {proj_x:.2f} {proj_y:.2f} {proj_z:.2f}")
        lines.append("animate cube")
        lines.append(f"    keyframe {kf1_time:.2f}")
        lines.append(f"        pos {proj_x:.2f} {proj_y:.2f} {proj_z:.2f}")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append(f"    keyframe {kf2_time:.2f}")
        lines.append(f"        pos 0.0 0.0 5.0")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append(f"    keyframe {kf3_time:.2f}")
        lines.append(f"        pos {proj_x:.2f} {proj_y:.2f} {proj_z:.2f}")
        lines.append(f"        color {r:.2f} {g:.2f} {b:.2f}")
        
        lines.append("}")
        lines.append("")
    
    with open(output_file, "w", encoding="utf-8") as f:
        f.write("\n".join(lines))
    
    print(f"Сгенерирован файл: {output_file}")
    print(f"Количество кубов: {len(cubes)} (рёбра тессеракта)")

generate_tesseract()