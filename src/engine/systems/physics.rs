use super::super::prelude::*;

static GRAVITY: Vector = Vector { x: 0.0, y: -9000.0 };

pub fn gravity(t: &mut Transform, r: &mut Rigidbody, dt: &std::time::Duration) {
    let secs = dt.as_secs_f32();
    let force = r.force;
    r.force = Vector::zero();
    r.acceleration = GRAVITY * secs * secs + force;
    r.speed += r.acceleration * secs;
    t.position += r.speed;
}
