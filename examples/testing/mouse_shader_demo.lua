local ui = require('ui')

ui.bar({
    position = 'top',
    min_interval = 0,
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
                uniform vec2 u_mouse;
                uniform vec2 u_resolution;
                uniform float u_time;

                void main() {
                    vec2 uv = vUV;
                    vec2 mouse = u_mouse;

                    // Distance from mouse position
                    float dist = distance(uv, mouse);

                    // Create ripple effect from mouse position
                    float ripple = sin(dist * 20.0 - u_time * 8.0) * 0.5 + 0.5;
                    ripple *= 1.0 - smoothstep(0.0, 0.3, dist);

                    // Color based on mouse position
                    vec3 color = vec3(mouse.x, mouse.y, 0.5 + 0.5 * sin(u_time));
                    color = mix(color, vec3(1.0), ripple);

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
                uniform vec2 u_mouse;
                uniform float u_time;

                void main() {
                    vec2 uv = vUV;
                    vec2 mouse = u_mouse;

                    // Create a light source at mouse position
                    float light = 1.0 / (distance(uv, mouse) * 5.0 + 0.1);

                    // Animated background pattern
                    vec2 grid = fract(uv * 10.0 + u_time * 0.5);
                    float pattern = smoothstep(0.4, 0.6, grid.x) * smoothstep(0.4, 0.6, grid.y);

                    vec3 color = vec3(pattern * 0.2) + vec3(light * 0.8, light * 0.4, light * 0.2);
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
                uniform vec2 u_mouse;
                uniform float u_time;

                void main() {
                    vec2 uv = vUV;
                    vec2 mouse = u_mouse;

                    // Create swirling effect around mouse
                    vec2 toMouse = uv - mouse;
                    float angle = atan(toMouse.y, toMouse.x);
                    float radius = length(toMouse);

                    // Spiral distortion
                    angle += radius * 10.0 + u_time * 2.0;

                    vec2 spiral = vec2(cos(angle), sin(angle)) * radius;
                    spiral += mouse;

                    // Color based on spiral position
                    vec3 color = vec3(
                        0.5 + 0.5 * sin(spiral.x * 10.0),
                        0.5 + 0.5 * sin(spiral.y * 10.0 + 2.0),
                        0.5 + 0.5 * sin((spiral.x + spiral.y) * 5.0 + 4.0)
                    );

                    // Fade based on distance from mouse
                    float fade = 1.0 - smoothstep(0.0, 0.5, radius);
                    color *= fade + 0.2;

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
                uniform vec2 u_mouse;
                uniform vec2 u_resolution;
                uniform float u_time;

                void main() {
                    vec2 uv = vUV;
                    vec2 mouse = u_mouse;

                    // Create multiple attraction points
                    vec2 p1 = mouse;
                    vec2 p2 = vec2(1.0 - mouse.x, mouse.y);
                    vec2 p3 = vec2(mouse.x, 1.0 - mouse.y);
                    vec2 p4 = vec2(1.0 - mouse.x, 1.0 - mouse.y);

                    float d1 = distance(uv, p1);
                    float d2 = distance(uv, p2);
                    float d3 = distance(uv, p3);
                    float d4 = distance(uv, p4);

                    // Create interference pattern
                    float wave1 = sin(d1 * 15.0 - u_time * 3.0);
                    float wave2 = sin(d2 * 15.0 - u_time * 3.0 + 1.57);
                    float wave3 = sin(d3 * 15.0 - u_time * 3.0 + 3.14);
                    float wave4 = sin(d4 * 15.0 - u_time * 3.0 + 4.71);

                    float interference = (wave1 + wave2 + wave3 + wave4) * 0.25;

                    vec3 color = vec3(
                        0.5 + 0.5 * interference,
                        0.5 + 0.3 * sin(interference * 2.0),
                        0.5 + 0.5 * cos(interference * 1.5)
                    );

                    FragColor = vec4(color, 1.0);
                }
            ]],
        }),
    },
})
