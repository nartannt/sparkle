#![allow(dead_code)]
#![allow(unused_variables)]

use crate::component::ComponentTrait;
use crate::component::ComponentType;

use cgmath::Matrix3;
use cgmath::Matrix4;
use cgmath::Quaternion;
use cgmath::Vector3;
use cgmath::Zero;
use libm::asinf;
use libm::atan2f;
use libm::cosf;
use libm::sinf;
use num::Float;

// TODO genrealise functions beyond f32 (not urgent, not important, would be nice tho)

// norm of a vector2
pub fn v3_norm<S: Float>(vec: Vector3<S>) -> S {
    return (vec.x.powi(2) + vec.y.powi(2) + vec.z.powi(2)).sqrt();
}

// normalised vector2
pub fn v3_normalised<S: Float>(vec: Vector3<S>) -> Vector3<S> {
    let norm = v3_norm(vec);
    let res = vec.map(|x| x / norm);

    if norm.is_zero() {
        return vec;
    } else {
        return res;
    }
}

// conversion between quaternions and euler angles
pub fn euler_to_quaternion(euler_rot: Vector3<f32>) -> Quaternion<f32> {
    let cx = cosf(euler_rot.x / 2.0);
    let sx = sinf(euler_rot.x / 2.0);
    let cy = cosf(euler_rot.y / 2.0);
    let sy = sinf(euler_rot.y / 2.0);
    let cz = cosf(euler_rot.z / 2.0);
    let sz = sinf(euler_rot.z / 2.0);

    let qw = cx * cy * cz + sx * sy * sz;
    let qx = sx * cy * cz - cx * sy * sz;
    let qy = cx * sy * cz + sx * cy * sz;
    let qz = cx * cy * sz - sx * sy * cz;

    let new_quat = Quaternion::new(qw, qx, qy, qz);

    return quaternion_normalised(new_quat);
}

pub fn quaternion_to_euler(q: Quaternion<f32>) -> Vector3<f32> {
    let x_rot = atan2f(
        2.0 * (q.s * q.v.x + q.v.y * q.v.z),
        1.0 - 2.0 * (q.v.x * q.v.x + q.v.y * q.v.y),
    );
    // the ifs are necessary for some edge cases (gimball lock)
    let y_temp = 2.0 * (q.s * q.v.y - q.v.z * q.v.x);
    let y_rot;
    if y_temp > 1.0 {
        y_rot = asinf(1.0);
    } else if y_temp < -1.0 {
        y_rot = asinf(-1.0);
    } else {
        y_rot = asinf(y_temp);
    }
    let z_rot = atan2f(
        2.0 * (q.s * q.v.z + q.v.x * q.v.y),
        1.0 - 2.0 * (q.v.y * q.v.y + q.v.z * q.v.z),
    );

    let res = Vector3::new(x_rot, y_rot, z_rot);
    //assert!(euler_to_quaternion(res) == q);
    return res;
}

pub fn quaternion_normalised(quat: Quaternion<f32>) -> Quaternion<f32> {
    let norm = num::Float::sqrt(
        quat.s * quat.s + quat.v.x * quat.v.x + quat.v.y * quat.v.y + quat.v.z * quat.v.z,
    );
    if norm.is_zero() {
        println!("hmm");
        return quat;
    } else {
        // should be allowed to do this
        if quat.s < 0.0 {
            return -quat / norm;
        } else {
            return quat / norm;
        }
    }
}

// get the vector rotated by rot
pub fn rotation_to_direction(rot: Quaternion<f32>, initial_dir: Vector3<f32>) -> Vector3<f32> {
    let quat_dir = quaternion_normalised(Quaternion::from_sv(0.0, initial_dir));
    let new_quat = rot * quat_dir * rot.conjugate();

    return v3_normalised(new_quat.v);
}

// the information of how an object is in space
// should we be using a generic num::Float type instead?
// probably, but that is something that can be refactored later
#[derive(Copy, Clone)]
pub struct Transform {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    rotation_quat: Quaternion<f32>,
    size: Vector3<f32>,
}

impl Default for Transform {
    fn default() -> Transform {
        Transform::new(
            // position
            Vector3::new(0.0, 0.0, 0.0f32),
            // rotation
            Vector3::new(0.0, 0.0, 0.0f32),
            // scale
            Vector3::new(1.0, 1.0, 1.0f32),
        )
    }
}

impl Transform {
    pub fn new(pos: Vector3<f32>, rot: Vector3<f32>, size: Vector3<f32>) -> Transform {
        let res = Transform {
            position: pos,
            rotation: rot,
            rotation_quat: euler_to_quaternion(rot),
            size,
        };
        return res;
    }

