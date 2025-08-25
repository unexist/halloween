precision mediump float;

uniform float u_time;
uniform vec2 u_resolution;
uniform vec2 u_imageSize;
uniform sampler2D iChannel0;
uniform float u_aspectRatio;
uniform int u_isMobile;

float random(float seed) {
    return fract(sin(seed) * 43758.5453123);
}

float randomRange(float min, float max, float seed) {
    return mix(min, max, random(seed));
}

float rand(vec2 p){
    p += .2127 + p.x + .3713 * p.y;
    vec2 r = 4.789 * sin(789.123 * (p));
    return fract(r.x * r.y);
}

float sn(vec2 p){
    vec2 i = floor(p - .5);
    vec2 f = fract(p - .5);
    f = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    float rt = mix(rand(i), rand(i + vec2(1., 0.)), f.x);
    float rb = mix(rand(i + vec2(0., 1.)), rand(i + vec2(1., 1.)), f.x);
    return mix(rt, rb, f.y);
}

float Hash( vec2 p)
{
    vec3 p2 = vec3(p.xy,1.0);
    return fract(sin(dot(p2,vec3(37.1,61.7, 12.4)))*3758.5453123);
}

float noise(in vec2 p)
{
    vec2 i = floor(p);
    vec2 f = fract(p);
    f *= f * (3.0-2.0*f);

    return mix(mix(Hash(i + vec2(0.,0.)), Hash(i + vec2(1.,0.)),f.x),
    mix(Hash(i + vec2(0.,1.)), Hash(i + vec2(1.,1.)),f.x),
    f.y);
}

float fbm(vec2 p, int octaves) {
    float v = 0.0;
    float amplitude = 0.5;
    float frequency = 1.0;

    for (int i = 0; i < 10; i++) {
        if (i >= octaves) break;
        v += noise(p * frequency) * amplitude;
        frequency *= 2.0;
        amplitude *= 0.5;
    }

    return v;
}

vec3 lightning(vec2 uv, float lSize, int lCount, float xPos) {

    float timeVal = u_time;
    vec3 finalColor = vec3(0.0);

    vec2 scaledUV = uv * u_resolution / max(u_resolution.x, u_resolution.y);

    if (xPos == 0.0) {
        scaledUV.x = 0.5;
    } else {
        scaledUV.x += (xPos + 0.5) * 2.0;
        if (u_resolution.x < u_resolution.y) {
            scaledUV.x += 0.35 * (u_resolution.y / u_resolution.x);
        }
    }

    if (u_resolution.x >= u_resolution.y) {
        scaledUV.y *= 2.5;
    }

    lSize *= randomRange(0.8, 1.2, Hash(uv));

    for (int i = 0; i < 10; ++i) {
        if (i >= lCount) break;
        float indexAsFloat = float(i);
        float amp = 80.0 * lSize + (indexAsFloat * 2.0);
        float period = 1.0 + (indexAsFloat + 1.0);

        float thickness = mix(0.2 * lSize, 1.0 * lSize, scaledUV.y * 0.5 + 0.5);
        float intensity = mix(1.2 * lSize, 2.0 * lSize, noise(scaledUV * 2.0));

        float t = abs(thickness / (sin(scaledUV.x + fbm(scaledUV + timeVal * period, 5)) * amp) * intensity);
        float show = fract(abs(sin(timeVal))) >= 0.95 ? 1.0 : 0.0;
        show *= step(abs(fbm(vec2(sin(u_time * 50.0), 0.0), 5)), 0.4);

        finalColor += t * vec3(2.5, 0.7, 0.5) * show;
    }

    return finalColor;
}

vec3 createFog(vec2 uv) {

    float fogDensity = 0.5;
    float fogScale = 4.5;

    vec2 animatedUV = uv;
    animatedUV.x += u_time * 0.1;
    animatedUV.y += u_time * 0.05;

    float fogNoise = fbm(animatedUV * fogScale, 2);
    vec3 fogColor = vec3(0.4804, 0.1392, 0.2098);

    vec3 fogEffect = fogColor * fogNoise * fogDensity;
    return fogEffect;
}

