use bevy::math::{Vec2, Vec3, vec3};

pub trait Utils {
    fn to_vec3(&self, vec2: &Vec2) -> Vec3 {
        vec3(vec2.x, vec2.y, 0.0)
    }
}
