#![allow(dead_code)]
#![allow(unused_variables)]

use crate::game_object::GameObject;

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

use legion::storage::Component;
use legion::world::World;
use legion::world::WorldOptions;
use legion::IntoQuery;
use legion::Entity;

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

    // when we add a GameObject to a scene,
    // if it has a GraphicComponent, if its model already exists, we don't do anything
    // else, we fetch it in the files and add it to the scene models
    pub models: HashMap<String, ObjectModel>,

    // same thing as models except for shaders
    // the first String is for the vertex shaders and the second one for fragment shaders
    pub programs: HashMap<(String, String), Program>,

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
            world: World::new(WorldOptions::default()),
            render_cam: None,
        }
    }

    pub fn add_component<C: Component>(&mut self, go: &GameObject, component: C) {
        let entity = self.game_objects.get(&go.get_id()).unwrap();
        let mut entry = self.world.entry(*entity).unwrap();
        entry.add_component(component);
    }

    pub fn add_object(&mut self, go: &GameObject) {
        let go_entry = self.world.push(());
        self.game_objects.insert(go.get_id(), go_entry);
    }

    pub fn load_graphic_component(
        display_clone: &Display<WindowSurface>,
        programs: &mut HashMap<(String, String), Program>,
        models: &mut HashMap<String, ObjectModel>,
        gc: &GraphicComponent,
    ) {
        // loads and adds the model corresponding to the gc of the go if said model hasn't already
        // been loaded, when improving performance, will need to check that
        if let Some(geometry) = &gc.geometry {
            models
                .entry(geometry.to_string())
                .or_insert_with(|| load_model(Path::new(&geometry), display_clone).unwrap());
        } else {
            println!("Warning: object has graphic component but no model");
        }

        // same thing as models but with shaders
        if let (Some(vertex_shader), Some(fragment_shader)) =
            (&gc.vertex_shader, &gc.fragment_shader)
        {
            let program_key = (vertex_shader.clone(), fragment_shader.clone());
            programs.entry(program_key).or_insert_with(|| {
                load_shaders(vertex_shader, fragment_shader, display_clone).unwrap()
            });
        } else {
            println!("Warning: object has graphic cock but no shaders")
        }
    }

    pub fn load_all_gc(&mut self, display_ref: &Display<WindowSurface>) {
        let mut gc_query = <&GraphicComponent>::query();
        gc_query.iter(&self.world).for_each(|gc| {
            Self::load_graphic_component(display_ref, &mut self.programs, &mut self.models, gc)
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
        //println!("drawing scene in theory");
        // refreshes the background colour
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        // TODO set up lights properly (can wait some)
        let light = [-1.0, 0.4, 0.9f32];

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
            //print!("drawing object in theory");
            //let go_entry = self.world.entry_ref(go.entity).unwrap();
            //let gc = go_entry.get_component::<GraphicComponent>().unwrap();
            if gc.is_active() && gc.can_be_drawn() {
                // TODO this is very unsatisfactory, need to find some way to not have to use clones
                let program_key = &(
                    gc.vertex_shader.clone().unwrap(),
                    gc.fragment_shader.clone().unwrap(),
                );
                let program = self.programs.get(program_key).unwrap();
                let object_geometry = self.models.get(gc.geometry.as_ref().unwrap()).unwrap();

                let positions = &object_geometry.vertices;
                let normals = &object_geometry.normals;
                let indices = &object_geometry.indices;

                let matrix = obj_transform.uniform_matrix();

                //println!("drawing object");
                target
                    .draw(
                        (positions, normals),
                        indices,
                        &program,
                        &uniform! {matrix: matrix, view: view, u_light: light, perspective: perspective},
                        &params,
                    )
                    .unwrap();
            }
        };

        let gc_query = <&GraphicComponent>::query();

        for (_, entity) in &self.game_objects {
            //println!("here?");
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
