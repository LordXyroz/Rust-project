#![allow(unused_variables)]
#![allow(dead_code)]

mod engine;

fn main() {
    let mut test = engine::Engine::new();
    test.engine_loop(); 
} 