use crate::camera::Camera;
use crate::fps_camera_controller::update_camera;
use crate::input::KeyboardState;
use crate::scene::Scene;

use std::thread::sleep;

use winit::event::KeyEvent;
//use winit::event::WindowEvent;
use winit::event::WindowEvent::KeyboardInput;
use winit::event::WindowEvent::RedrawRequested;
use winit::event::Event::AboutToWait;
//use winit::event_loop::ControlFlow::WaitUntil;
//use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::keyboard::PhysicalKey::Code;
//use winit::keyboard::Key;

//use glutin::event::VirtualKeyCode;
use winit::event_loop::EventLoopBuilder;
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
        let event_loop = EventLoopBuilder::new().build().expect("event loop building");
        let (window, display) = SimpleWindowBuilder::new().build(&event_loop);

        let mut keyboard_state = KeyboardState::new();
        let mut main_camera = Camera::new();

        self.scenes[0].load_all_gc(&display);

        let _game_loop = event_loop.run(move |ev, window_target| {
            //println!("beginning of game loop");
            let begin_frame_time = std::time::Instant::now();
            let next_frame_time = begin_frame_time + std::time::Duration::from_nanos(16_666_667);

            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: Code(KeyCode::KeyX),
                                ..
                            },
                        ..
                    } => {
                        println!("pressing x");
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

                        self.scenes[0].draw_scene(target, &main_camera);

                        if std::time::Instant::now() > next_frame_time {
                            println!("Warning: needed more time for this frame");
                        }
                    },
                    _ => (),
                },
                AboutToWait => {
                    window.request_redraw();
                    sleep(next_frame_time - std::time::Instant::now());
                }
                _ => (),//println!("event: {:?}", ev),
            };
        });
        println!("end of game loop");
    }
}
