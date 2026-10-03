#version 450
// FXAA 3.11 (Lottes), один проход, без subpixel-мерцания высокого качества.
layout(binding = 0) uniform sampler2D screenTex;
layout(location = 0) in vec2 vUv;
layout(location = 0) out vec4 fragColor;

void main() {
    vec2 texelSize = 1.0 / vec2(textureSize(screenTex, 0));

    vec3 rgbNW = texture(screenTex, vUv + vec2(-1.0, -1.0) * texelSize).rgb;
    vec3 rgbNE = texture(screenTex, vUv + vec2( 1.0, -1.0) * texelSize).rgb;
    vec3 rgbSW = texture(screenTex, vUv + vec2(-1.0,  1.0) * texelSize).rgb;
    vec3 rgbSE = texture(screenTex, vUv + vec2( 1.0,  1.0) * texelSize).rgb;
    vec3 rgbM  = texture(screenTex, vUv).rgb;

    vec3 luma = vec3(0.299, 0.587, 0.114);
    float lumaNW = dot(rgbNW, luma);
    float lumaNE = dot(rgbNE, luma);
    float lumaSW = dot(rgbSW, luma);
    float lumaSE = dot(rgbSE, luma);
    float lumaM  = dot(rgbM,  luma);

    float lumaMin = min(lumaM, min(min(lumaNW, lumaNE), min(lumaSW, lumaSE)));
    float lumaMax = max(lumaM, max(max(lumaNW, lumaNE), max(lumaSW, lumaSE)));
    float lumaRange = lumaMax - lumaMin;

    // Нет контраста на краю — выходим без изменений.
    if (lumaRange < max(0.0312, lumaMax * 0.125)) {
        fragColor = vec4(rgbM, 1.0);
        return;
    }

    float lumaL = dot(texture(screenTex, vUv + vec2(-1.0, 0.0) * texelSize).rgb, luma);
    float lumaR = dot(texture(screenTex, vUv + vec2( 1.0, 0.0) * texelSize).rgb, luma);
    float lumaU = dot(texture(screenTex, vUv + vec2( 0.0,-1.0) * texelSize).rgb, luma);
    float lumaD = dot(texture(screenTex, vUv + vec2( 0.0, 1.0) * texelSize).rgb, luma);

    float lumaMinA = min(lumaMin, min(lumaU, lumaD));
    float lumaMaxA = max(lumaMax, max(lumaU, lumaD));

    // Направление края и длина шага.
    vec2 dir;
    dir.x = -((lumaL + lumaR) - 2.0 * lumaM);
    dir.y = -((lumaU + lumaD) - 2.0 * lumaM);
    float dirReduce = max((lumaL + lumaR + lumaU + lumaD) * 0.0625, 1.0 / 128.0);
    float rcpDirMin = 1.0 / (min(abs(dir.x), abs(dir.y)) + dirReduce);
    dir = clamp(dir * rcpDirMin, vec2(-8.0), vec2(8.0)) * texelSize;

    vec3 rgbA = 0.5 * (texture(screenTex, vUv + dir * 0.3333).rgb + texture(screenTex, vUv - dir * 0.3333).rgb);
    vec3 rgbB = rgbA * 0.5 + 0.25 * (texture(screenTex, vUv + dir * 0.6667).rgb + texture(screenTex, vUv - dir * 0.6667).rgb);

    float lumaB = dot(rgbB, luma);
    if (lumaB < lumaMinA || lumaB > lumaMaxA) {
        fragColor = vec4(rgbA, 1.0);
    } else {
        fragColor = vec4(rgbB, 1.0);
    }
}