local ui = require('ui')

ui.bar({
    monitor = { index = 2 },
    position = 'top',

    min_interval = 0,

    style = {
        height = 200,
    },

    items = {
        ui.shader({
            style = {
                size = 300,
            },
            version = '#version 330',
            vertex = [[
                void main() {
                    const vec4 positions[3] = vec4[3](
                        vec4(-1.0, -1.0, 0.0, 1.0),
                        vec4( 3.0, -1.0, 0.0, 1.0),
                        vec4(-1.0,  3.0, 0.0, 1.0)
                    );
                    gl_Position = positions[gl_VertexID];
                }
            ]],
            fragment = [[
                out vec4 color;

                void main() {
                    color = vec4(1.0); // white
                }
            ]],
        }),
        ui.shader({
            style = {
                size = 300,
            },
            version = '#version 330 core',
            vertex = [[
                out vec2 vUV;

                void main() {
                    const vec2 verts[3] = vec2[3](
                        vec2(-1.0, -1.0),
                        vec2( 3.0, -1.0),
                        vec2(-1.0,  3.0)
                    );

                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);

                    vUV = gl_Position.xy * 0.5 + 0.5;
                }
            ]],
            fragment = [[
                in vec2 vUV;
                out vec4 FragColor;

                uniform int numChecks = 8; // number of squares per row/column
                void main() {

                    // Scale UV to number of checks
                    vec2 scaled = vUV * float(numChecks);

                    // Compute checkerboard pattern
                    float check = mod(floor(scaled.x) + floor(scaled.y), 2.0);

                    // Black & white checkerboard
                    FragColor = vec4(vec3(check), 1.0);
                }
            ]],
        }),
        ui.shader({
            style = {
                size = 300,
            },
            vertex = [[
                const vec2 verts[3] = vec2[3](
                    vec2(0.0, 1.0),
                    vec2(-1.0, -1.0),
                    vec2(1.0, -1.0)
                    );
                const vec4 colors[3] = vec4[3](
                    vec4(1.0, 0.0, 0.0, 1.0),
                    vec4(0.0, 1.0, 0.0, 1.0),
                    vec4(0.0, 0.0, 1.0, 1.0)
                    );
                out vec4 v_color;
                uniform float time_delta;
                void main() {
                    v_color = colors[gl_VertexID];
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    gl_Position.x *= cos(time_delta);
                }
            ]],
            fragment = [[
                precision mediump float;
                in vec4 v_color;
                out vec4 out_color;
                void main() {
                    out_color = v_color;
                }
            ]],
        }),
        ui.shader({
            style = {
                size = 300,
            },
            version = '#version 330 core',
            vertex = [[
                out vec2 vUV;
                void main() {
                    const vec2 verts[3] = vec2[3](
                        vec2(-1.0, -1.0),
                        vec2( 3.0, -1.0),
                        vec2(-1.0,  3.0)
                    );
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    vUV = gl_Position.xy * 0.5 + 0.5;
                }
            ]],
            fragment = [[
                in vec2 vUV;
                out vec4 FragColor;
                uniform float time_delta;

                void main() {
                    vec2 center = vec2(0.5, 0.5);
                    float dist = distance(vUV, center);
                    float wave = sin(dist * 20.0 - time_delta * 5.0) * 0.5 + 0.5;
                    vec3 color = vec3(wave * 0.8, wave * 0.4, wave);
                    FragColor = vec4(color, 1.0);
                }
            ]],
        }),
        ui.shader({
            style = {
                size = 300,
            },
            version = '#version 330 core',
            vertex = [[
                out vec2 vUV;
                void main() {
                    const vec2 verts[3] = vec2[3](
                        vec2(-1.0, -1.0),
                        vec2( 3.0, -1.0),
                        vec2(-1.0,  3.0)
                    );
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    vUV = gl_Position.xy * 0.5 + 0.5;
                }
            ]],
            fragment = [[
                in vec2 vUV;
                out vec4 FragColor;
                uniform float time_delta;

                void main() {
                    vec2 uv = vUV * 2.0 - 1.0;
                    float r = length(uv);
                    float a = atan(uv.y, uv.x);

                    float spiral = sin(r * 10.0 - a * 3.0 + time_delta * 2.0);
                    vec3 color = vec3(
                        0.5 + 0.5 * cos(spiral + 0.0),
                        0.5 + 0.5 * cos(spiral + 2.094),
                        0.5 + 0.5 * cos(spiral + 4.188)
                    );

                    FragColor = vec4(color, 1.0);
                }
            ]],
        }),
        ui.shader({
            style = {
                size = 300,
            },
            version = '#version 330 core',
            vertex = [[
                out vec2 vUV;
                void main() {
                    const vec2 verts[3] = vec2[3](
                        vec2(-1.0, -1.0),
                        vec2( 3.0, -1.0),
                        vec2(-1.0,  3.0)
                    );
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    vUV = gl_Position.xy * 0.5 + 0.5;
                }
            ]],
            fragment = [[
                in vec2 vUV;
                out vec4 FragColor;
                uniform float time_delta;

                float noise(vec2 p) {
                    return fract(sin(dot(p, vec2(12.9898, 78.233))) * 43758.5453);
                }

                void main() {
                    vec2 uv = vUV * 8.0;
                    vec2 i = floor(uv);
                    vec2 f = fract(uv);

                    float a = noise(i);
                    float b = noise(i + vec2(1.0, 0.0));
                    float c = noise(i + vec2(0.0, 1.0));
                    float d = noise(i + vec2(1.0, 1.0));

                    vec2 u = f * f * (3.0 - 2.0 * f);
                    float n = mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;

                    vec3 color = vec3(n) * vec3(0.8, 0.9, 1.0);
                    FragColor = vec4(color, 1.0);
                }
            ]],
        }),
        ui.shader({
            style = {
                size = 300,
            },
            version = '#version 330 core',
            vertex = [[
                out vec2 vUV;
                void main() {
                    const vec2 verts[3] = vec2[3](
                        vec2(-1.0, -1.0),
                        vec2( 3.0, -1.0),
                        vec2(-1.0,  3.0)
                    );
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    vUV = gl_Position.xy * 0.5 + 0.5;
                }
            ]],
            fragment = [[
                in vec2 vUV;
                out vec4 FragColor;
                uniform float time_delta;

                void main() {
                    vec2 uv = (vUV - 0.5) * 2.0;
                    float t = time_delta * 0.5;

                    for(int i = 0; i < 3; i++) {
                        float fi = float(i);
                        uv = abs(uv) / dot(uv, uv) - vec2(0.9 + 0.2 * cos(t + fi));
                    }

                    vec3 color = vec3(length(uv));
                    color = pow(color, vec3(0.4545));

                    FragColor = vec4(color, 1.0);
                }
            ]],
        }),
        ui.shader({
            style = {
                size = 300,
            },
            version = '#version 330 core',
            vertex = [[
                out vec2 vUV;
                void main() {
                    const vec2 verts[3] = vec2[3](
                        vec2(-1.0, -1.0),
                        vec2( 3.0, -1.0),
                        vec2(-1.0,  3.0)
                    );
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    vUV = gl_Position.xy * 0.5 + 0.5;
                }
            ]],
            fragment = [[
                in vec2 vUV;
                out vec4 FragColor;
                uniform float time_delta;

                void main() {
                    vec2 uv = vUV * 2.0 - 1.0;
                    float d = length(uv);

                    float rings = sin(d * 15.0 - time_delta * 3.0);
                    float fade = 1.0 - smoothstep(0.0, 1.0, d);

                    vec3 color1 = vec3(1.0, 0.3, 0.8);
                    vec3 color2 = vec3(0.2, 0.8, 1.0);
                    vec3 color = mix(color1, color2, rings * 0.5 + 0.5);

                    FragColor = vec4(color * fade, 1.0);
                }
            ]],
        }),
    },
})
