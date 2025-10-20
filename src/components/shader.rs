use egui::Ui;
use crate::config::{Component, Property};
use crate::global::Global;

use eframe::glow;
use std::time::Instant;

#[derive(Clone)]
struct ShaderState {
    inner: Option<Shader>,
}

impl Default for ShaderState {
    fn default() -> Self {
        Self {
            inner: None,
        }
    }
}

impl ShaderState {
    fn init(&mut self, comp: &mut Component, gl: &glow::Context) {
        use glow::HasContext as _;

        if self.inner.is_none() {
            let props = comp.props();

            let shader_version = if let Some(Property::String(version)) = props.get("version") {
                version
            } else {
                "#version 330"
            };

            let vertex_shader_source: String = props.get("vertex").unwrap_or_default().into();
            let fragment_shader_source: String = props.get("fragment").unwrap_or_default().into();

            unsafe {
                let program = gl.create_program().expect("Cannot create program");

                let shader_sources = [
                    (glow::VERTEX_SHADER, vertex_shader_source),
                    (glow::FRAGMENT_SHADER, fragment_shader_source),
                ];

                let shaders: Vec<_> = shader_sources
                    .iter()
                    .map(|(shader_type, shader_source)| {
                        let shader = gl
                            .create_shader(*shader_type)
                            .expect("Cannot create shader");
                        gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                        gl.compile_shader(shader);
                        assert!(
                            gl.get_shader_compile_status(shader),
                            "Failed to compile {shader_type}: {}",
                            gl.get_shader_info_log(shader)
                        );
                        gl.attach_shader(program, shader);
                        shader
                    })
                .collect();

                gl.link_program(program);
                assert!(
                    gl.get_program_link_status(program),
                    "{}",
                    gl.get_program_info_log(program)
                );

                for shader in shaders {
                    gl.detach_shader(program, shader);
                    gl.delete_shader(shader);
                }

                let vertex_array = gl
                    .create_vertex_array()
                    .expect("Cannot create vertex array");

                self.inner = Some(Shader {
                    program,
                    vertex_array,
                    start: Instant::now(),
                });
            }
        }
    }
}

#[derive(Clone)]
struct Shader {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    start: Instant,
}

impl Shader {
    fn paint(&self, gl: &glow::Context) {
        use glow::HasContext as _;

        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "time_delta").as_ref(),
                self.start.elapsed().as_secs_f32(),
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}

pub fn render(comp: &mut Component, ui: &mut Ui, global: &mut Global) {
    // uniqueness relies on the fact that each component is a child of a TUI cell
    let id = ui.id();

    let mut state: ShaderState = ui
        .data_mut(|d| d.get_persisted(id))
        .unwrap_or_default();

    state.init(comp, &*global.gl);

    if let Some(shader) = &state.inner {
        let available = ui.available_size();

        let (rect, _response) =
            ui.allocate_exact_size(available, egui::Sense::drag());

        let shader = shader.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(eframe::egui_glow::CallbackFn::new(move |_info, painter| {
                shader.paint(painter.gl());
            })),
        };
        ui.painter().add(callback);
    }

    ui.data_mut(|d| d.insert_persisted(id, state));
}