void main() {
    vec2 uv = gl_FragCoord.xy / u_resolution.y;

    vec2 p = uv.xy * vec2(3., 4.3);
    float f = .5 * sn(p) + .25 * sn(2. * p) + .125 * sn(4. * p) + .0625 * sn(8. * p) + .03125 * sn(16. * p) + .015 * sn(32. * p);

    float newT = u_time * 0.4 + sn(vec2(u_time * 1.)) * 0.1;
    p.x -= u_time * 0.2;

    p.y *= 1.3;
    float f2 = .5 * sn(p) + .25 * sn(2.04 * p + newT * 1.1) - .125 * sn(4.03 * p - u_time * 0.3) + .0625 * sn(8.02 * p - u_time * 0.4) + .03125 * sn(16.01 * p + u_time * 0.5) + .018 * sn(24.02 * p);

    float f4 = f2 * smoothstep(0.0, 1., uv.y);

    vec2 newUV = uv;
    newUV.x -= u_time * 0.3;
    newUV.y += u_time * 3.;

    float strength = 0.05 * sin(u_time * 0.5 + sn(newUV)) + 0.2;

    float rain = 0.0;

    if (u_isMobile == 0) {
        rain = sn(vec2(newUV.x * 20.1, newUV.y * 40.1 + newUV.x * 400.1 - 20. * strength));
        float rain2 = sn(vec2(newUV.x * 45. + u_time * 0.5, newUV.y * 30.1 + newUV.x * 200.1));
        rain = strength - rain;
        rain += smoothstep(0.2, 0.5, f4 + 0.1) * rain;
        rain += rain2 * (sin(strength) - 0.4) * 2.;
        rain = clamp(rain, 0., 0.5) * 0.5;
    }

    vec3 blueColor = vec3(0.5, 0.0, 1.5);
    vec3 redColor = vec3(2.0, 0.0, 3.0);
    float gradientFactor = uv.y;
    vec3 gradientColor = mix(redColor, blueColor, gradientFactor);

    vec3 clouds = mix(vec3(-0.2, -0.4, -0.15), vec3(3.2, 1.4, 1.3), f4 * f);

    vec3 painting = clouds;
    painting *= mix(painting, gradientColor, 1.0);

    float r = 1. - length(max(abs(gl_FragCoord.xy / u_resolution.xy * 2. - 1.) - .5, 0.));
    painting *= r;

    vec2 imageUV = vec2((gl_FragCoord.x - u_resolution.x * 0.5 + u_imageSize.x * 0.5) / u_imageSize.x, 1.0 - (gl_FragCoord.y - u_resolution.y * 0.5 + u_imageSize.y * 0.5) / u_imageSize.y);
    vec4 imageColor = texture2D(iChannel0, imageUV);

    float lightEnhanceFactor = 1.0 + 0.75 * clamp(sin(u_time*2.0), 0.0, 1.0);
    float lightAreas = smoothstep(0.7, 1.0, imageColor.r);

    vec3 enhancedLightAreas = imageColor.rgb * lightEnhanceFactor * lightAreas;
    vec3 finalImageColor = mix(imageColor.rgb, enhancedLightAreas, lightAreas);

    float simpleRandom = fract(sin(u_time) * 43758.5453123);
    int llCount = (simpleRandom > 0.75) ? 2 : 1;

    vec3 lightningColor = lightning(uv, 0.15, llCount, 0.35);

    vec3 lightningGlowColor = lightning(uv, 2.0, llCount * 2, 0.0);

    painting += lightningColor;

    imageColor = mix(vec4(painting, 1.), vec4(finalImageColor, 1.0), imageColor.a);

    vec3 fogEffect = createFog(uv);

    float fogGradient = smoothstep(0.3, 0.0, uv.y);
    fogEffect *= fogGradient * 1.5;

    gl_FragColor = imageColor;

    if (u_isMobile == 0) {
        gl_FragColor += clamp(rain, 0., 1.);
    }

    gl_FragColor.rgb += lightningGlowColor;
    gl_FragColor.rgb += fogEffect;
}
