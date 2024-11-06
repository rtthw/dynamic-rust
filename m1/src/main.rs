//! Method 1: Configuration Files



use common::{dreg::prelude::*, zones::ZoneTree};



fn main() -> Result<()> {
    let mut tree = ZoneTree::new();
    tree.split_current(common::zones::ZoneBranch::Horizontal);
    CrosstermPlatform::new()?
        .run(M1Program {
            should_exit: false,
            tree,
        })
}

struct M1Program {
    should_exit: bool,
    tree: ZoneTree,
}

impl Program for M1Program {
    fn update(&mut self, mut frame: Frame) {
        self.tree.root_node_mut().render_with_cb(&mut |node, area, buf| {
            
        }, frame.area, &mut frame.buffer);
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
