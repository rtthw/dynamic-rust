


use rhai::{Engine, EvalAltResult};



pub fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let script = "print(\"Hello, Rhai!\");";

    engine.run(script)?;

    Ok(())
}
