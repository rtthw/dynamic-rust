//! Zones



use dreg::prelude::*;



static mut ZONES: u32 = 0;

// SAFETY: This must never be called concurrently.
pub fn make_zone() -> Zone {
    unsafe {
        let id = ZONES;
        ZONES += 1;
        Zone {
            id
        }
    }
}

pub struct ZoneTree {
    root: ZoneNode,
    current: u32,
}

impl ZoneTree {
    pub fn new() -> Self {
        let root_zone = make_zone();
        let current = root_zone.id;
        
        Self {
            root: ZoneNode::Leaf(root_zone),
            current,
        }
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn root_node_mut(&mut self) -> &mut ZoneNode {
        &mut self.root
    }

    pub fn current_zone_mut(&mut self) -> Option<&mut Zone> {
        self.root.zone_mut(self.current)
    }

    pub fn current_node_mut(&mut self) -> Option<&mut ZoneNode> {
        self.root.node_mut(self.current)
    }

    pub fn current_parent_node(&self) -> (&ZoneNode, bool) {
        if let ZoneNode::Leaf(_zone) = &self.root {
            return (&self.root, false);
        }
        
        fn iter_for_parent<'a>(
            parent: &mut Option<&'a ZoneNode>,
            child_id: u32,
            node: &'a ZoneNode,
        ) -> bool {
            match node {
                ZoneNode::Leaf(zone) => zone.id == child_id,
                n => {
                    let _old = parent.replace(n);
                    let ZoneNode::Branch(_, children) = n else { return false; };
                    for child in children {
                        if iter_for_parent(parent, child_id, child) {
                            return true;
                        }
                    }
                    false
                }
            }
        }

        let mut parent = None;

        let found_parent = iter_for_parent(&mut parent, self.current, &self.root);

        (parent.unwrap_or(&self.root), found_parent)
    }
}

impl ZoneTree {
    pub fn split_current(&mut self, branch_kind: ZoneBranch) {
        if let Some(node) = self.current_node_mut() {
            let owned_node = node.clone();
            let new_node = ZoneNode::Leaf(make_zone());
            // TODO: If ZoneNode stops being clonable, then work some magic here.
            let _old = std::mem::replace(node, ZoneNode::Branch(branch_kind, vec![owned_node, new_node]));
        }
    }
}

impl ZoneTree {
    pub fn move_current(&mut self, movement: ZoneMovement) {
        // We only want to move the current node index if it's possible to do so.
        let (ZoneNode::Branch(kind, leaves), true) = self.current_parent_node() else {
            return;
        };
        match movement {
            ZoneMovement::Left => {
                if matches!(kind, ZoneBranch::Horizontal) {
                    let mut last_topmost: Option<&Zone> = None;
                    'find: for leaf in leaves.iter() {
                        let Some(current) = leaf.topmost_zone() else { continue; };
                        if current.id == self.current {
                            if let Some(zone) = last_topmost {
                                self.current = zone.id;
                                return;
                            } else {
                                break 'find;
                            }
                        }
                        last_topmost = Some(current);
                    }
                    // The leftmost zone was either never found, or invalid. Set to last topmost.
                    for leaf in leaves.iter().rev() {
                        let Some(current) = leaf.topmost_zone() else { continue; };
                        self.current = current.id;
                        return;
                    }
                }
            }
            ZoneMovement::Right => {}
            ZoneMovement::Up => {}
            ZoneMovement::Down => {}
        }
    }
}

#[derive(Clone)]
pub enum ZoneNode {
    Branch(ZoneBranch, Vec<ZoneNode>),
    Leaf(Zone),
}

impl ZoneNode {
    pub fn render_with_cb(
        &mut self, 
        cb: &mut impl FnMut(&mut Zone, bool, Rect, &mut Buffer),
        current_id: u32,
        area: Rect,
        buf: &mut Buffer,
    ) {
        match self {
            ZoneNode::Branch(ZoneBranch::Horizontal, leaves) => {
                let main_width = area.width / leaves.len() as u16;
                let i_len = leaves.len() - 1;
                let mut real_area = area;
                for (index, leaf) in leaves.iter_mut().enumerate() {
                    let (leaf_area, area) = if index == i_len {
                        (real_area, Rect::ZERO)
                    } else {
                        real_area.hsplit_len(main_width)
                    };
                    real_area = area;
                    leaf.render_with_cb(cb, current_id, leaf_area, buf);
                }
            }
            ZoneNode::Branch(ZoneBranch::Vertical, leaves) => {
                let main_height = area.height / leaves.len() as u16;
                let i_len = leaves.len() - 1;
                let mut real_area = area;
                for (index, leaf) in leaves.iter_mut().enumerate() {
                    let (leaf_area, area) = if index == i_len {
                        (real_area, Rect::ZERO)
                    } else {
                        real_area.vsplit_len(main_height)
                    };
                    real_area = area;
                    leaf.render_with_cb(cb, current_id, leaf_area, buf);
                }
            }
            ZoneNode::Leaf(zone) => {
                cb(zone, zone.id == current_id, area, buf);
            }
        }
    }

    pub fn zone_mut(&mut self, id: u32) -> Option<&mut Zone> {
        match self {
            ZoneNode::Branch(_kind, leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.zone_mut(id) {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => {
                if zone.id == id {
                    Some(zone)
                } else {
                    None
                }
            }
        }
    }

    pub fn node_mut(&mut self, id: u32) -> Option<&mut ZoneNode> {
        match self {
            ZoneNode::Branch(_kind, leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.node_mut(id) {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => {
                if zone.id == id {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    pub fn topmost_zone(&self) -> Option<&Zone> {
        match self {
            ZoneNode::Branch(_kind, leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.topmost_zone() {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => Some(zone),
        }
    }
    
    pub fn topmost_zone_mut(&mut self) -> Option<&mut Zone> {
        match self {
            ZoneNode::Branch(_kind, leaves) => {
                for leaf in leaves {
                    if let Some(zone) = leaf.topmost_zone_mut() {
                        return Some(zone);
                    }
                }
                None
            }
            ZoneNode::Leaf(zone) => Some(zone),
        }
    }

    pub fn is_branch(&self) -> bool {
        matches!(self, ZoneNode::Branch(_, _))
    }

    /// Whether this node is a leaf and has the given ID, or is a branch and has the given ID in its
    /// hierarchy somewhere.
    pub fn has_id(&self, needle_id: u32) -> bool {
        match self {
            ZoneNode::Branch(_, leaves) => {
                leaves.iter().any(|l| l.has_id(needle_id))
            }
            ZoneNode::Leaf(Zone { id }) => id == &needle_id,
        }
    }
}

/// A zone is a portion of the screen.
/// See [make_zone] for details on how to create zones.
#[derive(Clone)]
pub struct Zone {
    id: u32,
}

impl Zone {
    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Clone, Copy)]
pub enum ZoneBranch {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy)]
pub enum ZoneMovement {
    Left,
    Right,
    Up,
    Down,
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn zone_splitting_works() {
        let mut tree = ZoneTree::new();
        assert!(tree.current_zone_mut().is_some_and(|z| z.id() == 0));
        tree.split_current(ZoneBranch::Horizontal);
        assert!(tree.root_node_mut().is_branch());
    }
}
