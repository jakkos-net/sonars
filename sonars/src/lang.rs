use std::f32::consts::PI;

use thiserror::Error;

use crate::sound::SoundFn;

enum Expression{
    Sin(E,E),
    Add(E,E),
    Mul(E,E),
    Div(E,E),
    Sub(E,E),
    Mod(E, E),
    Pow(E,E),
    Num(f32),
    Invoke(String),
    Time
}
type E = Box<Expression>;

enum Statement{
    Assign(String, Vec<String>, Expression)
}

pub struct Program{
    hz: f32
}

impl Program{

    pub fn from_str(src: &str) -> Result<Self, ParsingError>
    {
        if let Ok(hz) = src.parse::<f32>(){
            Ok(Program{hz})
        } else {
            Err(ParsingError::Unknown)
        }
    }

    pub fn to_fn(&self) -> Result<SoundFn, CompilationError>
    {
        let hz = self.hz;
        Ok(Box::new(
            move |t| (t * hz * 2.0 * PI).sin()
        ))
    }
}


#[derive(Error, Debug)]
pub enum ParsingError{
    #[error("Unknown parsing error")]
    Unknown
}

#[derive(Error, Debug)]
pub enum CompilationError{
    #[error("Unknown compilation error")]
    Unknown
}