#version 450

// Depth-only pass: рендерим сцену из позиции источника света
// прямо в карту теней. На вход уже приходят мировые координаты вершин
// (модельная матрица запечена в буфер на CPU), поэтому используем только light_matrix.
layout(row_major, set = 0, binding = 0) uniform Uniforms {
    mat4 projection;
    mat4 view;
    mat4 model;
    vec4 light_pos;
    vec4 light_color;
    vec4 ambient;
    vec4 view_pos;
    mat4 light_matrix;
    vec4 shadow_params;
} uniforms;

layout(location = 0) in vec3 inPosition;

void main() {
    gl_Position = uniforms.light_matrix * vec4(inPosition, 1.0);
}