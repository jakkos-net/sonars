use self::bjorklund::bjork;

pub mod bjorklund;

pub fn sat(f: f32) -> f32 {
    f.clamp(0.0, 1.0)
}

pub fn pre_cache_maths() {
    bjork(1, 1, 1.0);
}

#[macro_export]
macro_rules! avg {
    ($t:expr;$($e:expr),*) => {{
        use $crate::math::Callable;
        let v: Vec<Box<dyn Callable>> = vec![$(Box::new($e)),*];
        let sum: f32 = v.iter().map(|f| f.call($t)).sum();
        sum/(v.len() as f32)
    }};
}

#[macro_export]
macro_rules! seq {
    ($t:expr;$($e:expr),*) => {{
        use $crate::math::Callable;
        let v: Vec<Box<dyn Callable>> = vec![$(Box::new($e)),*];
        let t = $t % (v.len() as f32);
        let step = t as usize;
        let frac = t % 1.0;
        println!("{t}");
        (v[step]).call(frac)
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

// seq([])
// pub fn seq(v: c) -> f32 {}
