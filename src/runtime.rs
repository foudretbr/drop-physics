use std::cell::{Cell, RefCell};
use std::time::Instant;

type Quat = [f32; 4];

thread_local! {
    static STATE: RefCell<Vec<(u32, Quat)>> = const { RefCell::new(Vec::new()) };
    static SEEN: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
    static LAST: Cell<Option<Instant>> = const { Cell::new(None) };
    static DT: Cell<f32> = const { Cell::new(0.016) };
}

fn mul(a: Quat, b: Quat) -> Quat {
    [
        a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
        a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
        a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
}

fn normalize(q: Quat) -> Quat {
    let n = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3]).sqrt();
    if n < 1.0e-6 { [0.0, 0.0, 0.0, 1.0] } else { [q[0] / n, q[1] / n, q[2] / n, q[3] / n] }
}

fn axis_angle(axis: [f32; 3], angle: f32) -> Quat {
    let (s, c) = (angle * 0.5).sin_cos();
    [axis[0] * s, axis[1] * s, axis[2] * s, c]
}

fn rotate(q: Quat, v: [f32; 3]) -> [f32; 3] {
    let u = [q[0], q[1], q[2]];
    let t = [
        2.0 * (u[1] * v[2] - u[2] * v[1]),
        2.0 * (u[2] * v[0] - u[0] * v[2]),
        2.0 * (u[0] * v[1] - u[1] * v[0]),
    ];
    [
        v[0] + q[3] * t[0] + (u[1] * t[2] - u[2] * t[1]),
        v[1] + q[3] * t[1] + (u[2] * t[0] - u[0] * t[2]),
        v[2] + q[3] * t[2] + (u[0] * t[1] - u[1] * t[0]),
    ]
}

fn from_to(a: [f32; 3], b: [f32; 3]) -> Quat {
    let d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    if d > 0.9999 {
        return [0.0, 0.0, 0.0, 1.0];
    }
    if d < -0.9999 {
        return [1.0, 0.0, 0.0, 0.0];
    }
    normalize([
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
        1.0 + d,
    ])
}

fn nlerp(a: Quat, b: Quat, t: f32) -> Quat {
    let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3];
    let s = if dot < 0.0 { -1.0 } else { 1.0 };
    normalize([
        a[0] + (s * b[0] - a[0]) * t,
        a[1] + (s * b[1] - a[1]) * t,
        a[2] + (s * b[2] - a[2]) * t,
        a[3] + (s * b[3] - a[3]) * t,
    ])
}

fn hash(id: u32) -> u32 {
    let mut h = id.wrapping_mul(2654435761);
    h ^= h >> 15;
    h.wrapping_mul(2246822519)
}

fn axis_of(id: u32) -> [f32; 3] {
    let h = hash(id);
    let v = [
        (h & 255) as f32 / 127.5 - 1.0,
        ((h >> 8) & 255) as f32 / 127.5 - 1.0,
        ((h >> 16) & 255) as f32 / 127.5 - 1.0,
    ];
    let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if n < 0.1 { [0.0, 1.0, 0.0] } else { [v[0] / n, v[1] / n, v[2] / n] }
}

pub fn begin_frame() {
    let now = Instant::now();
    let dt = LAST.get().map_or(0.016, |t| now.duration_since(t).as_secs_f32()).clamp(0.001, 0.1);
    LAST.set(Some(now));
    DT.set(dt);
    SEEN.with_borrow_mut(|s| s.clear());
}

pub fn end_frame() {
    SEEN.with_borrow(|seen| STATE.with_borrow_mut(|st| st.retain(|(id, _)| seen.contains(id))));
}

pub fn rotation(id: u32, resting: bool, velocity: [f32; 3], flat: bool) -> [f32; 4] {
    let dt = DT.get();
    SEEN.with_borrow_mut(|s| s.push(id));
    STATE.with_borrow_mut(|st| {
        let at = match st.iter().position(|(i, _)| *i == id) {
            Some(at) => at,
            None => {
                let start = axis_angle(axis_of(id.wrapping_add(7)), (hash(id) >> 24) as f32 / 255.0 * std::f32::consts::TAU);
                st.push((id, start));
                st.len() - 1
            }
        };
        let q = st[at].1;
        let next = if resting {
            let locals: &[[f32; 3]] = if flat { &[[0.0, 0.0, 1.0]] } else { &[[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] };
            let mut best = (0.0_f32, [0.0, 1.0, 0.0]);
            for l in locals {
                let w = rotate(q, *l);
                if w[1].abs() >= best.0 {
                    best = (w[1].abs(), w);
                }
            }
            let up = if best.1[1] >= 0.0 { [0.0, 1.0, 0.0] } else { [0.0, -1.0, 0.0] };
            let target = mul(from_to(best.1, up), q);
            nlerp(q, target, 1.0 - (-10.0 * dt).exp())
        } else {
            let speed = (velocity[0] * velocity[0] + velocity[1] * velocity[1] + velocity[2] * velocity[2]).sqrt();
            let spin = 8.0 + 1.5 * speed.min(20.0);
            normalize(mul(axis_angle(axis_of(id), spin * dt), q))
        };
        st[at].1 = next;
        next
    })
}

