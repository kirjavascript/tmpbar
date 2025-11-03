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
                uniform float u_zoom;

                void main() {
                    vec2 uv = vUV;
                    vec2 mouse = u_mouse;

                    // Simple zoom centered on mouse with direct mapping
                    uv = (uv - mouse) / u_zoom + mouse;

                    // Create a fractal pattern that benefits from zooming
                    vec2 c = uv * 4.0 - 2.0;
                    vec2 z = vec2(0.0);
                    float iterations = 0.0;

                    for (int i = 0; i < 100; i++) {
                        if (dot(z, z) > 4.0) break;
                        z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
                        iterations += 1.0;
                    }

                    // Color based on iterations and time
                    float t = iterations / 100.0;
                    vec3 color = vec3(
                        0.5 + 0.5 * sin(t * 6.28 + u_time),
                        0.5 + 0.5 * sin(t * 6.28 + u_time + 2.09),
                        0.5 + 0.5 * sin(t * 6.28 + u_time + 4.18)
                    );

                    // Add zoom indicator
                    float zoom_indicator = smoothstep(0.98, 1.0, length(uv - vec2(0.9, 0.1)));
                    color = mix(color, vec3(1.0, 1.0, 0.0), zoom_indicator * (u_zoom - 1.0) * 0.5);

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

                // Mandelbulb distance function
                float mandelbulb(vec3 pos) {
                    vec3 z = pos;
                    float dr = 1.0;
                    float r = 0.0;
                    float power = 8.0;
                    
                    for (int i = 0; i < 15; i++) {
                        r = length(z);
                        if (r > 2.0) break;
                        
                        // Convert to polar coordinates
                        float theta = acos(z.z / r);
                        float phi = atan(z.y, z.x);
                        dr = pow(r, power - 1.0) * power * dr + 1.0;
                        
                        // Scale and rotate the point
                        float zr = pow(r, power);
                        theta = theta * power;
                        phi = phi * power;
                        
                        // Convert back to cartesian coordinates
                        z = zr * vec3(sin(theta) * cos(phi), sin(phi) * sin(theta), cos(theta));
                        z += pos;
                    }
                    
                    return 0.5 * log(r) * r / dr;
                }

                // Raymarching function
                float raymarch(vec3 ro, vec3 rd) {
                    float t = 0.0;
                    for (int i = 0; i < 64; i++) {
                        vec3 pos = ro + t * rd;
                        float d = mandelbulb(pos);
                        if (d < 0.001 || t > 10.0) break;
                        t += d * 0.5;
                    }
                    return t;
                }

                // Calculate normal using finite differences
                vec3 calcNormal(vec3 pos) {
                    vec2 e = vec2(0.001, 0.0);
                    return normalize(vec3(
                        mandelbulb(pos + e.xyy) - mandelbulb(pos - e.xyy),
                        mandelbulb(pos + e.yxy) - mandelbulb(pos - e.yxy),
                        mandelbulb(pos + e.yyx) - mandelbulb(pos - e.yyx)
                    ));
                }

                void main() {
                    vec2 uv = (vUV - 0.5) * 2.0;
                    vec2 mouse = (u_mouse - 0.5) * 2.0;
                    
                    // Camera setup - mouse controls rotation
                    float camX = mouse.x * 3.14159;
                    float camY = mouse.y * 1.57;
                    
                    vec3 ro = vec3(
                        3.0 * cos(camY) * cos(camX),
                        3.0 * sin(camY),
                        3.0 * cos(camY) * sin(camX)
                    );
                    
                    vec3 target = vec3(0.0);
                    vec3 up = vec3(0.0, 1.0, 0.0);
                    
                    vec3 forward = normalize(target - ro);
                    vec3 right = normalize(cross(forward, up));
                    up = cross(right, forward);
                    
                    vec3 rd = normalize(forward + uv.x * right + uv.y * up);
                    
                    // Raymarch the scene
                    float t = raymarch(ro, rd);
                    
                    vec3 color = vec3(0.0);
                    
                    if (t < 10.0) {
                        vec3 pos = ro + t * rd;
                        vec3 normal = calcNormal(pos);
                        
                        // Lighting
                        vec3 lightDir = normalize(vec3(1.0, 1.0, 1.0));
                        float diff = max(dot(normal, lightDir), 0.0);
                        
                        // Specular
                        vec3 viewDir = normalize(ro - pos);
                        vec3 reflectDir = reflect(-lightDir, normal);
                        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
                        
                        // Color based on position and time
                        vec3 baseColor = vec3(
                            0.5 + 0.5 * sin(pos.x * 2.0 + u_time),
                            0.5 + 0.5 * sin(pos.y * 2.0 + u_time + 2.0),
                            0.5 + 0.5 * sin(pos.z * 2.0 + u_time + 4.0)
                        );
                        
                        color = baseColor * (0.3 + 0.7 * diff) + vec3(spec);
                        
                        // Fog
                        color = mix(color, vec3(0.1, 0.1, 0.2), smoothstep(2.0, 8.0, t));
                    } else {
                        // Background gradient
                        color = mix(vec3(0.1, 0.1, 0.2), vec3(0.0, 0.0, 0.1), uv.y * 0.5 + 0.5);
                    }
                    
                    FragColor = vec4(color, 1.0);
                }
            ]],
        }),
    },
})
