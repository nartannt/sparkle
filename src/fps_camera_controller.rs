#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::camera::Camera;
use crate::input::KeyboardState;
use crate::transform::rotation_to_direction;
use cgmath::Vector3;
use glium::winit::keyboard::KeyCode::*;
use glium::winit::keyboard::SmolStr;

// Once again could use more generic types, can't be bothered for now, might never be
enum CamInstr {
    Skip(),
    PrintTransform(),
    Move(Vector3<f32>),
    Rotate(Vector3<f32>),
}

pub fn update_camera(keyboard_state: &KeyboardState, camera: &mut Camera) -> () {
    let instructions = get_camera_instr(keyboard_state, camera);
    for instruction in instructions {
        execute_camera_instr(instruction, camera);
    }
}

fn execute_camera_instr(instructions: CamInstr, camera: &mut Camera) -> () {
    for instr in [instructions] {
        match instr {
            CamInstr::Skip() => {}

            CamInstr::Move(delta_pos) => {
                let pos = camera.transform.get_position();
                camera.transform.set_position(pos + delta_pos);
            }

            CamInstr::Rotate(delta_rot) => {
                // we want to rotate around the world y axis when we look "side to side"
                // but when rotating "up" or "down" we want to rotate around the local x axis
                // ie. the camera right/left (same thing for the z axis)
                camera
                    .transform
                    .rotate_by_world(Vector3::new(0.0, delta_rot.y, 0.0));
                camera
                    .transform
                    .rotate_by_local(Vector3::new(delta_rot.x, 0.0, delta_rot.z));
            }

            CamInstr::PrintTransform() => {
                let cam_fwd = camera.get_fwd();
                print!(
                    "cam fwd: x={}, y={}, z={}\n",
                    cam_fwd.x, cam_fwd.y, cam_fwd.z
                );
                let cam_up = camera.get_up();
                print!("cam up: x={}, y={}, z={}\n", cam_up.x, cam_up.y, cam_up.z);
                camera.transform.print_transform();
            }
        }
    }
}

fn get_camera_instr(keyboard_state: &KeyboardState, camera: &Camera) -> Vec<CamInstr> {
    let mspeed = 0.5;
    let rspeed = (3.1415 / 180.0) * 2.0;
    let pos = camera.transform.get_position();
    let fwd = Vector3::new(0.0, 0.0, 1.0);
    let mut camera_instructions: Vec<CamInstr> = Vec::new();
    //let mut all_instructions = [];
    // this implementation is satisfactory for tests, but not for actual use
    // TODO need to use mouse movement to rotate camera
    // TODO move relative to the camera fwd for (same for strafing left/right)
    //println!("event: {:?}", event);
    if keyboard_state.is_pressed(ArrowUp) {
        let delta_rot = Vector3::new(rspeed, 0.0, 0.0);
        camera_instructions.push(CamInstr::Rotate(delta_rot));
    }
    if keyboard_state.is_pressed(ArrowDown) {
        let delta_rot = Vector3::new(-rspeed, 0.0, 0.0);
        camera_instructions.push(CamInstr::Rotate(delta_rot));
    }
    if keyboard_state.is_pressed(ArrowLeft) {
        let delta_rot = Vector3::new(0.0, rspeed, 0.0);
        camera_instructions.push(CamInstr::Rotate(delta_rot));
    }
    if keyboard_state.is_pressed(ArrowRight) {
        let delta_rot = Vector3::new(0.0, -rspeed, 0.0);
        camera_instructions.push(CamInstr::Rotate(delta_rot));
    }
    if keyboard_state.is_pressed(KeyD) {
        let delta_pos = Vector3::new(mspeed, 0.0, 0.0);
        camera_instructions.push(CamInstr::Move(delta_pos));
    }
    if keyboard_state.is_pressed(KeyQ) {
        let delta_pos = Vector3::new(-mspeed, 0.0, 0.0);
        camera_instructions.push(CamInstr::Move(delta_pos));
    }
    if keyboard_state.is_pressed(KeyZ) {
        let delta_pos = Vector3::new(0.0, mspeed, 0.0);
        camera_instructions.push(CamInstr::Move(delta_pos));
    }
    if keyboard_state.is_pressed(KeyS) {
        let delta_pos = Vector3::new(0.0, -mspeed, 0.0);
        camera_instructions.push(CamInstr::Move(delta_pos));
    }
    if keyboard_state.is_pressed(KeyE) {
        let delta_pos = Vector3::new(0.0, 0.0, mspeed);
        camera_instructions.push(CamInstr::Move(delta_pos));
    }
    if keyboard_state.is_pressed(KeyR) {
        let delta_pos = Vector3::new(0.0, 0.0, -mspeed);
        camera_instructions.push(CamInstr::Move(delta_pos));
    }
    if keyboard_state.is_pressed(KeyP) {
        camera_instructions.push(CamInstr::PrintTransform());
    }
    return camera_instructions;
}
