//! Widgets



use dreg::prelude::*;



pub struct Block;

impl Block {
    pub fn render(self, area: Rect, buf: &mut Buffer) {
        for y in area.top()..area.bottom() {
            buf.get_mut(area.x, y).set_char('│');
            buf.get_mut(area.right().saturating_sub(1), y).set_char('│');
        }
        for x in area.left()..area.right() {
            buf.get_mut(x, area.y).set_char('─');
            buf.get_mut(x, area.bottom().saturating_sub(1)).set_char('─');
        }
        buf.get_mut(area.x, area.y).set_char('┌');
        buf.get_mut(area.right().saturating_sub(1), area.y).set_char('┐');
        buf.get_mut(area.right().saturating_sub(1), area.bottom().saturating_sub(1)).set_char('┘');
        buf.get_mut(area.x, area.bottom().saturating_sub(1)).set_char('└');
    }
}
