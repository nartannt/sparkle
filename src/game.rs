use crate::camera::Camera;
use crate::fps_camera_controller::update_camera;
use crate::input::KeyboardState;
use crate::input::MouseState;
use crate::scene::Scene;

use std::thread::sleep;

use glium::winit::event::KeyEvent;
use glium::winit::event::WindowEvent::KeyboardInput;
use glium::winit::event::WindowEvent::RedrawRequested;
use glium::winit::event::Event::AboutToWait;
use glium::winit::keyboard::KeyCode;
use glium::winit::keyboard::PhysicalKey::Code;
//use winit::event::WindowEvent;
//use winit::event_loop::ControlFlow::WaitUntil;
//use winit::event_loop::EventLoop;
//use winit::keyboard::Key;

//use glutin::event::VirtualKeyCode;
use glium::winit::event_loop::EventLoop;
use glium::backend::glutin::SimpleWindowBuilder;
//use glium::glutin::surface::WindowSurface;

pub struct Game {
    pub scenes: Vec<Scene>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            scenes: Vec::new(),
        }
    }

    pub fn add_scene(&mut self, mut scene: Scene) {
        self.scenes.push(scene);
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::builder().build().expect("event loop building");
        let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

        let mut keyboard_state = KeyboardState::new();
        let mut mouse_state = MouseState::new();
        let mut main_camera = Camera::new();

        // TODO implement handling multiple scenes
        let active_scene = &mut self.scenes[0];
        active_scene.load_all_gc(&display);

        let _game_loop = event_loop.run(move |ev, window_target| {
            //println!("beginning of game loop");
            let begin_frame_time = std::time::Instant::now();
            let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

            

            match ev {
                glium::winit::event::Event::WindowEvent { event, .. } => {

                    mouse_state.process_event(&event);
                    // call user created systems
                    
                    // internal event handling
                    match event {
                    glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: Code(KeyCode::KeyX),
                                ..
                            },
                        ..
                    } => {
                        println!("exiting");
                        window_target.exit();
                        return;
                    }
                    KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: Code(key_code),
                                state,
                                ..
                            },
                        ..
                    } => {
                        keyboard_state.process_event(state, key_code);
                    }
                    RedrawRequested => {
                        update_camera(&keyboard_state, &mut main_camera);

                        let target = display.draw();

                        active_scene.draw_scene(target, &main_camera);

                        if std::time::Instant::now() > next_frame_time {
                            println!("Warning: needed more time for this frame");
                        }
                    },
                    _ => (),
                };
                active_scene.execute_frame_steps(&keyboard_state, &mouse_state, &event); 
                },
                AboutToWait => {
                    window.request_redraw();
                    sleep(next_frame_time - std::time::Instant::now());
                }
                _ => (),//println!("event: {:?}", ev),
            };
        });
    }
}
