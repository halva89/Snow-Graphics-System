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

layout(binding = 1) uniform sampler2D texSampler;
layout(binding = 2) uniform sampler2D shadowMap;

layout(location = 0) in vec3 fragColor;
layout(location = 1) in vec2 fragTexCoord;
layout(location = 2) in vec3 fragWorldPos;
layout(location = 3) in vec3 fragNormal;
layout(location = 4) in vec4 fragLightPos;

layout(location = 0) out vec4 outColor;

// Коэффициент освещённости из карты теней: 1.0 = в свету, 0.0 = в тени.
// shadow_params: .x = базовый bias, .y = slope-коэффициент bias,
// .z = толщина полосы PCF (для раннего выхода). .w — переключатель пасса теней.
float shadow_factor(vec4 lightClip) {
    if (lightClip.w <= 0.0) return 1.0;
    vec3 proj = lightClip.xyz / lightClip.w;
    // Только XY — это NDC [-1,1], конвертируем в UV [0,1]. Z в Vulkan уже [0,1]
    // (ближняя плоскость света = 0, дальняя = 1), поэтому НЕ ремапим z.
    vec2 uv = proj.xy * 0.5 + 0.5;
    float ndcDepth = proj.z;
    // Вне карты теней — считаем освещённым.
    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || ndcDepth < 0.0 || ndcDepth > 1.0) return 1.0;

    // Slope-scaled bias: чем круче наклон поверхности к лучу света, тем больше
    // смещение — убирает shadow acne на полах/стенах, не «съедая» плоские грани.
    float slope = max(abs(dFdx(ndcDepth)), abs(dFdy(ndcDepth)));
    float bias = uniforms.shadow_params.x + uniforms.shadow_params.y * slope;
    float curDepth = ndcDepth - bias;

    // == Оптимизация ==
    // Один сэмпл в центре. Если мы (по глубине) далеко в свету или в тени —
    // выдаём результат сразу, без 3x3 PCF. PCF нужен только в узкой полосе
    // вокруг границы (толщина = shadow_params.z), то есть на небольшой доле пикселей.
    float delta = curDepth - texture(shadowMap, uv).r;
    float margin = uniforms.shadow_params.z;
    if (delta <= -margin) return 1.0;
    if (delta >=  margin) return 0.0;

    // == Сглаживание: 3x3 PCF (soft) вдоль границы ==
    vec2 texel = 1.0 / vec2(textureSize(shadowMap, 0));
    float lit = 0.0;
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            float occluder = texture(shadowMap, uv + vec2(x, y) * texel).r;
            lit += (curDepth <= occluder) ? 1.0 : 0.0;
        }
    }
    return lit / 9.0;
}

void main() {
    // Пасс теней: только глубина — пишем константу, ничего не читаем.
    if (uniforms.shadow_params.w > 0.5) {
        outColor = vec4(0.0);
        return;
    }

    vec4 texColor = texture(texSampler, fragTexCoord);
    vec3 base = fragColor * texColor.rgb;

    // Нулевая нормаль = unlit (оверлей, wireframe, термальный режим)
    if (length(fragNormal) < 1e-3) {
        // Шум датчика тепловизора: зернистость по координате пикселя
        float rn = fract(sin(dot(gl_FragCoord.xy, vec2(12.9898, 78.233))) * 43758.5453);
        float rn2 = fract(sin(dot(gl_FragCoord.xy, vec2(39.123, 197.83))) * 65731.17);
        outColor = vec4(base + (rn + rn2 - 1.0) * 0.08, 1.0);
        return;
    }

    vec3 N = normalize(fragNormal);
    vec3 L = normalize(uniforms.light_pos.xyz - fragWorldPos);
    vec3 V = normalize(uniforms.view_pos.xyz - fragWorldPos);
    vec3 H = normalize(L + V);

    float diff = max(dot(N, L), 0.0);
    float spec = pow(max(dot(N, H), 0.0), 64.0);
    float shadow = shadow_factor(fragLightPos);

    vec3 direct = uniforms.ambient.rgb + uniforms.light_color.rgb * (diff + spec * 0.6) * shadow;
    outColor = vec4(base * direct, 1.0);
}