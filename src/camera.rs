#![allow(dead_code)]
#![allow(unused_variables)]

use cgmath::Quaternion;
use cgmath::Vector3;

use crate::transform::rotation_to_direction;
use crate::transform::v3_normalised;
use crate::transform::Transform;

#[derive(Copy, Clone)]
pub struct Camera {
    pub transform: Transform,
    pub fov: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            transform: Transform::new(
                Vector3::new(0.0, 0.0, -5.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 1.0, 1.0),
            ),
            fov: 0.1,
        }
    }

    pub fn get_transform(self) -> Transform {
        return self.transform;
    }

    pub fn set_position(mut self, new_pos: Vector3<f32>) -> () {
        self.transform.set_position(new_pos);
    }

    pub fn get_fwd(self) -> Vector3<f32> {
        let fwd = Vector3::new(0.0, 0.0, 1.0);
        return rotation_to_direction(self.transform.get_qrot(), fwd);
    }

    pub fn get_up(self) -> Vector3<f32> {
        let up = Vector3::new(0.0, 1.0, 0.0);
        return rotation_to_direction(self.transform.get_qrot(), up);
    }

    pub fn view_matrix(self) -> [[f32; 4]; 4] {
        let fwd = Vector3::new(0.0, 0.0, 1.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let pos = self.transform.get_position();

        let cam_rot = self.transform.get_qrot();

        let fwd_r = cam_rot.conjugate() * Quaternion::from_sv(0.0, fwd) * cam_rot;
        let up_r = cam_rot.conjugate() * Quaternion::from_sv(0.0, up) * cam_rot;

        let fwd = fwd_r.v;
        let up = up_r.v;

        let f = v3_normalised(fwd);

        let s = Vector3::cross(up, f);

        let s_norm = v3_normalised(s);

        let u = Vector3::cross(f, s_norm);

        let p = [
            -pos[0] * s_norm[0] - pos[1] * s_norm[1] - pos[2] * s_norm[2],
            -pos[0] *      u[0] - pos[1] *      u[1] - pos[2] *      u[2],
            -pos[0] *      f[0] - pos[1] *      f[1] - pos[2] *      f[2],
        ];

        let res = [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ];

        return res;
    }
}
