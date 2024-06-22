#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use glium::backend::Facade;
use glium::glutin::surface::WindowSurface;
use glium::implement_vertex;
use glium::Display;
use glium::IndexBuffer;
use glium::Program;
use glium::VertexBuffer;

use obj::load_obj;
use obj::Obj;
use obj::ObjError;

extern crate glium;
extern crate obj;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    tex_coords: (f32, f32),
}

implement_vertex!(Vertex, position, tex_coords);

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32),
}

implement_vertex!(Normal, normal);

pub struct ObjectModel {
    pub vertices: glium::VertexBuffer<Vertex>,
    pub normals: glium::VertexBuffer<Normal>,
    pub indices: glium::IndexBuffer<u16>,
}

//#[derive(Default)]
pub struct GraphicComponent {
    pub is_active: bool,
    pub geometry: Option<String>,
    pub vertex_shader: Option<String>,
    pub fragment_shader: Option<String>,
}

impl<'a> GraphicComponent {
    pub fn new(
        model_path: Option<String>,
        vertex_shader_path: Option<String>,
        fragment_shader_path: Option<String>,
    ) -> Self {
        GraphicComponent {
            is_active: true,
            geometry: model_path,
            vertex_shader: vertex_shader_path,
            fragment_shader: fragment_shader_path,
        }
    }

    pub fn can_be_drawn(&self) -> bool {
        let res = self.geometry.is_some()
            && self.vertex_shader.is_some()
            && self.fragment_shader.is_some();
        return res;
    }

    pub fn add_shaders(&mut self, vertex_shader: String, fragment_shader: String) {
        self.vertex_shader = Some(vertex_shader);
        self.fragment_shader = Some(fragment_shader);
    }

    pub fn add_model(&mut self, model: String) {
        self.geometry = Some(model);
    }
    pub fn is_active(&self) -> bool {
        return self.is_active;
    }
}


pub fn load_model(model_file_path: &Path, display: &Display<WindowSurface>) -> Option<ObjectModel> {
    let file_result = File::open(model_file_path);
    match file_result {
        Err(err) => {
            println!("Warning, failed to open file: {}", err);
            return None;
        }
        Ok(file) => {
            let input = BufReader::new(file);
            let model_result: Result<Obj, ObjError> = load_obj(input);
            match model_result {
                Err(err) => {
                    println!("Warning, failed to load object: {}", err);
                    return None;
                }
                Ok(model) => {
                    let new_vertices: Vec<[f32; 3]>;
                    let new_normals: Vec<[f32; 3]>;
                    (new_vertices, new_normals) = model
                        .vertices
                        .iter()
                        .map(|vertex| (vertex.position, vertex.normal))
                        .unzip();

                    // need to factorise this, possibly with the previous line to avoid doing
                    // two loops
                    let vertices_vec: Vec<Vertex>;
                    let normals_vec: Vec<Normal>;
                    vertices_vec = new_vertices
                        .iter()
                        .map(|vertex| Vertex {
                            position: (vertex[0], vertex[1], vertex[2]),
                            tex_coords: (0.0, 0.0),
                        })
                        .collect();
                    normals_vec = new_normals
                        .iter()
                        .map(|normal| Normal {
                            normal: (normal[0], normal[1], normal[2]),
                        })
                        .collect();

                    let vertices_vertex_buffer = VertexBuffer::new(display, &vertices_vec);
                    let normals_vertex_buffer = VertexBuffer::new(display, &normals_vec);
                    let indices_vertex_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &model.indices,
                    );

                    if vertices_vertex_buffer.is_err()
                        || normals_vertex_buffer.is_err()
                        || indices_vertex_buffer.is_err()
                    {
                        println!("Error, could not create index buffers for this object");
                        return None;
                    } else {
                        // can use the unwraps because the if guarantees that they will not be
                        // errors
                        let new_geometry = ObjectModel {
                            vertices: vertices_vertex_buffer.unwrap(),
                            normals: normals_vertex_buffer.unwrap(),
                            indices: indices_vertex_buffer.unwrap(),
                        };
                        return Some(new_geometry);
                    }
                }
            }
        }
    }
}

// we have two options, we can chose to have the engine crash if anything unexpected happens or
// wait until we have no choice, will go with the middle ground of waiting as long as possible
// whilst loudly complaining
// check if shaders already loaded?
// TODO return an error, print warning and continue the best we can if function fails
pub fn load_shaders<'a, F: Facade>(
    vertex_shader_path: &str,
    fragment_shader_path: &str,
    facade: &'a F,
) -> Option<Program> {
    let vertex_file_res = fs::read_to_string(vertex_shader_path);
    let fragment_file_res = fs::read_to_string(fragment_shader_path);

    match vertex_file_res {
        Err(err) => {
            println!("Warning, failed to open vertex shader file: {}", err);
            return None;
        }
        Ok(_) => {}
    };

    match fragment_file_res {
        Err(err) => {
            println!("Warning, failed to open fragment shader file: {}", err);
            return None;
        }
        Ok(_) => {}
    };

    let vertex_shader_src = vertex_file_res.unwrap();
    let fragment_shader_src = fragment_file_res.unwrap();

    let res = glium::Program::from_source(facade, &vertex_shader_src, &fragment_shader_src, None);
    match res {
        Err(prog_err) => {
            println!("WARNING: shaders have failed to compile");
            println!("{}", prog_err.to_string());
            return None;
        }
        Ok(prog_res) => {
            return Some(prog_res);
        }
    }
}