    pub fn get_position(&self) -> Vector3<f32> {
        return self.position;
    }

    pub fn get_rotation(&self) -> Vector3<f32> {
        return self.rotation;
    }

    pub fn get_qrot(&self) -> Quaternion<f32> {
        return self.rotation_quat;
    }

    // rotates the object relative to its x, y and z axis
    // the computations actually correspond to that of a global rotation but due to some inversion
    // when calculating the view matrix, it results in a local rotation
    pub fn rotate_by_local(&mut self, rot_delta: Vector3<f32>) -> () {
        let delta_rot_quat = euler_to_quaternion(rot_delta);
        self.rotation_quat = quaternion_normalised(delta_rot_quat * self.rotation_quat);
        self.rotation = quaternion_to_euler(self.rotation_quat);
    }

    // rotates the object along the world x, y and z axes
    // same thing, computes local rotation but the result is a world rotation
    pub fn rotate_by_world(&mut self, rot_delta: Vector3<f32>) -> () {
        // multiply by quaternion of rotation around local_x by rot_delta.x (same for the rest)
        let (local_x, local_y, local_z) = self.local_axes();
        let x_rot = Quaternion::from_sv(cosf(rot_delta.x / 2.0), sinf(rot_delta.x / 2.0) * local_x);
        let y_rot = Quaternion::from_sv(cosf(rot_delta.y / 2.0), sinf(rot_delta.y / 2.0) * local_y);
        let z_rot = Quaternion::from_sv(cosf(rot_delta.z / 2.0), sinf(rot_delta.z / 2.0) * local_z);
        // quaternion multiplication is not commutative, however, the order shouldn't matter in
        // this case, (can be shown by a quick calculation)
        let total_rot_quat = quaternion_normalised(z_rot * y_rot * x_rot);

        self.rotation_quat = quaternion_normalised(total_rot_quat * self.rotation_quat);
        self.rotation = quaternion_to_euler(self.rotation_quat);
    }

    // returns a tuple with the local x, y and z axes
    pub fn local_axes(&self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        let world_x = Vector3::new(1.0, 0.0, 0.0);
        let local_x = rotation_to_direction(self.rotation_quat, world_x);

        let world_y = Vector3::new(0.0, 1.0, 0.0);
        let local_y = rotation_to_direction(self.rotation_quat, world_y);

        let world_z = Vector3::new(0.0, 0.0, 1.0);
        let local_z = rotation_to_direction(self.rotation_quat, world_z);

        return (local_x, local_y, local_z);
    }

    pub fn set_position(&mut self, new_pos: Vector3<f32>) -> () {
        self.position = new_pos;
    }

    pub fn set_rotation(&mut self, new_rot: Vector3<f32>) -> () {
        self.rotation_quat = euler_to_quaternion(new_rot);
        self.rotation = new_rot;
    }

    pub fn uniform_matrix(&self) -> [[f32; 4]; 4] {
        // translation
        let pos = self.position;
        let trans_matrix = Matrix4::from([
            [1.0, 0.0, 0.0, pos.x],
            [0.0, 1.0, 0.0, pos.y],
            [0.0, 0.0, 1.0, pos.z],
            [0.0, 0.0, 0.0, 1.0f32],
        ]);

        // scaling
        let scale = self.size;
        let scale_matrix = Matrix4::from([
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 1.0, 1.0f32],
        ]);

        // rotation
        let rotation = self.rotation_quat;
        let rot_matrix = Matrix3::from(rotation);

        let rot_matrix_4 = Matrix4::from([
            rot_matrix.x.extend(0.0).into(),
            rot_matrix.y.extend(0.0).into(),
            rot_matrix.z.extend(0.0).into(),
            [0.0, 0.0, 0.0, 1.0f32],
        ]);

        return (trans_matrix * scale_matrix * rot_matrix_4).into();
    }

    pub fn print_transform(self) -> () {
        let euler_rot = quaternion_to_euler(self.rotation_quat) * (360.0 / (2.0 * 3.141592));
        println!(
            "rotation - x={}, y={}, z={}",
            euler_rot.x, euler_rot.y, euler_rot.z
        );
        println!(
            "position - x={}, y={}, z={}",
            self.position.x, self.position.y, self.position.z
        );
        println!("");
    }
}

impl ComponentTrait for Transform {
    fn is_active(&self) -> bool {
        return true;
    }

    fn set_active(&mut self, _activation: bool) {}

    fn component_type(&self) -> ComponentType {
        ComponentType::Transform
    }
}
