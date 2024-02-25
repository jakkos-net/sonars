use std::f32::consts::TAU;

use crate::bjork;

pub mod bjorklund;

pub fn cache_maths() {
    bjork!(1, 1)(1.0);
}

pub fn sat(f: f32) -> f32 {
    f.clamp(0.0, 1.0)
}

pub fn abs(f: f32) -> f32 {
    f.abs()
}

#[macro_export]
macro_rules! avg {
    ($($e:expr),*) => {
        |t| {
            use $crate::math::Callable;
            let v: Vec<Box<dyn Callable>> = vec![$(Box::new($e)),*];
            let sum: f32 = v.iter().map(|f| f.call(t)).sum();
            sum/(v.len() as f32)
        }
    };
}

pub fn sin(f: f32) -> f32 {
    (f * TAU).sin()
}

pub fn cos(f: f32) -> f32 {
    (f * TAU).cos()
}

pub fn tan(f: f32) -> f32 {
    (f * TAU).tan()
}

pub fn saw(f: f32) -> f32 {
    (f % 1.0) * 2.0 - 1.0
}

pub fn tri(f: f32) -> f32 {
    saw(f).abs() * 2.0 - 1.0
}

pub fn sqr(f: f32) -> f32 {
    ((f % 2.0) as usize) as f32 - 1.0
}

pub fn id(f: f32) -> f32 {
    f
}

#[macro_export]
macro_rules! seq {
    ($($e:expr),*) => {
        |t| {
            use $crate::math::Callable;
            let v: &[Box<dyn Callable>] = &[$(Box::new($e)),*];
            let t = t % (v.len() as f32);
            let step = t as usize;
            let frac = t % 1.0;
            (v[step]).call(frac)
        }
    };
}

#[macro_export]
macro_rules! env {
    ($($e:expr),*) => {{
        |t| {
            let v = [$($e),*];
            let scaled_t:f32 = (t % 1.0) * (v.len()-1) as f32;
            let step = scaled_t as usize;
            let frac = scaled_t % 1.0;
            v[step] * (1.0-frac) + v[step+1] * frac
        }
    }};
}

#[macro_export]
macro_rules! detune {
    ($n:expr, $k:expr, $f: expr) => {{
        let f = $f;
        let n = $n;
        let k = $k as f32;
        let mut up = 1.0;
        let mut down = 1.0;
        move |t: f32| {
            let mut acc = f(1.0, t);
            for _ in 0..n {
                up *= k;
                down /= k;
                acc += f(up, t);
                acc += f(down, t);
            }

            acc / ((n * 2 + 1) as f32)
        }
    }};
}
pub trait Callable {
    fn call(&self, t: f32) -> f32;
}

impl<T> Callable for T
where
    T: Fn(f32) -> f32,
{
    fn call(&self, t: f32) -> f32 {
        (self)(t)
    }
}

impl Callable for f32 {
    fn call(&self, _t: f32) -> f32 {
        *self
    }
}

impl Callable for usize {
    fn call(&self, _t: f32) -> f32 {
        *self as f32
    }
}
