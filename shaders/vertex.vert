#version 450

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

layout(set = 0, binding = 1) uniform sampler2D texSampler;

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec3 inColor;
layout(location = 2) in vec2 inTexCoord;
layout(location = 3) in vec3 inNormal;

layout(location = 0) out vec3 fragColor;
layout(location = 1) out vec2 fragTexCoord;
layout(location = 2) out vec3 fragWorldPos;
layout(location = 3) out vec3 fragNormal;
layout(location = 4) out vec4 fragLightPos;

void main() {
    vec4 worldPos = uniforms.model * vec4(inPosition, 1.0);
    // Пасс теней (shadow_params.w > 0.5) проецирует в пространство света.
    bool shadowPass = uniforms.shadow_params.w > 0.5;
    vec4 clip = shadowPass
        ? (uniforms.light_matrix * worldPos)
        : (uniforms.projection * uniforms.view * worldPos);
    gl_Position = clip;
    fragColor = inColor;
    fragTexCoord = inTexCoord;
    fragWorldPos = worldPos.xyz;
    fragNormal = inNormal;
    fragLightPos = uniforms.light_matrix * worldPos;
}