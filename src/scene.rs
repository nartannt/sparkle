#![allow(dead_code)]
#![allow(unused_variables)]

use crate::game_object::GameObject;
use crate::input::MouseState;
use crate::input::KeyboardState;

use crate::script_component::ScriptComponent;

use crate::camera::Camera;
use crate::graphic_component::load_model;
use crate::graphic_component::load_shaders;
use crate::graphic_component::GraphicComponent;
use crate::graphic_component::ObjectModel;
use crate::transform::Transform;

use glium::glutin::surface::WindowSurface;
use glium::uniform;
use glium::Display;
use glium::Frame;
use glium::Program;
use glium::Surface;

use glium::winit::event::WindowEvent;

use glium::texture::RawImage2d;
use glium::texture::Texture2d;

use legion::storage::Component;
use legion::world::World;
use legion::world::WorldOptions;
use legion::IntoQuery;
use legion::Entity;
use legion::Schedule;
use legion::systems::Builder;
use legion::systems::Resources;
use legion::systems::System;
use legion::systems::Step::Systems;
use legion::systems::Executor;
use legion::systems::Step;

use legion::systems::ParallelRunnable;

use image::io::Reader as ImageReader;

use std::collections::HashMap;
use std::path::Path;

pub struct Scene {
    pub name: String,

    // ideally we would like to maintain the invariant that only one scene is active at a time, i
    // currently don't know how that would be enforced
    pub is_active: bool,

    // list of all the gameobjects in our scene
    pub game_objects: HashMap<i64, Entity>,

    // since we will only deal with go in relation to their scene
    // it makes sense to have a world per scene
    pub world: World,

    // vec of steps to execute each frame
    frame_steps : Vec<Step>,

    // when we add a GameObject to a scene,
    // if it has a GraphicComponent, if its model already exists, we don't do anything
    // else, we fetch it in the files and add it to the scene models
    pub models: HashMap<String, ObjectModel>,

    // same thing as models except for shaders
    // the first String is for the vertex shaders and the second one for fragment shaders
    pub programs: HashMap<(String, String), Program>,

    pub textures: HashMap<String, Texture2d>,

