import sys

def generate_grid():
    rows = 100
    cols = 100
    radius = 12
    segments = 6
    
    print("window 1920 1080")
    print('title "Snow Graphics System - 16384 Circles"')
    print()
    
    for row in range(rows):
        y = -0.95 + (row / (rows - 1)) * 1.9
        for col in range(cols):
            x = -0.95 + (col / (cols - 1)) * 1.9
            
            # Цвет от координат
            r = (x + 1.0) / 2.0
            g = (y + 1.0) / 2.0
            b = 0.8
            
            print(f"circle")
            print(f"{x:.4f} {y:.4f} {radius} {segments}")
            print(f"{r:.2f} {g:.2f} {b:.2f}")
            print()

if __name__ == "__main__":
    sys.stdout = open("assets/level1.sgss", "w")
    generate_grid()