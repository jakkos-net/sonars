use std::f32::consts::TAU;

use anyhow::bail;
use rhai::{Engine, Func};

use crate::sound::SoundFn;

// Munk: a "meditative" ;), musical, funKtional language.

// struct Munk {}

// impl Munk {
//     fn from_source() {

//     }

//     fn to_fn(){

//     }

//     fn to_rhai(){

//     }
// }

// temp, just compile rhai language
pub fn compile(src: &str) -> anyhow::Result<SoundFn> {
    let mut engine = Engine::new();
    engine.set_optimization_level(rhai::OptimizationLevel::Full);
    register_fns(&mut engine);

    let script = format!(
        "
            fn main (t) {{
                {src}
            }}
        "
    );

    let inner_fn = Func::<(f32,), f32>::create_from_script(engine, &script, "main")?;

    if let Err(e) = inner_fn(1.0) {
        bail!("source code does not evalulate to a number!\n{e}")
    }

    let sound_fn = Box::new(move |f| (inner_fn)(f).unwrap().clamp(-1.0, 1.0));

    Ok(sound_fn)
}

fn register_fns(engine: &mut Engine) {
    engine.register_fn("sin", |f: f32| (f * TAU).sin());
}
