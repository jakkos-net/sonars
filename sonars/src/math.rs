use self::bjorklund::bjorklund;

pub mod bjorklund;

pub fn saturate(f: f32) -> f32 {
    f.clamp(0.0, 1.0)
}

pub fn pre_cache_maths() {
    bjorklund(1, 1, 1.0);
}

// seq([])
// pub fn seq(v: c) -> f32 {}
