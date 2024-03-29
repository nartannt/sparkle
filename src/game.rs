use crate::camera::Camera;
use crate::fps_camera_controller::update_camera;
use crate::input::KeyboardState;
use crate::scene::Scene;

use std::thread::sleep;
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use winit::event::WindowEvent::KeyboardInput;
use winit::event::WindowEvent::RedrawRequested;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey::Code;
//use winit::keyboard::Key;

//use glutin::event::VirtualKeyCode;
use glium::backend::glutin::SimpleWindowBuilder;
use glium::glutin::surface::WindowSurface;

pub struct Game {
    pub display: glium::Display<WindowSurface>,
    pub event_loop: EventLoop<()>,

    pub scenes: Vec<Scene>,
}

impl Game {
    pub fn new() -> Game {
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let simple_window_builder = SimpleWindowBuilder::new();
        //let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
        let (_, display) = simple_window_builder.build(&event_loop);

        Game {
            display,
            event_loop,
            scenes: Vec::new(),
        }
    }

    pub fn add_scene(&mut self, mut scene: Scene) {
        scene.add_display_clone(&self.display);
        self.scenes.push(scene);
    }

    pub fn run(mut self) {
        let mut keyboard_state = KeyboardState::new();
        let mut main_camera = Camera::new();

        self.scenes[0].load_all_gc();

        let _game_loop = self.event_loop.run(move |ev, _| {
            let begin_frame_time = std::time::Instant::now();
            let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        //*control_flow = glutin::event_loop::ControlFlow::Exit;
                        return;
                    }
                    KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: Code(KeyCode::KeyX),
                                ..
                            },
                        ..
                    } => {
                        //*control_flow = glutin::event_loop::ControlFlow::Exit;
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

                        let target = self.display.draw();

                        self.scenes[0].draw_scene(target, &main_camera);

                        if std::time::Instant::now() > next_frame_time {
                            println!("Warning: needed more time for this frame");
                        }

                        sleep(next_frame_time - std::time::Instant::now());

                        //*control_flow = WaitUntil(next_frame_time);
                    }
                    _ => {
                        //println!("event: {:?}", event);
                    }
                },
                _ => (),
            }
        });
    }
}
