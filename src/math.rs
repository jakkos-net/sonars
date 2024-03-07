use crate::{euc, sound::Float};

pub mod bjorklund;

const TAU: Float = std::f64::consts::TAU as Float;

pub fn cache_maths() {
    euc!(1, 1)(1.0);
}

pub fn sat(f: Float) -> Float {
    f.clamp(0.0, 1.0)
}

pub fn clip(f: Float) -> Float {
    f.clamp(-1.0, 1.0)
}

pub fn abs(f: Float) -> Float {
    f.abs()
}

#[macro_export]
macro_rules! avg {
    ($($e:expr),*) => {
        |t| {
            use $crate::math::Callable;
            let v: Vec<Box<dyn Callable>> = vec![$(Box::new($e)),*];
            let sum: TimeInType = v.iter().map(|f| f.call(t)).sum();
            sum/(v.len() as TimeInType)
        }
    };
}

pub fn sin(f: Float) -> Float {
    (f * TAU).sin()
}

pub fn cos(f: Float) -> Float {
    (f * TAU).cos()
}

pub fn tan(f: Float) -> Float {
    (f * TAU).tan()
}

pub fn saw(f: Float) -> Float {
    (f % 1.0) * 2.0 - 1.0
}

pub fn tri(f: Float) -> Float {
    saw(f).abs() * 2.0 - 1.0
}

pub fn sqr(f: Float) -> Float {
    ((f % 2.0) as usize) as Float - 1.0
}

pub fn id(f: Float) -> Float {
    f
}

pub fn pow(f: Float, p: Float) -> Float {
    f.powf(p)
}

pub fn quant(f: Float, n: usize) -> Float {
    let n = n as Float;
    (((f * n) as usize) as Float) / n
}

#[macro_export]
macro_rules! seq {
    ($($e:expr),*) => {
        |t| {
            use $crate::math::Callable;
            let v: &[Box<dyn Callable>] = &[$(Box::new($e)),*];
            let t = t % (v.len() as TimeInType);
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
            let scaled_t:TimeInType = (t % 1.0) * (v.len()-1) as TimeInType;
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
        let k = $k as TimeInType;
        let mut up = 0.0;
        let mut down = 0.0;
        move |t: TimeInType| {
            let mut acc = f(1.0, t);
            for _ in 0..n {
                up += k;
                down -= k;
                acc += f(up, t);
                acc += f(down, t);
            }

            acc / ((n * 2 + 1) as TimeInType)
        }
    }};
}
pub trait Callable {
    fn call(&self, t: Float) -> Float;
}

impl<T> Callable for T
where
    T: Fn(Float) -> Float,
{
    fn call(&self, t: Float) -> Float {
        (self)(t)
    }
}

impl Callable for Float {
    fn call(&self, _t: Float) -> Float {
        *self
    }
}

impl Callable for usize {
    fn call(&self, _t: Float) -> Float {
        *self as Float
    }
}
