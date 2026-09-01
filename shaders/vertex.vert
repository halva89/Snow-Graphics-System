#version 450

layout(row_major, set = 0, binding = 0) uniform Uniforms {
    mat4 projection;
    mat4 view;
    mat4 model;
} uniforms;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;

layout(location = 0) out vec3 fragColor;

void main() {
    gl_Position = uniforms.projection * uniforms.view * uniforms.model * vec4(inPosition, 1.0);
    fragColor = inColor;
}