local ui = require('ui')

ui.bar({
    monitor = { index = 2 },
    position = 'top',

    style = {
        height = 300,
    },

    items = {
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
                uniform float u_angle;
                void main() {
                    v_color = colors[gl_VertexID];
                    gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                    gl_Position.x *= cos(u_angle);
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
    },
})
