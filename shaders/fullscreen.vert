#version 450
// Полноэкранный треугольник для пост-пасса. Вершины приходят из маленького
// VB (inPos), UV считаем из позиции — без gl_VertexIndex (совместимее).
layout(location = 0) in vec2 inPos;
layout(location = 0) out vec2 vUv;

void main() {
    vUv = inPos * 0.5 + 0.5;
    gl_Position = vec4(inPos, 0.0, 1.0);
}