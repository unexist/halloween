// Heavily based on https://www.shadertoy.com/view/flBcD3

#ifdef GL_ES
precision mediump float;
#endif

///Render water
#define MAX 100.
#define EPS 4e-4

uniform float u_time;
uniform vec2 u_resolution;
uniform vec2 u_imageSize;
uniform sampler2D iChannel0;
uniform int u_isMobile;

// Classic pseudo-random hash
float hash(vec2 p) {
    return fract(sin(p.x * 75.3 + p.y * 94.2) * 4952.);
}

// Bi-cubic value noise
float value(vec2 p) {
    vec2 f = floor(p);
    vec2 s = p - f;
    s *= s * (3.0 - 2.0 * s);
    vec2 o = vec2(0, 1);

    return mix(mix(hash(f + o.xx), hash(f + o.yx), s.x),
            mix(hash(f + o.xy), hash(f + o.yy), s.x), s.y);
}

// Approximate SDF from fractal value noise
float dist(vec3 p) {
    vec2 n = p.xz * 0.6 + 1.0;
    mat2 m = mat2(0.6754904, 0.7373688, -0.7373688, 0.6754904) * 2.0;
    float weight = 0.3;
    float water = 0.0;
    float speed = 0.2;

    for (int i = 0; i < 10; i++) {
        water += smoothstep(0.1, 0.9, value(n + speed * u_time)) * weight;
        n *= m;
        speed *= 1.3;
        weight *= 0.46;
    }

    return (water + 0.5 - p.y);
}

// Compute normals from SDF derivative
vec3 normal(vec3 p) {
    vec2 e = vec2(4, -4) * EPS;

    return normalize(dist(p + e.yxx) * e.yxx + dist(p + e.xyx) * e.xyx +
    dist(p + e.xxy) * e.xxy + dist(p + e.yyy) * e.yyy);
}

#define T texture(iChannel0, gl_FragCoord.xy / r

// Render water
void main() {
    vec3 ray = normalize(vec3(gl_FragCoord.xy * 2.0 - u_resolution.y, u_resolution.x));
    ray.yz *= mat2(cos(0.5 + vec4(0, 11, 33, 0)));
    vec3 pos = vec3(u_time * 0.2, 0, 0);
    vec4 mar = vec4(pos, 0);

    for (int i = 0; i < 50; i++) {
        float stp = dist(mar.xyz);
        mar += vec4(ray, 1) * stp;

        if (stp < EPS || mar.w > MAX) break;
    }

    vec3 nor = normal(mar.xyz);
    vec3 sun = normalize(vec3(0, -1, 9));
    vec3 ref = refract(ray, nor, 1.333);
    float spec = exp(dot(ref, sun) * 9.0 - 9.0);
    float fog = max(1.0 - mar.w / MAX, 0.0);

    gl_FragColor = vec4(vec3(sqrt(spec) * fog), 1. - 2. / mar.w);

    // Bookeh pass
    /*vec2 r = u_resolution, p = vec2(T).a * r.y / 8e2, gl_FragColor -= gl_FragColor);

    for (float i = 1.; i < 16.; i += 1. / i) {
        p *= -mat2(.7374, .6755, -.6755, .7374);
        gl_FragColor += exp(vec4(1, T + p * i / r)) / .1);
    }

    gl_FragColor = log(gl_FragColor.gbar * .1);
    gl_FragColor /= gl_FragColor.a;
    gl_FragColor += T, .5 - ceil(log(.5 / r.y))) - .1; */
}
