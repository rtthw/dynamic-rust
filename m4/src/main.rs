


use common::dreg::prelude::*;
use rhai::{Engine, EvalAltResult};



pub fn main() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();
    let script = "print(\"Hello, Rhai!\");";

    engine.run(script)?;

    Ok(())
}

struct M4Program {
    should_exit: bool,
}

impl Program for M4Program {
    fn update(&mut self, frame: Frame) {
        let area = frame.area;
        frame.buffer.set_stringn(area.x, area.y, "TODO: M4", area.width as usize, Style::new());
    }

    fn on_input(&mut self, input: Input) {
        match input {
            Input::KeyDown(Scancode::Q) => {
                self.should_exit = true;
            }
            _ => {}
        }
    }

    fn on_platform_request(&mut self, _request: &str) -> Option<&str> {
        None
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }
}
