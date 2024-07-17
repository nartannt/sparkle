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

use tobj::load_obj;

extern crate glium;
extern crate tobj;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    normal: (f32, f32, f32),
    tex_coord: (f32, f32),
}

implement_vertex!(Vertex, position, normal, tex_coord);

pub struct ObjectModel {
    pub vertices: glium::VertexBuffer<Vertex>,
    pub indices: glium::IndexBuffer<u32>,
}

//#[derive(Default)]
pub struct GraphicComponent {
    pub is_active: bool,
    pub model_path: Option<String>,
    pub texture_path: Option<String>,
    pub vertex_shader_path: Option<String>,
    pub fragment_shader_path: Option<String>,
}

impl<'a> GraphicComponent {
    pub fn new(
        model_path: Option<String>,
        vertex_shader_path: Option<String>,
        fragment_shader_path: Option<String>,
    ) -> Self {
        GraphicComponent {
            is_active: true,
            model_path,
            texture_path: None,
            vertex_shader_path,
            fragment_shader_path,
        }
    }

    pub fn can_be_drawn(&self) -> bool {
        let res = self.model_path.is_some()
            && self.vertex_shader_path.is_some()
            && self.fragment_shader_path.is_some();
        return res;
    }

    pub fn add_shaders(&mut self, vertex_shader: String, fragment_shader: String) {
        self.vertex_shader_path = Some(vertex_shader);
        self.fragment_shader_path = Some(fragment_shader);
    }

    pub fn add_texture(&mut self, texture_path: String) {
        self.texture_path = Some(texture_path);
    }

    pub fn add_model(&mut self, model_path: String) {
        self.model_path = Some(model_path);
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
            let models_result = load_obj(model_file_path, &tobj::GPU_LOAD_OPTIONS);
            match models_result {
                Err(err) => {
                    println!("Warning, failed to load object: {}", err);
                    return None;
                }
                Ok((models, materials)) => {
                    // TODO this implies that if a single .obj file contains more than one model,
                    // only the first one will loaded, the rest will be ignored
                    let mesh = &models[0].mesh;
                    
                    let positions = mesh.positions.chunks_exact(3);
                    let normals = mesh.normals.chunks_exact(3);
                    // TODO this is not very pretty please don't be mad 🥺👉👈
                    // TODO investigate because i cannot fathom why this is necessary
                    let tmp = vec![0.0f32; 2 * mesh.positions.len() / 3];
                    let tex_coords = 
                        if mesh.texcoords.len() > 0 {
                            mesh.texcoords.chunks_exact(2)
                        } else {
                            tmp.chunks_exact(2)
                        };

                    let vertex_info = positions.zip(normals).zip(tex_coords);

                    // need to factorise this, possibly with the previous line to avoid doing
                    // two loops
                    let vertices_vec: Vec<Vertex>;
                    vertices_vec = vertex_info
                        .map(|((position, normal), tex_coord)| Vertex {
                            position: (position[0], position[1], position[2]),
                            normal: (normal[0], normal[1], normal[2]),
                            tex_coord: (tex_coord[0], tex_coord[1]),
                        })
                        .collect();

                    let vertices_vertex_buffer = VertexBuffer::new(display, &vertices_vec);
                    let indices_vertex_buffer = IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &mesh.indices,
                    );

                    if vertices_vertex_buffer.is_err()
                        || indices_vertex_buffer.is_err()
                    {
                        println!("Error, could not create index buffers for this object");
                        return None;
                    } else {
                        // can use the unwraps because the if guarantees that they will not be
                        // errors
                        let new_geometry = ObjectModel {
                            vertices: vertices_vertex_buffer.unwrap(),
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
