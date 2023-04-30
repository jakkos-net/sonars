use std::f32::consts::TAU;

use anyhow::bail;
use rhai::{Engine, Func};

use crate::{math::cached_bjorklund, sound::SoundFn};

enum Expr {
    Var(String),
    Num(f32),
    List(Vec<Expr>),
}

struct Program {
    exprs: Vec<Expr>,
}

impl Program {
    fn from_source(src: &str) -> anyhow::Result<Self> {
        let _parts = src.split_ascii_whitespace();

        todo!();
    }

    fn thing<'a>(mut parts: impl Iterator<Item = &'a str>) {
        match parts.next() {
            Some(part) => match part {
                "(" => {
                    let _elems = parts.take_while(|part| *part != ")");
                    // let closing = parts.next();
                }
                ")" => (),
                other => {
                    if let Ok(num) = other.parse() {
                        Expr::Num(num);
                    } else {
                        Expr::Var(other.into());
                    }
                }
            },
            None => todo!(),
        }
    }

    pub fn to_fn(&self) -> anyhow::Result<SoundFn> {
        let ast = self.to_rhai()?;
        let mut engine = Engine::new();
        engine.set_optimization_level(rhai::OptimizationLevel::Full);
        register_fns(&mut engine);

        let inner_fn = Func::<(f32,), rhai::Array>::create_from_ast(engine, ast, "main");
        if let Err(e) = inner_fn(1.0) {
            bail!("source code does not evalulate to a number!\n{e}")
        }
        let sound_fn = Box::new(move |f| {
            let v = (inner_fn)(f).unwrap();
            [
                v[0].as_float().unwrap_or(0.0).clamp(-1.0, 1.0),
                v[1].as_float().unwrap_or(0.0).clamp(-1.0, 1.0),
            ]
        });
        Ok(sound_fn)
    }

    fn to_rhai(&self) -> anyhow::Result<rhai::AST> {
        todo!()
    }
}

// temp, just compile rhai language
pub fn compile(src: &str) -> anyhow::Result<SoundFn> {
    let mut engine = Engine::new();
    engine.set_optimization_level(rhai::OptimizationLevel::Full);
    register_fns(&mut engine);

    let script = format!(
        "
        {RHAI_FUNCTIONS}
        
            fn main (t) {{
                {src}
            }}
        "
    );

    // todo_major: work out if there is a faster way to return two floats from rhai, this way seems like it would be slow...

    let inner_fn = Func::<(f32,), rhai::Array>::create_from_script(engine, &script, "main")?;

    if let Err(e) = inner_fn(1.0) {
        bail!("source code does not evalulate to a number!\n{e}")
    }

    // todo_major deal with case where the rhai script returns a wrong-length or wrong-type array
    let sound_fn = Box::new(move |f| {
        let v = (inner_fn)(f).unwrap();
        [
            v[0].as_float().unwrap_or(0.0).clamp(-1.0, 1.0),
            v[1].as_float().unwrap_or(0.0).clamp(-1.0, 1.0),
        ]
    });

    Ok(sound_fn)
}

fn register_fns(engine: &mut Engine) {
    engine.register_fn("sin", |f: f32| (f * TAU).sin());
    engine.register_fn("abs", |f: f32| f.abs());
    engine.register_fn("pow", |n: f32, p: f32| n.powf(p));
    engine.register_fn("ln", |n: f32| n.ln());
    engine.register_fn("log", |n: f32, b: f32| n.log(b));
    engine.register_fn("euc", |steps: f32, pulses: f32, t: f32| {
        // euclidean-rhythm-like
        // given steps and pulses, return the pattern
        // select the current step based on t, where we cycle the full steps every 1s
        // if the step is "on", return the progress through the step
        // otherwise return 0.0
        // e.g. rhythm 10 steps, 3 pulses -> pattern [on, off, off, on, off, off, on, off, off ,off]
        //      if t = 0.37, we are in the fourth step (step 1 is 0 to 0.1, 2 is 0.1 to 0.2...)
        //      step 4 is on, so we return 0.7, as we are 70% through the step
        //      if t = 0.51, we are in the sixth step, which is off, so we return 0.0
        let tmod = t % 1.0;
        let index = (steps * (t % 1.0)) as usize;
        let gate = if cached_bjorklund(steps as usize, pulses as usize, index) {
            1.0_f32
        } else {
            0.0_f32
        };
        gate * ((tmod * steps) % 1.0)
    });
    engine.register_fn("frc", |f: f32| f.fract());
    engine.register_fn("flr", |f: f32| f.floor());
    engine.register_fn("cei", |f: f32| f.ceil());
    engine.register_fn("env", |a: f32, d: f32, s: f32, r: f32, t: f32| {
        // asdr-like, we don't really have the concept of hold
        // attack time, decay time, sustain level, release time
        // todo_major see if we can speed this up
        // todo_maybe do we want to add a check for a + d + r being greater than 1.0?
        // attack: over time span a, we linearly increase from 0 upto 1
        if t <= a {
            t / a
        } else {
            // decay: once we are past time a, but still before a+d, we linearly decrease from 1.0 to s over a period of d
            if t <= d + a {
                // lerp: start * (1-k) + end * k
                // where k is the proportion of time we are through the interval
                // k goes from a to a+d
                let k = (t - a) / (a + d);
                1.0 * (1.0 - k) + k * s
            } else {
                // sustain?
                let one_minus_t = 1.0 - t;
                if one_minus_t >= r {
                    s
                }
                // release?
                else {
                    one_minus_t / r * s
                }
            }
        }
    });
    engine.register_fn("lrp", |a: f32, b: f32, t: f32| (1.0 - t) * a + t * b);
    engine.register_fn("rfl", |f: f32| (-1.0 + (f % 2.0)).abs());
    engine.register_fn("saw", |f: f32| 2.0 * (f - (0.5 + f).floor()));
    engine.register_fn("sqw", |f: f32| (1.0 - (f % 2.0).floor()));
    engine.register_fn("sat", |f: f32| f.clamp(0.0, 1.0));
    engine.register_fn("clm", |f: f32, min: f32, max: f32| f.clamp(min, max));
    engine.register_fn("max", |a: f32, b: f32| a.max(b));
    engine.register_fn("min", |a: f32, b: f32| a.min(b));
}

const RHAI_FUNCTIONS: &str = include_str!("./functions.rhai");

#[cfg(test)]
mod tests {
    use rhai::{Engine, Scope};

    use super::RHAI_FUNCTIONS;

    #[test]
    fn test_rhai_function_loading() {
        let engine = Engine::new();
        let ast = engine.compile(RHAI_FUNCTIONS).unwrap();
        assert!(ast.has_functions());
        assert_eq!(
            engine
                .call_fn::<f32>(&mut Scope::new(), &ast, "test_func", ())
                .unwrap(),
            1.0
        );
    }
}
