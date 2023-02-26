# Sonars

Hiya, this is a __very work in progress__ application where you can live code sound (and hopefully music!) on the web and desktop. Something like a budget version of [Glicol](https://glicol.org/). Right now everything is a bit of a mess (don't judge my code 😄). 


[__very work in progress__ live web demo](https://jakkos.dev/sonars)



## Development environment

* This project is written in Rust. You'll need to have a recent Rust stable installation.

* There's a `justfile` provided for ease of running commands. Once you have `just` (`cargo install just`) installed:
    * `just run` for running desktop
    * `just web` for running web (deps: `cargo install wasm-server-runner`)
    * `just web_build` for creating a web build in `./docs` for GitHub Pages. (deps: `cargo install wasm-bindgen-cli`)
    `just` on Windows needs a bash-like enviornment, Git for Windows bash should work.