    // the camera which will draw the scene next, if it is none, the scene is not rendered
    pub render_cam: Option<Camera>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            name: "new scene".to_string(),
            is_active: true,
            game_objects: HashMap::new(),
            models: HashMap::new(),
            programs: HashMap::new(),
            textures: HashMap::new(),
            world: World::new(WorldOptions::default()),
            frame_steps : Vec::new(),
            render_cam: None,
        }
    }

    pub fn add_component<C: Component>(&mut self, go: &GameObject, component: C) {
        let entity = self.game_objects.get(&go.get_id()).unwrap();
        let mut entry = self.world.entry(*entity).unwrap();
        entry.add_component(component);
    }

    pub fn add_object(&mut self, go: &mut GameObject) {
        let go_entry = self.world.push(());
        self.game_objects.insert(go.get_id(), go_entry);
    }

    pub fn execute_frame_steps
        (&mut self, keyboard_state: &KeyboardState, mouse_state: &MouseState, window_event: &WindowEvent) {
        let mut resources = Resources::default();
        // TODO this is unsatisfactory, because the lifetime of WindowEvent will clearly
        // last until the last call of the execution and will never be used after that
        // but it still asks a static lifetime, probably need help from Lucas or Vincent
        // on that one
        // TODO there is redundancy here as we pass the internally maintained inputs as well as the
        // whole of window_event, in the future, we will want to have a single structure which
        // contains the nicely presented inputs for the users as well as the whole of window_event
        // for those that need events for which I haven't implemented an interface
        resources.insert(window_event.clone());
        resources.insert(mouse_state.clone());
        resources.insert(keyboard_state.clone());
        for step in self.frame_steps.iter_mut() {
            match step {
                Systems(executor) => executor.execute(&mut self.world, &mut resources),
                _ => (),
            };
        };
    }

    pub fn add_system<T: ParallelRunnable + 'static>(&mut self, system: T) {
        let new_executor = Executor::new(vec![Box::new(system)]);
        self.frame_steps.push(Systems(new_executor));
    }



    // TODO find out if it is possible to take &mut self as argument instead of getting everything
    // through by hand
    pub fn load_graphic_component(
        gc: &GraphicComponent,
        display_clone: &Display<WindowSurface>,
        models: &mut HashMap<String, ObjectModel>,
        programs: &mut HashMap<(String, String), Program>,
        textures: &mut HashMap<String, Texture2d>,
    ) {
        // loads and adds the model corresponding to the gc of the go if said model hasn't already
        // been loaded, when improving performance, will need to check that
        if let Some(geometry) = &gc.model_path {
            models
                .entry(geometry.to_string())
                .or_insert_with(|| load_model(Path::new(&geometry), display_clone).unwrap());
        } else {
            println!("Warning: object has graphic component but no model");
        }

        // same thing as models but with shaders
        if let (Some(vertex_shader), Some(fragment_shader)) =
            (&gc.vertex_shader_path, &gc.fragment_shader_path)
        {
            let program_key = (vertex_shader.clone(), fragment_shader.clone());
            programs.entry(program_key).or_insert_with(|| {
                load_shaders(vertex_shader, fragment_shader, display_clone).unwrap()
            });
        } else {
            println!("Warning: object has graphic cock but no shaders")
        }

        // same thing again but with textures
        if let Some(texture_path) = &gc.texture_path {
            // copied from the tutorial
            let image = ImageReader::open(texture_path).unwrap().decode().unwrap().to_rgba8();
            let image_dimensions = image.dimensions();
            let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
            let texture = Texture2d::new(display_clone, image).unwrap();
            textures.insert(texture_path.to_string(), texture); 
        } else {
            println!("Warning: object has graphic cock but no textures")
        }
    }

    pub fn load_all_gc(&mut self, display_ref: &Display<WindowSurface>) {
        let mut gc_query = <&GraphicComponent>::query();
        gc_query.iter(&self.world).for_each(|gc| {
            Self::load_graphic_component(gc, display_ref, &mut self.models, &mut self.programs, &mut self.textures)
        });
    }

    // user accessible function that will allow them to set the camera of a scene so that it may be
    // drawn
    pub fn to_draw(&mut self, camera: Camera) {
        self.render_cam = Some(camera);
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    // will draw all active objects with active graphic components
    // we assume that all objects have at most one graphic component
    pub fn draw_scene(&mut self, mut target: Frame, camera: &Camera) {
        //println!("drawing scene");
        // refreshes the background colour
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        // TODO set up lights properly (can wait)
        let light = [0.0, 0.0, 0.0f32];

        // parameters, not 100% what they do
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // computes the camera's veiw matrix
        let view = camera.view_matrix();

        // computes the perspective matrix
        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.1 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
                [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
            ]
        };

        // we need the game object in order to draw the object because that is where its
        // transform is stored
        let mut draw_component = |gc: &GraphicComponent, obj_transform: &Transform| {
            //let go_entry = self.world.entry_ref(go.entity).unwrap();
            //let gc = go_entry.get_component::<GraphicComponent>().unwrap();
            if gc.is_active() && gc.can_be_drawn() {
                // TODO this is very unsatisfactory, need to find some way to not have to use clones
                let program_key = &(
                    gc.vertex_shader_path.clone().unwrap(),
                    gc.fragment_shader_path.clone().unwrap(),
                );
                let object_geometry = self.models.get(gc.model_path.as_ref().unwrap()).unwrap();
                let program = self.programs.get(program_key).unwrap();
                let texture = self.textures.get(gc.texture_path.as_ref().unwrap()).unwrap();

                let vertices = &object_geometry.vertices;
                let indices = &object_geometry.indices;

                let matrix = obj_transform.uniform_matrix();

                //println!("drawing object");
                target
                    .draw(
                        vertices,
                        indices,
                        &program,
                        &uniform! {matrix: matrix, view: view, u_light: light, perspective: perspective, tex: texture},
                        &params,
                    )
                    .unwrap();
            }
        };

        let gc_query = <&GraphicComponent>::query();

        for (_, entity) in &self.game_objects {
            let go_entry = self.world.entry(*entity).unwrap();
            let gc_res = go_entry.get_component::<GraphicComponent>();
            let transform_res = go_entry.get_component::<Transform>();
            if let (Ok(gc), Ok(transform)) = (gc_res, transform_res) {
                draw_component(gc, transform);
            }
        }

        target.finish().unwrap();
    }
}
